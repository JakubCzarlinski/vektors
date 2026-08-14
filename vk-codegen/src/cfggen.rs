//! Generates `#[cfg(...)]` token streams from `DepExpr` and feature-name lists.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::RwLock;

use proc_macro2::TokenStream;
use quote::quote;

use crate::ir::{Availability, DepExpr};

static FEATURE_IMPLICATIONS: RwLock<Option<BTreeMap<String, BTreeSet<String>>>> = RwLock::new(None);

pub fn set_feature_implications(implications: BTreeMap<String, BTreeSet<String>>) {
    *FEATURE_IMPLICATIONS
        .write()
        .expect("feature implication lock") = Some(implications);
}

/// `#[cfg(feature = "A")]`  or  `#[cfg(any(feature="A", feature="B", ...))]`
#[must_use]
pub fn cfg_any(features: &[String]) -> TokenStream {
    match features.len() {
        0 => quote! {},
        1 => {
            let f = &features[0];
            quote! { #[cfg(feature = #f)] }
        }
        _ => {
            let items: Vec<TokenStream> =
                features.iter().map(|f| quote! { feature = #f }).collect();
            quote! { #[cfg(any(#(#items),*))] }
        }
    }
}

/// Convert DNF clauses to a `cfg` expression.
#[must_use]
pub fn cfg_expr_from_dnf(clauses: &[Vec<String>]) -> TokenStream {
    clauses_to_ts(&simplify_clauses(clauses))
}

#[must_use]
pub fn cfg_providers_with_dep(features: &[String], dep: Option<&DepExpr>) -> TokenStream {
    let Some(dep) = dep else {
        return cfg_any(features);
    };

    let mut clauses = Vec::<Vec<String>>::new();
    let dep_clauses = dep.to_dnf_clauses();

    if features.is_empty() {
        clauses = dep_clauses;
    } else {
        for provider in features {
            for dep_clause in &dep_clauses {
                let mut clause = dep_clause.clone();
                if !clause.contains(provider) {
                    clause.insert(0, provider.clone());
                }
                clause.sort();
                if !clauses.contains(&clause) {
                    clauses.push(clause);
                }
            }
        }
    }

    let expr = cfg_expr_from_dnf(&clauses);
    quote! { #[cfg(#expr)] }
}

#[must_use]
pub fn cfg_availability(
    availability: &[Availability],
    fallback_features: &[String],
    fallback_dep: Option<&DepExpr>,
) -> TokenStream {
    if availability.is_empty() {
        return cfg_providers_with_dep(fallback_features, fallback_dep);
    }

    let expr = cfg_availability_expr(availability, fallback_features, fallback_dep);
    quote! { #[cfg(#expr)] }
}

#[must_use]
pub fn cfg_availability_expr(
    availability: &[Availability],
    fallback_features: &[String],
    fallback_dep: Option<&DepExpr>,
) -> TokenStream {
    if availability.iter().any(|item| !item.excluded_by.is_empty()) {
        let normal: Vec<Availability> = availability
            .iter()
            .filter(|item| item.excluded_by.is_empty())
            .cloned()
            .collect();
        let normal_clauses = if normal.is_empty() {
            Vec::new()
        } else {
            simplify_clauses(&availability_clauses(
                &normal,
                fallback_features,
                fallback_dep,
            ))
        };
        let mut routes: Vec<TokenStream> = availability
            .iter()
            .filter(|item| !item.excluded_by.is_empty())
            .flat_map(|item| {
                let dep_clauses = item
                    .dep
                    .as_ref()
                    .map(DepExpr::to_dnf_clauses)
                    .unwrap_or_else(|| vec![Vec::new()]);
                dep_clauses.into_iter().filter_map(|mut clause| {
                    if !clause.contains(&item.provider) {
                        clause.insert(0, item.provider.clone());
                    }
                    // An unrestricted route which is satisfied whenever this route is
                    // satisfied makes the restricted route redundant.
                    if normal_clauses
                        .iter()
                        .any(|normal| clause_implies(&clause, normal))
                    {
                        return None;
                    }
                    let positive = cfg_expr_from_dnf(&[clause]);
                    let excluded: Vec<TokenStream> = item
                        .excluded_by
                        .iter()
                        .map(|feature| quote! { feature = #feature })
                        .collect();
                    let exclusion = match excluded.as_slice() {
                        [feature] => quote! { not(#feature) },
                        _ => quote! { not(any(#(#excluded),*)) },
                    };
                    Some(quote! { all(#positive, #exclusion) })
                })
            })
            .collect();
        if !normal_clauses.is_empty() {
            routes.insert(0, cfg_expr_from_dnf(&normal_clauses));
        }
        return match routes.as_slice() {
            [] => quote! { all() },
            [route] => route.clone(),
            _ => quote! { any(#(#routes),*) },
        };
    }
    let clauses = availability_clauses(availability, fallback_features, fallback_dep);
    cfg_expr_from_dnf(&clauses)
}

#[must_use]
pub fn cfg_availability_implies(
    lhs_availability: &[Availability],
    lhs_fallback_features: &[String],
    lhs_fallback_dep: Option<&DepExpr>,
    rhs_availability: &[Availability],
    rhs_fallback_features: &[String],
    rhs_fallback_dep: Option<&DepExpr>,
) -> bool {
    let mut lhs_clauses =
        availability_clauses(lhs_availability, lhs_fallback_features, lhs_fallback_dep);
    let mut rhs_clauses =
        availability_clauses(rhs_availability, rhs_fallback_features, rhs_fallback_dep);

    if lhs_clauses.is_empty() {
        lhs_clauses.push(Vec::new());
    }
    if rhs_clauses.is_empty() {
        rhs_clauses.push(Vec::new());
    }

    lhs_clauses.iter().all(|lhs_clause| {
        rhs_clauses
            .iter()
            .any(|rhs_clause| clause_implies(lhs_clause, rhs_clause))
    })
}

pub fn push_availability(
    availability: &mut Vec<Availability>,
    provider: &str,
    dep: &Option<DepExpr>,
) {
    if provider.is_empty() {
        return;
    }
    let item = Availability::new(provider.to_owned(), dep.clone());
    if !availability.contains(&item) {
        availability.push(item);
    }
}

pub fn set_dep_if_unset(dst: &mut Option<DepExpr>, dep: &Option<DepExpr>) {
    if dep.is_none() {
        *dst = None;
    } else if dst.is_none() {
        *dst = dep.clone();
    }
}

fn availability_clauses(
    availability: &[Availability],
    fallback_features: &[String],
    fallback_dep: Option<&DepExpr>,
) -> Vec<Vec<String>> {
    if availability.is_empty() {
        let Some(dep) = fallback_dep else {
            return fallback_features
                .iter()
                .map(|provider| vec![provider.clone()])
                .collect();
        };

        let dep_clauses = dep.to_dnf_clauses();
        if fallback_features.is_empty() {
            return dep_clauses;
        }

        let mut clauses = Vec::<Vec<String>>::new();
        for provider in fallback_features {
            for dep_clause in &dep_clauses {
                let mut clause = dep_clause.clone();
                if !clause.contains(provider) {
                    clause.insert(0, provider.clone());
                }
                clause.sort();
                if !clauses.contains(&clause) {
                    clauses.push(clause);
                }
            }
        }
        return clauses;
    }

    let mut clauses = Vec::<Vec<String>>::new();
    for item in availability {
        let dep_clauses = item
            .dep
            .as_ref()
            .map(DepExpr::to_dnf_clauses)
            .unwrap_or_else(|| vec![vec![]]);
        for mut clause in dep_clauses {
            if !clause.contains(&item.provider) {
                clause.insert(0, item.provider.clone());
            }
            clause.sort();
            if !clauses.contains(&clause) {
                clauses.push(clause);
            }
        }
    }
    clauses
}

fn simplify_clauses(clauses: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut simplified = Vec::<Vec<String>>::new();

    for clause in clauses {
        let mut clause = clause.clone();
        clause.sort();
        clause.dedup();
        let original = clause.clone();
        clause = original
            .iter()
            .filter(|feature| {
                !original
                    .iter()
                    .any(|other| other != *feature && feature_implies(other, feature))
            })
            .cloned()
            .collect();
        if !simplified.contains(&clause) {
            simplified.push(clause);
        }
    }

    let mut out = Vec::new();
    'clause: for (idx, clause) in simplified.iter().enumerate() {
        for (other_idx, other) in simplified.iter().enumerate() {
            if idx != other_idx && clause_implies(clause, other) {
                continue 'clause;
            }
        }
        out.push(clause.clone());
    }
    out
}

fn clause_implies(clause: &[String], other: &[String]) -> bool {
    other.iter().all(|required| {
        clause
            .iter()
            .any(|feature| feature_implies(feature, required))
    })
}

fn feature_implies(feature: &str, required: &str) -> bool {
    feature == required
        || feature_implies_derived_dependency(feature, required)
        || feature_implies_vulkan_base_1_0(feature, required)
}

fn feature_implies_vulkan_base_1_0(feature: &str, required: &str) -> bool {
    if required != "VK_BASE_VERSION_1_0" {
        return false;
    }

    is_vulkan_core_feature(feature) || is_vulkan_extension(feature)
}

fn is_vulkan_core_feature(feature: &str) -> bool {
    feature.starts_with("VK_BASE_VERSION_")
        || feature.starts_with("VK_COMPUTE_VERSION_")
        || feature.starts_with("VK_GRAPHICS_VERSION_")
        || feature.starts_with("VK_VERSION_")
}

fn is_vulkan_extension(feature: &str) -> bool {
    feature.starts_with("VK_") && !is_vulkan_core_feature(feature) && feature != "VKSC_VERSION_1_0"
}

fn feature_implies_derived_dependency(feature: &str, required: &str) -> bool {
    FEATURE_IMPLICATIONS
        .read()
        .expect("feature implication lock")
        .as_ref()
        .and_then(|implications| implications.get(feature))
        .is_some_and(|deps| deps.contains(required))
}

fn clauses_to_ts(clauses: &[Vec<String>]) -> TokenStream {
    match clauses.len() {
        0 => quote! { all() },
        1 => clause_to_ts(&clauses[0]),
        _ => {
            let cs: Vec<TokenStream> = clauses.iter().map(|c| clause_to_ts(c)).collect();
            quote! { any(#(#cs),*) }
        }
    }
}

fn clause_to_ts(clause: &[String]) -> TokenStream {
    match clause.len() {
        0 => quote! { all() },
        1 => {
            let f = &clause[0];
            quote! { feature = #f }
        }
        _ => {
            let items: Vec<TokenStream> = clause.iter().map(|f| quote! { feature = #f }).collect();
            quote! { all(#(#items),*) }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use super::{cfg_availability_expr, feature_implies, set_feature_implications};
    use crate::ir::Availability;

    #[test]
    fn feature_implications_are_derived_from_configured_dependencies() {
        set_feature_implications(BTreeMap::from([(
            "VK_VERSION_1_4".to_owned(),
            BTreeSet::from([
                "VK_VERSION_1_3".to_owned(),
                "VK_GRAPHICS_VERSION_1_3".to_owned(),
            ]),
        )]));

        assert!(feature_implies("VK_VERSION_1_4", "VK_GRAPHICS_VERSION_1_3"));
        assert!(feature_implies("VK_VERSION_1_4", "VK_VERSION_1_3"));
        assert!(!feature_implies(
            "VK_COMPUTE_VERSION_1_4",
            "VK_GRAPHICS_VERSION_1_3"
        ));
    }

    #[test]
    fn availability_can_be_removed_by_an_api_feature() {
        let availability = Availability {
            provider: "VK_VERSION_1_0".to_owned(),
            dep: None,
            excluded_by: vec!["VKSC_VERSION_1_0".to_owned()],
        };
        let cfg = cfg_availability_expr(&[availability], &[], None).to_string();
        assert!(cfg.contains("feature = \"VK_VERSION_1_0\""));
        assert!(cfg.contains("not (feature = \"VKSC_VERSION_1_0\")"));
    }
}
