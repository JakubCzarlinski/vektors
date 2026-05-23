use crate::cfggen::{cfg_any, cfg_availability, cfg_expr_from_dnf};
use crate::codegen::{deprecate_attr, feature_key, pretty, refpage_url};
use crate::ir::{Enum, EnumValue, EnumVariant, Registry};
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use std::collections::{BTreeMap, HashMap, HashSet};

pub fn gen_enums_rs(reg: &Registry) -> String {
    let disabled: HashSet<String> = reg
        .extensions
        .iter()
        .filter(|e| e.is_disabled())
        .map(|e| e.name.clone())
        .collect();

    let mut groups: BTreeMap<Vec<String>, TokenStream> = BTreeMap::new();

    let mut seen_features = HashSet::new();
    for e in reg.enums.values().flatten() {
        let token_stream = gen_enum(e, reg, &disabled);
        if token_stream.is_empty() {
            continue;
        }

        // Collect unique features.
        let all_feats: Vec<String> = e
            .provided_by
            .iter()
            .chain(e.variants.iter().flat_map(|v| v.provided_by.iter()))
            .filter(|&feature| !disabled.contains(feature))
            .filter(|&feature| seen_features.insert(feature.clone()))
            .cloned()
            .collect();

        groups
            .entry(feature_key(&all_feats))
            .or_default()
            .extend(token_stream);
        seen_features.clear();
    }

    let mut ts = TokenStream::new();
    ts.extend(quote! {
        //! Vulkan enum and bitmask types.
        //!
        //! Enums are `repr(transparent)` newtypes over `i32`/`i64`.
        //! Bitmasks are `repr(transparent)` newtypes over `u32`/`u64`
        //! with `|`, `&`, `^`, `!` and compound-assignment operators.
    });

    for items in groups.into_values() {
        ts.extend(items);
    }

    pretty(&ts)
}

fn gen_enum(e: &Enum, reg: &Registry, disabled: &HashSet<String>) -> TokenStream {
    // Filter variants to those that are enabled or core.
    let variants: Vec<_> = e
        .variants
        .iter()
        .filter(|v| v.provided_by.is_empty() || v.provided_by.iter().any(|f| !disabled.contains(f)))
        .cloned()
        .collect();

    // Features for the enum: its own enabled providers + enabled providers of its kept variants.
    let mut all_feats: Vec<String> = e
        .provided_by
        .iter()
        .filter(|f| !disabled.contains(*f))
        .cloned()
        .collect();
    for variant in &variants {
        for feature in &variant.provided_by {
            if !disabled.contains(feature) && !all_feats.contains(feature) {
                all_feats.push(feature.clone());
            }
        }
    }
    let value_by_name: HashMap<String, EnumValue> = e
        .variants
        .iter()
        .map(|v| (v.name.clone(), v.value.clone()))
        .collect();

    // If the enum was introduced by an extension (non-empty provided_by) but all its providers
    // and variants are disabled, skip it entirely.
    if !e.provided_by.is_empty() && all_feats.is_empty() && variants.is_empty() {
        return quote! {};
    }

    let mut availability = e.availability.clone();
    for variant in &variants {
        availability.extend(variant.availability.clone());
    }
    let cfg = cfg_availability(&availability, &all_feats, e.dep.as_ref());
    let name = format_ident!("{}", &e.name);
    let url_str = format!(" [{}]({})", &e.name, refpage_url(&e.name));
    let mut doc = quote! { #[doc = #url_str] };
    if let Some(ref comment) = e.comment {
        let comment = comment.trim();
        if !comment.is_empty() {
            doc.extend(quote! { #[doc = " "] });
            let comment = " ".to_string() + comment;
            doc.extend(quote! { #[doc = #comment] });
        }
    }
    if let Some(ref dep) = e.dep {
        doc.extend(quote! { #[doc = " "] });
        let depends_on = dep.atoms().join("`, `");
        let comment = format!(" **Availability:** depends on `{depends_on}`.");
        doc.extend(quote! { #[doc = #comment] });
    }

    let depr = deprecate_attr(&e.depr);

    if let Some(ref alias) = e.alias {
        if let Some(target) = reg.enums.get(alias).and_then(|items| items.first()) {
            let mut resolved = target.clone();
            resolved.name = e.name.clone();
            resolved.alias = None;
            resolved.comment = e.comment.clone().or(resolved.comment);
            resolved.dep = e.dep.clone();
            resolved.availability = e.availability.clone();
            resolved.depr = e.depr.clone();
            resolved.provided_by = e.provided_by.clone();
            return gen_enum(&resolved, reg, disabled);
        }
        let a = format_ident!("{}", alias);
        return quote! { #cfg pub type #name = #a; };
    }

    if e.is_bitmask {
        return gen_bitmask_type(e, cfg, name, doc.clone(), &all_feats, depr, disabled);
    }

    let inner = if e.bit_width == 64 {
        quote! {i64}
    } else {
        quote! {i32}
    };
    let variant_names = shorten_variant_names(e, &variants, false);
    let mut variant_token_stream = TokenStream::new();
    let mut display_match_arms = TokenStream::new();
    let mut seen_features: HashSet<String> = HashSet::new();
    let mut seen_rust_names: HashSet<String> = HashSet::new();

    for variant in variants {
        if !seen_features.insert(variant.name.clone()) {
            continue;
        }
        let rust_variant_name = variant_names
            .get(&variant.name)
            .map(String::as_str)
            .unwrap_or(&variant.name);
        if !seen_rust_names.insert(rust_variant_name.to_owned()) {
            continue;
        }
        let variant_name = format_ident!("{}", rust_variant_name);
        let variant_doc = variant.comment.as_deref().unwrap_or("");
        let variant_depr = deprecate_attr(&variant.depr);
        let mut variant_cfg = variant_cfg(&variant, &all_feats, disabled);

        if let Some(ref aset) = variant.api {
            if aset.vulkansc && !aset.vulkan {
                variant_cfg = quote! { #variant_cfg #[cfg(feature = "VKSC_VERSION_1_0")] };
            } else if aset.vulkan && !aset.vulkansc {
                variant_cfg = quote! { #variant_cfg #[cfg(not(feature = "VKSC_VERSION_1_0"))] };
            }
        }

        let resolved = resolve_enum_value(&variant.value, &value_by_name);
        let val = enum_val_tokens(&resolved, false);
        let variant_name_str = variant.name.clone();
        if variant_doc.is_empty() {
            variant_token_stream.extend(quote! {
                #variant_cfg #variant_depr
                pub const #variant_name: Self = Self(#val);
            });
        } else {
            variant_token_stream.extend(quote! {
                #variant_cfg #[doc = #variant_doc] #variant_depr
                pub const #variant_name: Self = Self(#val);
            });
        }
        display_match_arms.extend(quote! {
            #variant_cfg
            value if value == Self::#variant_name.0 => f.write_str(#variant_name_str),
        });
    }

    doc.extend(quote! {
        #cfg #depr
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
        pub struct #name(pub #inner);

        #cfg
        impl #name { #variant_token_stream }

        #cfg
        impl core::fmt::Display for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self.0 {
                    #display_match_arms
                    _ => write!(f, "{}({})", stringify!(#name), self.0),
                }
            }
        }
    });
    doc
}

fn gen_bitmask_type(
    e: &Enum,
    cfg: TokenStream,
    name: proc_macro2::Ident,
    doc: TokenStream,
    all_feats: &[String],
    depr: TokenStream,
    disabled: &HashSet<String>,
) -> TokenStream {
    let mut doc = doc;
    let inner = if e.bit_width == 64 {
        quote! {u64}
    } else {
        quote! {u32}
    };
    let mut bit_token_stream = TokenStream::new();
    let mut display_token_stream = TokenStream::new();
    let mut known_bits_token_stream = TokenStream::new();
    let mut seen_features: HashSet<String> = HashSet::new();
    let mut seen_rust_names: HashSet<String> = HashSet::new();
    let variant_names = shorten_variant_names(e, &e.variants, true);
    let value_by_name: HashMap<String, EnumValue> = e
        .variants
        .iter()
        .map(|v| (v.name.clone(), v.value.clone()))
        .collect();

    for variant in &e.variants {
        if !seen_features.insert(variant.name.clone()) {
            continue;
        }

        let rust_variant_name = variant_names
            .get(&variant.name)
            .map(String::as_str)
            .unwrap_or(&variant.name);
        if !seen_rust_names.insert(rust_variant_name.to_owned()) {
            continue;
        }
        let variant_name = format_ident!("{}", rust_variant_name);
        let variant_doc = variant.comment.as_deref().unwrap_or("");
        let variant_depr = deprecate_attr(&variant.depr);
        let mut variant_cfg = variant_cfg(variant, all_feats, disabled);

        if let Some(ref aset) = variant.api {
            if aset.vulkansc && !aset.vulkan {
                variant_cfg = quote! { #variant_cfg #[cfg(feature = "VKSC_VERSION_1_0")] };
            } else if aset.vulkan && !aset.vulkansc {
                variant_cfg = quote! { #variant_cfg #[cfg(not(feature = "VKSC_VERSION_1_0"))] };
            }
        }

        let resolved = resolve_enum_value(&variant.value, &value_by_name);
        let val = enum_val_tokens(&resolved, true);
        let variant_name_str = variant.name.clone();
        if variant_doc.is_empty() {
            bit_token_stream.extend(quote! {
                #variant_cfg #variant_depr
                pub const #variant_name: Self = Self(#val);
            });
        } else {
            bit_token_stream.extend(quote! {
                #variant_cfg #[doc = #variant_doc] #variant_depr
                pub const #variant_name: Self = Self(#val);
            });
        }
        display_token_stream.extend(quote! {
            #variant_cfg
            if self.intersects(Self::#variant_name) {
                if wrote {
                    f.write_str(" | ")?;
                }
                f.write_str(#variant_name_str)?;
                wrote = true;
            }
        });
        known_bits_token_stream.extend(quote! {
            #variant_cfg
            {
                bits |= Self::#variant_name.0;
            }
        });
    }

    let display_impl = if display_token_stream.is_empty() {
        quote! {
            #cfg
            impl core::fmt::Display for #name {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    if self.0 == 0 {
                        f.write_str("0")
                    } else {
                        write!(f, "0x{:x}", self.0)
                    }
                }
            }
        }
    } else {
        quote! {
            #cfg
            impl core::fmt::Display for #name {
                #[allow(unused_mut)]
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    if self.is_empty() {
                        f.write_str("0")
                    } else {
                        let mut wrote = false;
                        #display_token_stream
                        let known_bits = {
                            let mut bits = 0;
                            #known_bits_token_stream
                            bits
                        };
                        let unknown_bits = self.0 & !known_bits;
                        if unknown_bits != 0 {
                            if wrote {
                                f.write_str(" | ")?;
                            }
                            write!(f, "0x{:x}", unknown_bits)?;
                            wrote = true;
                        }
                        if wrote {
                            Ok(())
                        } else {
                            write!(f, "0x{:x}", self.0)
                        }
                    }
                }
            }
        }
    };

    doc.extend(
        quote! {
            #cfg #depr
            #[repr(transparent)]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
            pub struct #name(pub #inner);

            #cfg
            impl #name {
                pub const EMPTY: Self = Self(0);
                #bit_token_stream
                #[inline] pub const fn contains(self, o: Self) -> bool { (self.0 & o.0) == o.0 }
                #[inline] pub const fn intersects(self, o: Self) -> bool { (self.0 & o.0) != 0 }
                #[inline] pub const fn is_empty(self) -> bool { self.0 == 0 }
            }
            #cfg impl core::ops::BitOr        for #name { type Output=Self; #[inline] fn bitor   (self,r:Self)->Self{Self(self.0|r.0)} }
            #cfg impl core::ops::BitOrAssign  for #name { #[inline] fn bitor_assign   (&mut self,r:Self){self.0|=r.0} }
            #cfg impl core::ops::BitAnd       for #name { type Output=Self; #[inline] fn bitand  (self,r:Self)->Self{Self(self.0&r.0)} }
            #cfg impl core::ops::BitAndAssign for #name { #[inline] fn bitand_assign  (&mut self,r:Self){self.0&=r.0} }
            #cfg impl core::ops::BitXor       for #name { type Output=Self; #[inline] fn bitxor  (self,r:Self)->Self{Self(self.0^r.0)} }
            #cfg impl core::ops::BitXorAssign for #name { #[inline] fn bitxor_assign  (&mut self,r:Self){self.0^=r.0} }
            #cfg impl core::ops::Not          for #name { type Output=Self; #[inline] fn not(self)->Self{Self(!self.0)} }

            #cfg impl core::ops::BitOr        <#inner> for #name { type Output=Self; #[inline] fn bitor   (self,r:#inner)-> Self{Self(self.0|r)} }
            #cfg impl core::ops::BitOrAssign  <#inner> for #name { #[inline] fn bitor_assign   (&mut self,r:#inner){self.0|=r} }
            #cfg impl core::ops::BitAnd       <#inner> for #name { type Output=Self; #[inline] fn bitand  (self,r:#inner)-> Self{Self(self.0&r)} }
            #cfg impl core::ops::BitAndAssign  <#inner> for #name { #[inline] fn bitand_assign  (&mut self,r:#inner){self.0&=r} }
            #cfg impl core::ops::BitXor       <#inner> for #name { type Output=Self; #[inline] fn bitxor  (self,r:#inner)-> Self{Self(self.0^r)} }
            #cfg impl core::ops::BitXorAssign  <#inner> for #name { #[inline] fn bitxor_assign  (&mut self,r:#inner){self.0^=r} }

            #display_impl
        }
    );
    doc
}

fn shorten_variant_names(
    e: &Enum,
    variants: &[EnumVariant],
    strip_bit_suffix: bool,
) -> HashMap<String, String> {
    let variant_tokens: Vec<_> = variants
        .iter()
        .map(|variant| variant.name.split('_').collect::<Vec<_>>())
        .collect();
    let common_prefix_len = common_token_prefix_len(&variant_tokens).max(type_prefix_len(
        e,
        variant_tokens.first().map(Vec::as_slice).unwrap_or(&[]),
    ));
    let vendor_tag = vendor_tag(&e.name);
    let value_by_name: HashMap<String, EnumValue> = e
        .variants
        .iter()
        .map(|variant| (variant.name.clone(), variant.value.clone()))
        .collect();
    let mut used = HashMap::<String, (String, EnumValue)>::new();
    let mut out = HashMap::new();

    for variant in variants {
        let original = &variant.name;
        let resolved = resolve_enum_value(&variant.value, &value_by_name);
        let mut tokens: Vec<&str> = original.split('_').skip(common_prefix_len).collect();
        if tokens.is_empty() {
            tokens = original.split('_').collect();
        }

        if vendor_tag.is_some_and(|tag| tokens.last() == Some(&tag)) {
            tokens.pop();
        }
        if strip_bit_suffix && tokens.last() == Some(&"BIT") {
            tokens.pop();
        }
        if tokens.is_empty() {
            tokens = original.split('_').collect();
        }

        let mut shortened = tokens.join("_");
        if shortened
            .as_bytes()
            .first()
            .is_some_and(|byte| byte.is_ascii_digit())
        {
            shortened = format!("{}_{}", numeric_variant_prefix(e, &tokens), shortened);
        }

        if let Some((existing, existing_value)) = used.get(&shortened)
            && existing != original
            && existing_value != &resolved
        {
            out.insert(original.clone(), original.clone());
            continue;
        }

        used.insert(shortened.clone(), (original.clone(), resolved));
        out.insert(original.clone(), shortened);
    }

    out
}

fn common_token_prefix_len(variant_tokens: &[Vec<&str>]) -> usize {
    if variant_tokens.len() <= 1 {
        return 0;
    }
    let Some(first) = variant_tokens.first() else {
        return 0;
    };
    let mut len = 0;
    'prefix: for (idx, token) in first.iter().enumerate() {
        for tokens in variant_tokens.iter().skip(1) {
            if tokens.get(idx).copied() != Some(*token) {
                break 'prefix;
            }
        }
        len += 1;
    }
    if first.len() == len {
        len.saturating_sub(1)
    } else {
        len
    }
}

fn type_prefix_len(e: &Enum, variant_tokens: &[&str]) -> usize {
    let mut type_tokens = type_name_tokens(&e.name);
    if e.is_bitmask {
        if type_tokens.ends_with(&["FLAG".to_owned(), "BITS".to_owned()]) {
            type_tokens.truncate(type_tokens.len() - 2);
        } else if type_tokens.ends_with(&["FLAGS".to_owned()]) {
            type_tokens.truncate(type_tokens.len() - 1);
        }
    }
    let mut len = 0;
    for (type_token, variant_token) in type_tokens.iter().zip(variant_tokens) {
        if type_token != variant_token {
            break;
        }
        len += 1;
    }
    len
}

fn type_name_tokens(name: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut previous_was_lower_or_digit = false;
    for ch in name.chars() {
        if ch == '_' {
            if !current.is_empty() {
                tokens.push(current.to_ascii_uppercase());
                current.clear();
            }
            previous_was_lower_or_digit = false;
            continue;
        }
        if ch.is_ascii_uppercase() && previous_was_lower_or_digit && !current.is_empty() {
            tokens.push(current.to_ascii_uppercase());
            current.clear();
        }
        previous_was_lower_or_digit = ch.is_ascii_lowercase() || ch.is_ascii_digit();
        current.push(ch);
    }
    if !current.is_empty() {
        tokens.push(current.to_ascii_uppercase());
    }
    tokens
}

fn vendor_tag(type_name: &str) -> Option<&'static str> {
    const TAGS: &[&str] = &[
        "AMD", "AMDX", "ANDROID", "ARM", "EXT", "FUCHSIA", "GGP", "GOOGLE", "HUAWEI", "IMG",
        "INTEL", "KHR", "LUNARG", "MESA", "MSFT", "MVK", "NN", "NV", "NVX", "OHOS", "QCOM", "QNX",
        "SEC", "VALVE",
    ];
    TAGS.iter().copied().find(|tag| type_name.ends_with(tag))
}

fn numeric_variant_prefix(e: &Enum, tokens: &[&str]) -> &'static str {
    if e.name == "VkImageCompressionFixedRateFlagBitsEXT"
        || tokens.iter().any(|token| token.ends_with("BPC"))
    {
        "RATE"
    } else if e.is_bitmask {
        "BIT"
    } else {
        "VALUE"
    }
}

fn variant_cfg(
    variant: &EnumVariant,
    all_feats: &[String],
    disabled: &HashSet<String>,
) -> TokenStream {
    let v_feats: Vec<_> = variant
        .provided_by
        .iter()
        .filter(|f| !disabled.contains(*f))
        .cloned()
        .collect();

    if variant.dep.is_none() && variant.availability.is_empty() {
        return if v_feats.is_empty() || v_feats == all_feats {
            quote! {}
        } else {
            cfg_any(&v_feats)
        };
    }

    let mut clauses = Vec::<Vec<String>>::new();
    if variant.availability.is_empty() {
        let Some(dep) = &variant.dep else {
            return quote! {};
        };
        for provider in v_feats {
            for mut clause in dep.to_dnf_clauses() {
                if !clause.contains(&provider) {
                    clause.insert(0, provider.clone());
                }
                if clause.iter().any(|feature| disabled.contains(feature)) {
                    continue;
                }
                clause.sort();
                if !clauses.contains(&clause) {
                    clauses.push(clause);
                }
            }
        }
    } else {
        for availability in &variant.availability {
            if disabled.contains(&availability.provider) {
                continue;
            }
            let dep_clauses = availability
                .dep
                .as_ref()
                .map(|dep| dep.to_dnf_clauses())
                .unwrap_or_else(|| vec![vec![]]);
            for mut clause in dep_clauses {
                if !clause.contains(&availability.provider) {
                    clause.insert(0, availability.provider.clone());
                }
                if clause.iter().any(|feature| disabled.contains(feature)) {
                    continue;
                }
                clause.sort();
                if !clauses.contains(&clause) {
                    clauses.push(clause);
                }
            }
        }
    }

    if clauses.is_empty() {
        quote! {}
    } else {
        let expr = cfg_expr_from_dnf(&clauses);
        quote! { #[cfg(#expr)] }
    }
}

fn enum_val_tokens(val: &EnumValue, unsigned: bool) -> TokenStream {
    match val {
        EnumValue::Integer(n) => {
            if unsigned {
                let l = Literal::u64_unsuffixed(*n as u64);
                quote! {#l}
            } else {
                let l = Literal::i64_unsuffixed(*n);
                quote! {#l}
            }
        }
        EnumValue::Hex(n) => {
            let l = Literal::u64_unsuffixed(*n);
            quote! {#l}
        }
        EnumValue::BitPos(p) => {
            let p = u64::from(*p);
            quote! { 1 << #p }
        }
        EnumValue::Offset {
            extnumber,
            offset,
            negative,
        } => {
            let v = 1_000_000_000i64 + (i64::from(*extnumber) - 1) * 1000 + i64::from(*offset);
            let v = if *negative { -v } else { v };
            let l = Literal::i64_unsuffixed(v);
            quote! {#l}
        }
        EnumValue::Alias(a) => {
            let a = format_ident!("{}", a);
            quote! { Self::#a.0 }
        }
        EnumValue::Expr(s) => normalize_expr(s, unsigned).parse().unwrap_or_else(|_| {
            let l = Literal::i64_unsuffixed(0);
            quote! {#l}
        }),
    }
}

fn resolve_enum_value(value: &EnumValue, value_by_name: &HashMap<String, EnumValue>) -> EnumValue {
    fn resolve_with_seen(
        value: &EnumValue,
        value_by_name: &HashMap<String, EnumValue>,
        seen: &mut HashSet<String>,
    ) -> EnumValue {
        match value {
            EnumValue::Alias(name) => {
                if !seen.insert(name.clone()) {
                    return value.clone();
                }
                let resolved = value_by_name
                    .get(name)
                    .map(|v| resolve_with_seen(v, value_by_name, seen))
                    .unwrap_or_else(|| value.clone());
                seen.remove(name);
                resolved
            }
            _ => value.clone(),
        }
    }

    let mut seen = HashSet::new();
    resolve_with_seen(value, value_by_name, &mut seen)
}

fn normalize_expr(s: &str, unsigned: bool) -> String {
    let s = s.trim();
    if s == "(~0U)" || s == "~0U" || s == "(~0u)" || s == "~0u" {
        return if unsigned {
            "u32::MAX".into()
        } else {
            "-1i32 as i32".into()
        };
    }
    if s == "(~0ULL)" || s == "~0ULL" || s == "(~0ull)" || s == "~0ull" {
        return if unsigned {
            "u64::MAX".into()
        } else {
            "-1i64 as i64".into()
        };
    }
    if s.starts_with("(~") || s.starts_with('~') {
        return if unsigned {
            "u32::MAX".into()
        } else {
            "-1i32 as i32".into()
        };
    }
    s.to_owned()
}
