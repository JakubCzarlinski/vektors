use std::collections::{HashMap, HashSet};

use crate::ir::{ApiSet, Availability, Registry, TypedefKind};

#[derive(Clone)]
struct DeclarationUse {
    api: ApiSet,
    availability: Vec<Availability>,
    references: Vec<String>,
}

/// Makes extension-owned declarations available wherever another declaration
/// embeds or aliases them.
///
/// Vulkan extension dependencies describe runtime/spec requirements. They do
/// not necessarily describe the declarations reused in an extension's C ABI.
/// For example, `VK_QCOM_render_pass_transform` embeds
/// `VkSurfaceTransformFlagBitsKHR` without depending on `VK_KHR_surface`.
/// Propagating the consumer's availability to that enum lets the Rust ABI be
/// emitted without inventing a Vulkan extension dependency.
pub fn close_declaration_availability(registry: &mut Registry) {
    let extension_declarations = extension_declaration_names(registry);
    let feature_implications = registry
        .transitive_deps()
        .into_iter()
        .collect::<HashMap<_, _>>();

    loop {
        let uses = declaration_uses(registry);
        let mut additions = Vec::<(String, ApiSet, Availability)>::new();

        for declaration in uses {
            for reference in declaration.references {
                if !extension_declarations.contains(&reference) {
                    continue;
                }
                for availability in &declaration.availability {
                    if type_is_available(
                        registry,
                        &reference,
                        &declaration.api,
                        availability,
                        &feature_implications,
                    ) {
                        continue;
                    }
                    additions.push((
                        reference.clone(),
                        declaration.api.clone(),
                        availability.clone(),
                    ));
                }
            }
        }

        let mut changed = false;
        for (name, api, availability) in additions {
            changed |= add_type_availability(registry, &name, &api, &availability);
        }
        if !changed {
            break;
        }
    }
}

fn type_is_available(
    registry: &Registry,
    name: &str,
    source_api: &ApiSet,
    source: &Availability,
    implications: &HashMap<String, HashSet<String>>,
) -> bool {
    let mut targets = Vec::new();
    if let Some(items) = registry.typedefs.get(name) {
        for item in items.iter().filter(|item| item.api.intersects(source_api)) {
            targets.extend(item_availability(
                &item.provided_by,
                &item.availability,
                &item.dep,
            ));
        }
    }
    if let Some(items) = registry.structs.get(name) {
        for item in items.iter().filter(|item| item.api.intersects(source_api)) {
            targets.extend(item_availability(
                &item.provided_by,
                &item.availability,
                &item.dep,
            ));
        }
    }
    if let Some(items) = registry.enums.get(name) {
        for item in items.iter().filter(|item| item.api.intersects(source_api)) {
            targets.extend(item_availability(
                &item.provided_by,
                &item.availability,
                &item.dep,
            ));
            for variant in &item.variants {
                targets.extend(item_availability(
                    &variant.provided_by,
                    &variant.availability,
                    &variant.dep,
                ));
            }
        }
    }

    source_implies_target(source, &targets, implications)
}

fn source_implies_target(
    source: &Availability,
    targets: &[Availability],
    implications: &HashMap<String, HashSet<String>>,
) -> bool {
    if targets.contains(source) {
        return true;
    }

    let source_clauses = availability_clauses(source);
    source_clauses.iter().all(|source_clause| {
        targets.iter().any(|target| {
            if !target.excluded_by.is_empty() {
                return false;
            }
            availability_clauses(target).iter().any(|target_clause| {
                target_clause.iter().all(|required| {
                    source_clause
                        .iter()
                        .any(|feature| feature_implies(feature, required, implications))
                })
            })
        })
    })
}

fn availability_clauses(availability: &Availability) -> Vec<Vec<String>> {
    let mut clauses = availability
        .dep
        .as_ref()
        .map(crate::ir::DepExpr::to_dnf_clauses)
        .unwrap_or_else(|| vec![Vec::new()]);
    for clause in &mut clauses {
        if !clause.contains(&availability.provider) {
            clause.push(availability.provider.clone());
        }
    }
    clauses
}

fn feature_implies(
    feature: &str,
    required: &str,
    implications: &HashMap<String, HashSet<String>>,
) -> bool {
    feature == required
        || implications
            .get(feature)
            .is_some_and(|dependencies| dependencies.contains(required))
        || (required == "VK_BASE_VERSION_1_0"
            && (is_core_provider(feature) || is_extension_provider(feature)))
}

fn is_extension_provider(provider: &str) -> bool {
    provider.starts_with("VK_") && !is_core_provider(provider) && provider != "VKSC_VERSION_1_0"
}

fn extension_declaration_names(registry: &Registry) -> HashSet<String> {
    let mut names = HashSet::new();
    for (name, items) in &registry.typedefs {
        if items
            .iter()
            .any(|item| has_extension_provider(&item.provided_by))
        {
            names.insert(name.clone());
        }
    }
    for (name, items) in &registry.structs {
        if items
            .iter()
            .any(|item| has_extension_provider(&item.provided_by))
        {
            names.insert(name.clone());
        }
    }
    for (name, items) in &registry.enums {
        if items
            .iter()
            .any(|item| has_extension_provider(&item.provided_by))
        {
            names.insert(name.clone());
        }
    }
    names
}

fn has_extension_provider(providers: &[String]) -> bool {
    providers.iter().any(|provider| !is_core_provider(provider))
}

fn is_core_provider(provider: &str) -> bool {
    [
        "VK_BASE_VERSION_",
        "VK_COMPUTE_VERSION_",
        "VK_GRAPHICS_VERSION_",
        "VK_VERSION_",
        "VKSC_VERSION_",
    ]
    .iter()
    .any(|prefix| provider.starts_with(prefix))
}

fn declaration_uses(registry: &Registry) -> Vec<DeclarationUse> {
    let mut uses = Vec::new();

    for item in registry.typedefs.values().flatten() {
        let mut references = Vec::new();
        extend_optional(&mut references, item.alias.as_ref());
        extend_optional(&mut references, item.bitmask_bits.as_ref());
        if let TypedefKind::Handle { parent, .. } = &item.kind {
            extend_optional(&mut references, parent.as_ref());
        }
        if let Some(encoded) = &item.ty {
            references.extend(type_names(encoded).map(str::to_owned));
        }
        uses.push(DeclarationUse {
            api: item.api.clone(),
            availability: item_availability(&item.provided_by, &item.availability, &item.dep),
            references,
        });
    }

    for item in registry.structs.values().flatten() {
        let mut references = item
            .members
            .iter()
            .filter(|member| {
                member
                    .api
                    .as_ref()
                    .is_none_or(|api| api.intersects(&item.api))
            })
            .map(|member| member.ty.base.clone())
            .collect::<Vec<_>>();
        extend_optional(&mut references, item.alias.as_ref());
        uses.push(DeclarationUse {
            api: item.api.clone(),
            availability: item_availability(&item.provided_by, &item.availability, &item.dep),
            references,
        });
    }

    for item in registry.enums.values().flatten() {
        let mut references = Vec::new();
        extend_optional(&mut references, item.alias.as_ref());
        uses.push(DeclarationUse {
            api: item.api.clone(),
            availability: item_availability(&item.provided_by, &item.availability, &item.dep),
            references,
        });
    }

    for item in registry.commands.values().flatten() {
        let mut references = vec![item.return_type.base.clone()];
        references.extend(
            item.params
                .iter()
                .filter(|parameter| {
                    parameter
                        .api
                        .as_ref()
                        .is_none_or(|api| api.intersects(&item.api))
                })
                .map(|parameter| parameter.ty.base.clone()),
        );
        uses.push(DeclarationUse {
            api: item.api.clone(),
            availability: item_availability(&item.provided_by, &item.availability, &item.dep),
            references,
        });
    }

    uses
}

fn item_availability(
    providers: &[String],
    availability: &[Availability],
    dependency: &Option<crate::ir::DepExpr>,
) -> Vec<Availability> {
    if !availability.is_empty() {
        return availability.to_vec();
    }
    providers
        .iter()
        .map(|provider| Availability::new(provider.clone(), dependency.clone()))
        .collect()
}

fn extend_optional(references: &mut Vec<String>, reference: Option<&String>) {
    if let Some(reference) = reference {
        references.push(reference.clone());
    }
}

fn type_names(encoded: &str) -> impl Iterator<Item = &str> {
    encoded
        .split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .filter(|token| token.starts_with("Vk") || token.starts_with("PFN_vk"))
}

fn add_type_availability(
    registry: &mut Registry,
    name: &str,
    source_api: &ApiSet,
    availability: &Availability,
) -> bool {
    let mut changed = false;
    if let Some(items) = registry.typedefs.get_mut(name) {
        for item in items
            .iter_mut()
            .filter(|item| item.api.intersects(source_api))
        {
            changed |=
                add_availability(&mut item.provided_by, &mut item.availability, availability);
        }
    }
    if let Some(items) = registry.structs.get_mut(name) {
        for item in items
            .iter_mut()
            .filter(|item| item.api.intersects(source_api))
        {
            changed |=
                add_availability(&mut item.provided_by, &mut item.availability, availability);
        }
    }
    if let Some(items) = registry.enums.get_mut(name) {
        for item in items
            .iter_mut()
            .filter(|item| item.api.intersects(source_api))
        {
            changed |=
                add_availability(&mut item.provided_by, &mut item.availability, availability);
        }
    }
    changed
}

fn add_availability(
    providers: &mut Vec<String>,
    availability: &mut Vec<Availability>,
    addition: &Availability,
) -> bool {
    let mut changed = false;
    if !providers.contains(&addition.provider) {
        providers.push(addition.provider.clone());
        changed = true;
    }
    if !availability.contains(addition) {
        availability.push(addition.clone());
        changed = true;
    }
    changed
}

#[cfg(test)]
mod tests {
    use crate::parser::{apply_require_extensions, parse_registry};

    #[test]
    fn consumer_feature_exposes_reused_extension_type_without_extension_dependency() {
        let xml =
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../registry/vk.xml"))
                .expect("read Vulkan registry");
        let mut registry = parse_registry(&xml);
        apply_require_extensions(&mut registry);

        let surface_transform = registry
            .enums
            .get("VkSurfaceTransformFlagBitsKHR")
            .and_then(|items| items.first())
            .expect("surface transform enum");
        assert!(
            surface_transform
                .provided_by
                .iter()
                .any(|provider| provider == "VK_QCOM_render_pass_transform")
        );
        assert!(
            registry
                .feature_deps()
                .get("VK_QCOM_render_pass_transform")
                .is_none_or(Vec::is_empty)
        );
    }
}
