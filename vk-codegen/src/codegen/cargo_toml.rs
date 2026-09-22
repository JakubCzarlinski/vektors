use std::collections::HashSet;

use crate::ir::{ApiSet, Extension, Registry, TypedefKind};
use indexmap::IndexMap;

pub fn gen_cargo_toml(reg: &Registry, crate_version: &str) -> String {
    let mut feature_deps = reg.feature_deps();
    for ext in &reg.extensions {
        if ext.is_disabled() || ext.is_video_header() {
            continue;
        }
        let dependencies = feature_deps.entry(ext.name.clone()).or_default();
        for dependency in inferred_core_dependencies(reg, ext) {
            if !dependencies.contains(&dependency) {
                dependencies.push(dependency);
            }
        }
    }
    let transitive_deps = transitive_dependencies(&feature_deps);
    // Build the complete set of known feature names so deps can be filtered
    let known_features: HashSet<String> = reg.all_feature_names().into_iter().collect();

    let filter_deps = |deps: Vec<String>| -> Vec<String> {
        deps.into_iter()
            .filter(|d| known_features.contains(d))
            .collect()
    };
    let simplify_deps = |deps: &mut Vec<String>| {
        let original = deps.clone();
        deps.retain(|dep| {
            !original.iter().any(|other| {
                other != dep
                    && transitive_deps
                        .get(other)
                        .is_some_and(|other_deps| other_deps.contains(dep))
            })
        });
    };

    let mut lines: Vec<String> = vec![
        "[package]".into(),
        "name = \"vk\"".into(),
        format!("version = \"{crate_version}\""),
        "edition = \"2024\"".into(),
        "license = \"MIT\"".into(),
        "repository = \"https://github.com/JakubCzarlinski/vk\"".into(),
        "homepage = \"https://github.com/JakubCzarlinski/vk\"".into(),
        "authors = [\"Jakub Czarlinski <jakubczarlinski@gmail.com>\"]".into(),
        "keywords = [\"vulkan\", \"ffi\", \"bindings\", \"graphics\"]".into(),
        "categories = [\"api-bindings\", \"external-ffi-bindings\", \"graphics\"]".into(),
        "rust-version = \"1.95\"".into(),
        "description = \"Auto-generated Vulkan FFI (vk-codegen)\"".into(),
        String::new(),
        "[dependencies]".into(),
        "libloading = { version = \"0.9.0\", default-features = false }\n".into(),
        "[features]".into(),
        "# Vulkan 1.0 enabled by default".into(),
        "default = [\"VK_VERSION_1_0\"]".into(),
        String::new(),
    ];

    lines.push("# Core Vulkan versions".into());
    for feat in &reg.features {
        if let Some(ref comment) = feat.comment {
            lines.push(format!("# {comment}"));
        }
        lines.push(format!("# version: {}", feat.number));
        let mut deps = filter_deps(feature_deps.get(&feat.name).cloned().unwrap_or_default());
        simplify_deps(&mut deps);
        lines.push(format!("{} = [{}]", feat.name, toml_feat_list(&deps)));
    }
    lines.push(String::new());

    lines.push("# Extensions".into());
    for ext in &reg.extensions {
        if ext.is_disabled() || ext.is_video_header() {
            continue;
        }

        // Cargo features can't express OR dependencies. `feature_deps` contains
        // the dependencies common to every valid XML clause plus the core API
        // slices required by the extension's generated declarations.
        let mut common_deps = feature_deps.get(&ext.name).cloned().unwrap_or_default();

        // Filter out any dependencies that aren't actually known features
        common_deps.retain(|d| known_features.contains(d));
        simplify_deps(&mut common_deps);

        if let Some(ref s) = ext.depr.superseded_by {
            lines.push(format!("# superseded by: {s}"));
        }
        if let Some(ref s) = ext.depr.obsoleted_by {
            lines.push(format!("# obsoleted by: {s}"));
        }
        if let Some(ref s) = ext.depr.promoted_to {
            lines.push(format!("# promoted to: {s}"));
        }
        if let Some(ref s) = ext.depr.deprecated {
            lines.push(format!("# deprecated: {s}"));
        }
        if let Some(ref s) = ext.requires_core {
            lines.push(format!("# requires core: {s}"));
        }
        if let Some(ref s) = ext.ext_type {
            lines.push(format!("# type: {s}"));
        }
        if let Some(ref s) = ext.comment {
            lines.push(format!("# {s}"));
        }
        lines.push(format!("{} = [{}]", ext.name, toml_feat_list(&common_deps)));
    }
    lines.push(String::new());

    lines.join("\n")
}

fn transitive_dependencies(
    direct: &IndexMap<String, Vec<String>>,
) -> IndexMap<String, HashSet<String>> {
    let mut transitive: IndexMap<String, HashSet<String>> = direct
        .iter()
        .map(|(feature, dependencies)| (feature.clone(), dependencies.iter().cloned().collect()))
        .collect();
    loop {
        let mut changed = false;
        for feature in direct.keys() {
            let current = transitive.get(feature).cloned().unwrap_or_default();
            let mut expanded = current.clone();
            for dependency in current {
                if let Some(indirect) = transitive.get(&dependency) {
                    expanded.extend(indirect.iter().cloned());
                }
            }
            if transitive.get(feature) != Some(&expanded) {
                transitive.insert(feature.clone(), expanded);
                changed = true;
            }
        }
        if !changed {
            return transitive;
        }
    }
}

fn inferred_core_dependencies(reg: &Registry, ext: &Extension) -> Vec<String> {
    let mut visitor = CoreDependencyVisitor {
        reg,
        extension: &ext.name,
        api: &ext.api,
        visited_types: HashSet::new(),
        visited_commands: HashSet::new(),
        dependencies: Vec::new(),
    };

    // A conditional require block is emitted only when its own dependency is
    // enabled, so it must not strengthen the extension feature unconditionally.
    for require in ext
        .requires
        .iter()
        .filter(|require| require.depends.is_none())
    {
        for name in &require.types {
            visitor.visit_type(name);
        }
        for name in &require.commands {
            visitor.visit_command(name);
        }
        for require_enum in &require.enums {
            if let Some(parent) = &require_enum.extends {
                visitor.visit_type(parent);
            }
        }
    }

    visitor.dependencies
}

struct CoreDependencyVisitor<'a> {
    reg: &'a Registry,
    extension: &'a str,
    api: &'a ApiSet,
    visited_types: HashSet<String>,
    visited_commands: HashSet<String>,
    dependencies: Vec<String>,
}

impl CoreDependencyVisitor<'_> {
    fn visit_command(&mut self, name: &str) {
        self.visit_command_definition(name, true);
    }

    fn visit_command_definition(&mut self, name: &str, require_extension_provider: bool) {
        if !self.visited_commands.insert(name.to_owned()) {
            return;
        }
        let Some(commands) = self.reg.commands.get(name) else {
            return;
        };
        let mut command_dependencies = Vec::new();
        for command in commands {
            if !command.api.intersects(self.api)
                || (require_extension_provider
                    && !command
                        .provided_by
                        .iter()
                        .any(|provider| provider == self.extension)
                    && command.alias.is_none())
            {
                continue;
            }
            if let Some(alias) = &command.alias {
                command_dependencies.push((true, alias.clone()));
                continue;
            }
            command_dependencies.push((false, command.return_type.base.clone()));
            for param in &command.params {
                if param
                    .api
                    .as_ref()
                    .is_none_or(|param_api| param_api.intersects(self.api))
                {
                    command_dependencies.push((false, param.ty.base.clone()));
                }
            }
        }
        for (is_command, dependency) in command_dependencies {
            if is_command {
                self.visit_command_definition(&dependency, false);
            } else {
                self.visit_type(&dependency);
            }
        }
    }

    fn visit_type(&mut self, name: &str) {
        if !self.visited_types.insert(name.to_owned()) {
            return;
        }

        let extension_owns_type = self
            .reg
            .structs
            .get(name)
            .into_iter()
            .flatten()
            .any(|item| self.is_extension_item(&item.api, &item.provided_by))
            || self
                .reg
                .typedefs
                .get(name)
                .into_iter()
                .flatten()
                .any(|item| self.is_extension_item(&item.api, &item.provided_by))
            || self
                .reg
                .enums
                .get(name)
                .into_iter()
                .flatten()
                .any(|item| self.is_extension_item(&item.api, &item.provided_by));

        if !extension_owns_type {
            if let Some(provider) = self.core_provider(name)
                && !self.dependencies.contains(&provider)
            {
                self.dependencies.push(provider);
            }
            return;
        }

        let mut type_dependencies = Vec::new();
        if let Some(structs) = self.reg.structs.get(name) {
            for item in structs {
                if !self.is_extension_item(&item.api, &item.provided_by) {
                    continue;
                }
                if let Some(alias) = &item.alias {
                    type_dependencies.push(alias.clone());
                }
                for member in &item.members {
                    if member
                        .api
                        .as_ref()
                        .is_none_or(|member_api| member_api.intersects(self.api))
                    {
                        type_dependencies.push(member.ty.base.clone());
                    }
                }
            }
        }
        if let Some(typedefs) = self.reg.typedefs.get(name) {
            for item in typedefs {
                if !self.is_extension_item(&item.api, &item.provided_by) {
                    continue;
                }
                if let Some(alias) = &item.alias {
                    type_dependencies.push(alias.clone());
                }
                if let TypedefKind::Handle { parent, .. } = &item.kind
                    && let Some(parent) = parent
                {
                    type_dependencies.push(parent.clone());
                }
                if let Some(ty) = &item.ty {
                    for dependency in type_names(ty) {
                        type_dependencies.push(dependency.to_owned());
                    }
                }
                if let Some(bits) = &item.bitmask_bits {
                    type_dependencies.push(bits.clone());
                }
            }
        }
        if let Some(enums) = self.reg.enums.get(name) {
            for item in enums {
                if !self.is_extension_item(&item.api, &item.provided_by) {
                    continue;
                }
                if let Some(alias) = &item.alias {
                    type_dependencies.push(alias.clone());
                }
            }
        }
        for dependency in type_dependencies {
            self.visit_type(&dependency);
        }
    }

    fn is_extension_item(&self, api: &ApiSet, providers: &[String]) -> bool {
        api.intersects(self.api) && providers.iter().any(|provider| provider == self.extension)
    }

    fn core_provider(&self, name: &str) -> Option<String> {
        let mut providers = Vec::new();
        let mut add = |api: &ApiSet, item_providers: &[String]| {
            if !api.intersects(self.api) {
                return;
            }
            for provider in item_providers {
                if is_core_api_feature(provider) && !providers.contains(provider) {
                    providers.push(provider.clone());
                }
            }
        };

        if let Some(items) = self.reg.structs.get(name) {
            for item in items {
                add(&item.api, &item.provided_by);
            }
        }
        if let Some(items) = self.reg.typedefs.get(name) {
            for item in items {
                add(&item.api, &item.provided_by);
            }
        }
        if let Some(items) = self.reg.enums.get(name) {
            for item in items {
                add(&item.api, &item.provided_by);
            }
        }

        providers.into_iter().min_by_key(|provider| {
            self.reg
                .features
                .iter()
                .position(|feature| feature.name == *provider)
                .unwrap_or(usize::MAX)
        })
    }
}

fn is_core_api_feature(name: &str) -> bool {
    [
        "VK_BASE_VERSION_",
        "VK_COMPUTE_VERSION_",
        "VK_GRAPHICS_VERSION_",
        "VK_VERSION_",
        "VKSC_VERSION_",
    ]
    .iter()
    .any(|prefix| name.starts_with(prefix))
}

fn type_names(encoded: &str) -> impl Iterator<Item = &str> {
    encoded
        .split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .filter(|token| token.starts_with("Vk") || token.starts_with("PFN_vk"))
}

fn toml_feat_list(deps: &[String]) -> String {
    deps.iter()
        .map(|d| format!("\"{d}\""))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::gen_cargo_toml;
    use crate::parser::{apply_require_extensions, parse_registry};

    #[test]
    fn registry_declarations_select_the_required_core_api_slice() {
        let xml =
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../registry/vk.xml"))
                .expect("read Vulkan registry");
        let mut registry = parse_registry(&xml);
        apply_require_extensions(&mut registry);
        registry.simplify_features();

        let cargo = gen_cargo_toml(&registry, "0.0.0");
        assert!(cargo.contains("VK_EXT_debug_utils = [\"VK_BASE_VERSION_1_0\"]"));
        assert!(cargo.contains("VK_NVX_image_view_handle = [\"VK_COMPUTE_VERSION_1_0\"]"));
        assert!(cargo.contains("VK_KHR_dynamic_rendering = [\"VK_GRAPHICS_VERSION_1_3\"]"));
        assert!(cargo.contains("VK_EXT_depth_range_unrestricted = []"));
    }
}
