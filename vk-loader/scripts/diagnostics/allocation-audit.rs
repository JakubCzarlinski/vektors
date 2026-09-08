//! Source inventory, not a proof of allocation safety: no type checking or macro expansion.

extern crate alloc;

use alloc::{collections::BTreeMap, string::String, vec::Vec};
use core::mem;
use std::{
    fs,
    io::{self, Write as _},
    path::{Path, PathBuf},
};

use clap::Parser;
use quote::ToTokens;
use syn::{
    parse::Parser as _,
    spanned::Spanned as _,
    visit::{self, Visit},
};

#[derive(Parser)]
#[command(
    about = "Inventory potential loader allocations across all platform cfgs; not a safety proof"
)]
struct Arguments {
    #[arg(default_value = "vk-loader")]
    root: PathBuf,
    #[arg(long)]
    include_generated: bool,
    #[arg(long)]
    include_tests: bool,
    /// Include unclassified calls and macros to expose gaps in the name-based inventory.
    #[arg(long)]
    all_calls: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct Finding {
    line: usize,
    column: usize,
    function: String,
    cfg: String,
    category: &'static str,
    operation: String,
    expression: String,
    consumer: String,
}

struct Inventory {
    findings: Vec<Finding>,
    function: String,
    cfg: Vec<String>,
    include_tests: bool,
    consumer: String,
    all_calls: bool,
}

fn is_test(attributes: &[syn::Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        attribute.path().is_ident("test")
            || (attribute.path().is_ident("cfg")
                && attribute
                    .parse_args::<syn::Meta>()
                    .is_ok_and(|meta| requires_test(&meta)))
    })
}

fn requires_test(meta: &syn::Meta) -> bool {
    match meta {
        syn::Meta::Path(path) => path.is_ident("test"),
        syn::Meta::List(list) => {
            let Ok(items) = list.parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            ) else {
                return false;
            };
            match (list.path.is_ident("all"), list.path.is_ident("any")) {
                (true, _) => items.iter().any(requires_test),
                (_, true) => !items.is_empty() && items.iter().all(requires_test),
                _ => false,
            }
        }
        _ => false,
    }
}

impl Inventory {
    fn with_attributes(&mut self, attributes: &[syn::Attribute], visit: impl FnOnce(&mut Self)) {
        if !self.include_tests && is_test(attributes) {
            return;
        }
        let depth = self.cfg.len();
        self.attributes(attributes);
        visit(self);
        self.cfg.truncate(depth);
    }

    fn new(include_tests: bool) -> Self {
        Self {
            findings: Vec::new(),
            function: String::from("<module>"),
            cfg: Vec::new(),
            include_tests,
            consumer: String::new(),
            all_calls: false,
        }
    }

    fn attributes(&mut self, attributes: &[syn::Attribute]) {
        self.cfg.extend(
            attributes
                .iter()
                .filter(|attribute| {
                    attribute.path().is_ident("cfg") || attribute.path().is_ident("cfg_attr")
                })
                .map(|attribute| attribute.meta.to_token_stream().to_string()),
        );
    }

    fn record(
        &mut self,
        node: &impl ToTokens,
        span: proc_macro2::Span,
        category: &'static str,
        operation: String,
    ) {
        let start = span.start();
        self.findings.push(Finding {
            line: start.line,
            column: start.column + 1,
            function: self.function.clone(),
            cfg: self.cfg.join(" && "),
            category,
            operation,
            expression: node
                .to_token_stream()
                .to_string()
                .replace(['\n', '\t'], " "),
            consumer: self.consumer.clone(),
        });
    }
}

fn method_category(name: &str) -> Option<&'static str> {
    match name {
        "to_owned" | "to_string" | "to_vec" | "to_path_buf" | "repeat" | "to_lowercase"
        | "to_uppercase" => Some("owning-conversion"),
        "push" | "push_str" | "insert" | "extend" | "extend_from_slice" | "extend_from_within"
        | "resize" | "resize_with" | "reserve" | "reserve_exact" | "shrink_to_fit"
        | "shrink_to" | "append" | "write_fmt" | "write_str" => {
            Some("capacity-or-writer-dependent")
        }
        "clone" | "clone_from" | "cloned" | "collect" | "into" | "to_string_lossy"
        | "into_owned" | "join" | "replace" | "replacen" | "path" | "file_name" | "split_off"
        | "unzip" | "partition" | "concat" => Some("type-dependent"),
        "canonicalize" | "read_dir" | "exists" | "try_exists" | "is_file" | "is_dir"
        | "metadata" | "symlink_metadata" | "read_to_end" | "read_to_string" => {
            Some("stdlib-hidden-allocation")
        }
        "into_boxed_slice" | "into_boxed_str" => Some("may-shrink-or-copy"),
        "into_vec" | "into_string" | "into_bytes" => Some("ownership-transfer"),
        "try_reserve" | "try_reserve_exact" => Some("fallible-boundary"),
        // Once's queue backend can allocate a system-owned thread handle on
        // contention; mutex storage also differs between standard backends.
        // These are review candidates, not unconditional allocation claims.
        "call_once" | "call_once_force" | "get_or_init" | "get_or_try_init" => {
            Some("synchronization-backend-dependent")
        }
        // A fallible signature still needs its caller's error/rollback reviewed.
        // This also records our lazily initialized native global mutexes.
        "try_lock" => Some("fallible-synchronization"),
        _ => None,
    }
}

fn path_category(path: &syn::Path) -> Option<&'static str> {
    let name = path.segments.last()?.ident.to_string();
    if name.starts_with("try_")
        && matches!(
            name.as_str(),
            "try_box"
                | "try_box_uninit"
                | "try_box_uninit_slice"
                | "try_boxed_slice_filled"
                | "try_collect"
                | "try_collect_results"
                | "try_string"
                | "try_path"
                | "try_os_string"
                | "try_c_string"
                | "try_c_string_bytes"
                | "try_into_boxed_slice"
                | "try_push"
        )
    {
        return Some("fallible-boundary");
    }
    let owner = path
        .segments
        .iter()
        .rev()
        .nth(1)
        .map(|segment| segment.ident.to_string());
    match (owner.as_deref(), name.as_str()) {
        (Some("GlobalMutex" | "GlobalLazyMutex"), "new") => {
            Some("deferred-synchronization-initialization")
        }
        (Some("ObjectMutex" | "Mutex"), "try_new") => Some("fallible-synchronization"),
        (Some("Mutex" | "RwLock" | "Once" | "OnceLock" | "LazyLock"), "new" | "force") => {
            Some("synchronization-backend-dependent")
        }
        (Some("Vec" | "String" | "HashMap" | "HashSet" | "VecDeque"), "new") => None,
        (Some("Box" | "Rc" | "Arc" | "CString"), "new" | "new_uninit" | "new_uninit_slice")
        | (
            Some("Vec" | "String" | "OsString" | "PathBuf" | "HashMap" | "HashSet" | "VecDeque"),
            "with_capacity",
        )
        | (Some("OsString"), "from_wide") => Some("infallible-allocation"),
        (
            Some("Box" | "Vec" | "String" | "OsString" | "PathBuf" | "CString" | "Rc" | "Arc"),
            "from" | "from_iter" | "default",
        ) => Some("type-dependent"),
        (
            Some("env"),
            "var" | "var_os" | "vars" | "vars_os" | "current_exe" | "current_dir" | "split_paths",
        )
        | (
            Some("fs"),
            "read" | "read_to_string" | "read_dir" | "canonicalize" | "read_link" | "metadata"
            | "symlink_metadata",
        )
        | (Some("File"), "open" | "create" | "options") => Some("stdlib-hidden-allocation"),
        (Some("thread"), "current" | "park" | "park_timeout") => Some("stdlib-hidden-allocation"),
        (_, "alloc" | "alloc_zeroed" | "realloc" | "malloc" | "calloc") => {
            Some("raw-allocation-check-null")
        }
        (_, "to_owned" | "to_string" | "to_vec" | "to_path_buf" | "clone") => {
            Some("type-dependent")
        }
        _ => None,
    }
}

impl<'ast> Visit<'ast> for Inventory {
    fn visit_attribute(&mut self, node: &'ast syn::Attribute) {
        if self.all_calls && node.path().is_ident("derive") {
            self.record(
                node,
                node.span(),
                "derived-implementation",
                String::from("derive"),
            );
        }
        visit::visit_attribute(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        let consumer = match node.func.as_ref() {
            syn::Expr::Path(path) => path.path.to_token_stream().to_string(),
            _ => String::from("<indirect-call>"),
        };
        let previous = mem::replace(&mut self.consumer, consumer);
        if self.all_calls {
            match node.func.as_ref() {
                syn::Expr::Path(path) if path_category(&path.path).is_none() => self.record(
                    node,
                    node.span(),
                    "unclassified-call",
                    path.path.to_token_stream().to_string(),
                ),
                syn::Expr::Path(_) => {}
                _ => self.record(
                    node,
                    node.span(),
                    "indirect-call",
                    node.func.to_token_stream().to_string(),
                ),
            }
        }
        visit::visit_expr_call(self, node);
        self.consumer = previous;
    }
    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        self.with_attributes(&node.attrs, |inventory| {
            visit::visit_item_mod(inventory, node)
        });
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        self.with_attributes(&node.attrs, |inventory| {
            visit::visit_item_impl(inventory, node)
        });
    }

    fn visit_item_static(&mut self, node: &'ast syn::ItemStatic) {
        self.with_attributes(&node.attrs, |inventory| {
            visit::visit_item_static(inventory, node)
        });
    }

    fn visit_item_const(&mut self, node: &'ast syn::ItemConst) {
        self.with_attributes(&node.attrs, |inventory| {
            visit::visit_item_const(inventory, node)
        });
    }

    fn visit_impl_item_const(&mut self, node: &'ast syn::ImplItemConst) {
        self.with_attributes(&node.attrs, |inventory| {
            visit::visit_impl_item_const(inventory, node)
        });
    }

    fn visit_trait_item_const(&mut self, node: &'ast syn::TraitItemConst) {
        self.with_attributes(&node.attrs, |inventory| {
            visit::visit_trait_item_const(inventory, node)
        });
    }

    fn visit_expr_block(&mut self, node: &'ast syn::ExprBlock) {
        self.with_attributes(&node.attrs, |inventory| {
            visit::visit_expr_block(inventory, node)
        });
    }

    fn visit_local(&mut self, node: &'ast syn::Local) {
        self.with_attributes(&node.attrs, |inventory| visit::visit_local(inventory, node));
    }

    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        if !self.include_tests && is_test(&node.attrs) {
            return;
        }
        let previous = mem::replace(&mut self.function, node.sig.ident.to_string());
        let depth = self.cfg.len();
        self.attributes(&node.attrs);
        visit::visit_item_fn(self, node);
        self.cfg.truncate(depth);
        self.function = previous;
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        if !self.include_tests && is_test(&node.attrs) {
            return;
        }
        let previous = mem::replace(&mut self.function, node.sig.ident.to_string());
        let depth = self.cfg.len();
        self.attributes(&node.attrs);
        visit::visit_impl_item_fn(self, node);
        self.cfg.truncate(depth);
        self.function = previous;
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        let name = node.method.to_string();
        if let Some(category) = method_category(&name) {
            self.record(node, node.method.span(), category, name);
        } else if self.all_calls {
            self.record(node, node.method.span(), "unclassified-method", name);
        }
        visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_path(&mut self, node: &'ast syn::ExprPath) {
        if let Some(category) = path_category(&node.path) {
            self.record(
                node,
                node.span(),
                category,
                node.path.to_token_stream().to_string(),
            );
        }
        visit::visit_expr_path(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        let Some(segment) = node.path.segments.last() else {
            return;
        };
        let name = segment.ident.to_string();
        match name.as_str() {
            "format" | "vec" => self.record(node, node.span(), "allocating-macro", name.clone()),
            "write" | "writeln" => self.record(node, node.span(), "writer-dependent", name.clone()),
            "format_args" => {}
            _ if self.all_calls => {
                self.record(node, node.span(), "unexpanded-macro", name.clone());
            }
            _ => {}
        }
        // Parse known expression-list macros, including allocations hidden in format_args!.
        // Custom macro expansion remains outside this inventory's guarantees.
        if matches!(
            name.as_str(),
            "format" | "format_args" | "write" | "writeln"
        ) {
            if let Ok(arguments) =
                syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated
                    .parse2(node.tokens.clone())
            {
                for expression in &arguments {
                    self.visit_expr(expression);
                }
            }
        } else if name == "vec" {
            let tokens = &node.tokens;
            if let Ok(expression) = syn::parse2::<syn::Expr>(quote::quote!([#tokens])) {
                self.visit_expr(&expression);
            }
        }
    }
}

fn source_files(
    directory: &Path,
    arguments: &Arguments,
    output: &mut Vec<PathBuf>,
) -> io::Result<()> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        let kind = entry.file_type()?;
        if kind.is_dir() {
            if arguments.include_generated || entry.file_name() != "generated" {
                source_files(&path, arguments, output)?;
            }
        } else if kind.is_file()
            && path.extension().is_some_and(|extension| extension == "rs")
            && (arguments.include_tests
                || !matches!(
                    path.file_stem().and_then(|name| name.to_str()),
                    Some("tests" | "fault")
                ))
        {
            output.push(path);
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = Arguments::parse();
    let mut files = Vec::new();
    for directory in ["src", "json"] {
        source_files(&arguments.root.join(directory), &arguments, &mut files)?;
    }
    files.sort();
    let mut output = io::BufWriter::new(io::stdout().lock());
    writeln!(
        output,
        "file\tline\tcolumn\tfunction\tcfg\tcategory\toperation\texpression\tconsumer"
    )?;
    let mut counts = BTreeMap::new();
    for path in &files {
        let source = fs::read_to_string(path)?;
        let syntax =
            syn::parse_file(&source).map_err(|error| format!("{}: {error}", path.display()))?;
        let mut inventory = Inventory::new(arguments.include_tests);
        inventory.all_calls = arguments.all_calls;
        inventory.visit_file(&syntax);
        inventory
            .findings
            .sort_by_key(|finding| (finding.line, finding.column));
        for finding in inventory.findings {
            *counts.entry(finding.category).or_insert(0_usize) += 1;
            writeln!(
                output,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                path.display(),
                finding.line,
                finding.column,
                finding.function,
                finding.cfg,
                finding.category,
                finding.operation,
                finding.expression,
                finding.consumer
            )?;
        }
    }
    output.flush()?;
    eprintln!(
        "Scanned {} files across all source cfg branches; findings require review.",
        files.len()
    );
    for (category, count) in counts {
        eprintln!("{category}: {count}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retains_initializer_and_local_platform_guards_without_leaking_them() {
        let syntax = syn::parse_file(
            r#"
            #[cfg(windows)] static LOCK: Mutex<()> = Mutex::new(());
            #[cfg(unix)] const LOCK: Mutex<()> = Mutex::new(());
            #[cfg(test)] static TEST: Mutex<()> = Mutex::new(());
            fn example() {
                #[cfg(windows)] let path = data.to_owned();
                #[cfg(unix)] { data.to_vec(); }
                #[cfg(test)] { data.to_string(); }
                data.into_vec();
            }
            impl Example { #[cfg(windows)] const LOCK: Mutex<()> = Mutex::new(()); }
            trait ExampleTrait { #[cfg(unix)] const LOCK: Mutex<()> = Mutex::new(()); }
        "#,
        )
        .unwrap();
        let mut inventory = Inventory::new(false);
        inventory.visit_file(&syntax);
        let guards: Vec<_> = inventory
            .findings
            .iter()
            .map(|finding| finding.cfg.as_str())
            .collect();
        assert_eq!(
            guards,
            [
                "cfg (windows)",
                "cfg (unix)",
                "cfg (windows)",
                "cfg (unix)",
                "",
                "cfg (windows)",
                "cfg (unix)"
            ]
        );
        let mut with_tests = Inventory::new(true);
        with_tests.visit_file(&syntax);
        assert_eq!(with_tests.findings.len(), inventory.findings.len() + 2);
    }

    #[test]
    fn tracks_deferred_mutex_initialization_and_fallible_acquisition() {
        let syntax = syn::parse_file(
            "fn example() { GlobalMutex::new(None); GlobalLazyMutex::new(init); ObjectMutex::try_new(value)?; registry.try_lock()?; Once::new(); once.call_once(init); }",
        ).unwrap();
        let mut inventory = Inventory::new(false);
        inventory.visit_file(&syntax);
        let categories: Vec<_> = inventory
            .findings
            .iter()
            .map(|finding| finding.category)
            .collect();
        assert_eq!(
            categories,
            [
                "deferred-synchronization-initialization",
                "deferred-synchronization-initialization",
                "fallible-synchronization",
                "fallible-synchronization",
                "synchronization-backend-dependent",
                "synchronization-backend-dependent",
            ]
        );
    }

    #[test]
    fn all_calls_exposes_wrappers_indirection_and_unknown_macros() {
        let syntax = syn::parse_file(
            "fn example() { wrapper(); handle.allocate(); (callbacks.allocate)(4); opendir()(path); custom!(hidden()); }",
        ).unwrap();
        let mut inventory = Inventory::new(false);
        inventory.all_calls = true;
        inventory.visit_file(&syntax);
        let operations: Vec<_> = inventory
            .findings
            .iter()
            .map(|finding| (finding.category, finding.operation.as_str()))
            .collect();
        assert!(operations.contains(&("unclassified-call", "wrapper")));
        assert!(operations.contains(&("unclassified-method", "allocate")));
        assert!(operations.contains(&("indirect-call", "(callbacks . allocate)")));
        assert!(operations.contains(&("indirect-call", "opendir ()")));
        assert!(operations.contains(&("unexpanded-macro", "custom")));
        // Unknown macro bodies are deliberately not reported as expanded calls.
        assert!(
            !operations
                .iter()
                .any(|(_, operation)| *operation == "hidden")
        );
    }

    #[test]
    fn all_calls_records_derived_implementations_without_claiming_they_allocate() {
        let syntax =
            syn::parse_file("#[derive(Clone)] struct Data { values: Vec<String> }").unwrap();
        let mut inventory = Inventory::new(false);
        inventory.all_calls = true;
        inventory.visit_file(&syntax);
        assert_eq!(inventory.findings.len(), 1);
        assert_eq!(inventory.findings[0].category, "derived-implementation");
    }

    #[test]
    fn distinguishes_format_arguments_from_nested_owned_formatting() {
        let syntax =
            syn::parse_file(r#"fn example() { log(format_args!("{}", format!("{}", value))); }"#)
                .unwrap();
        let mut inventory = Inventory::new(false);
        inventory.visit_file(&syntax);
        assert_eq!(inventory.findings.len(), 1);
        assert_eq!(inventory.findings[0].operation, "format");
    }

    #[test]
    fn visits_all_platform_branches_but_excludes_test_modules() {
        let syntax = syn::parse_file(
            r#"
            #[cfg(windows)] fn windows() { OsString::from_wide(units); }
            #[cfg(unix)] fn unix() { bytes.to_vec(); }
            #[cfg(test)] mod tests { fn test() { Box::new(1); } }
        "#,
        )
        .unwrap();
        let mut inventory = Inventory::new(false);
        inventory.visit_file(&syntax);
        assert_eq!(inventory.findings.len(), 2);
        assert!(inventory.findings[0].cfg.contains("windows"));
        assert!(inventory.findings[1].cfg.contains("unix"));
        let mut all = Inventory::new(true);
        all.visit_file(&syntax);
        assert_eq!(all.findings.len(), 3);
    }

    #[test]
    fn compound_test_cfg_does_not_hide_platform_code() {
        let syntax = syn::parse_file(
            r#"
            #[cfg(all(test, windows))] fn test_only() { Box::new(1); }
            #[cfg(any(test, windows))] fn windows_too() { Box::new(2); }
        "#,
        )
        .unwrap();
        let mut inventory = Inventory::new(false);
        inventory.visit_file(&syntax);
        assert_eq!(inventory.findings.len(), 1);
        assert_eq!(inventory.findings[0].function, "windows_too");
    }

    #[test]
    fn distinguishes_ownership_transfer_and_fallible_growth() {
        let syntax = syn::parse_file(
            "fn example() { data.into_vec(); data.try_reserve(1)?; data.push(value); data.append(&mut other); data.split_off(1); }",
        )
        .unwrap();
        let mut inventory = Inventory::new(false);
        inventory.visit_file(&syntax);
        let categories: Vec<_> = inventory
            .findings
            .iter()
            .map(|finding| finding.category)
            .collect();
        assert_eq!(
            categories,
            [
                "ownership-transfer",
                "fallible-boundary",
                "capacity-or-writer-dependent",
                "capacity-or-writer-dependent",
                "type-dependent"
            ]
        );
    }
}
