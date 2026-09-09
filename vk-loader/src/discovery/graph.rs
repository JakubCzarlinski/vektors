//! Meta-layer dependency validation shared by discovery and activation.

use alloc::boxed::Box;

use crate::allocation;

use super::LayerManifest;

#[derive(Clone, Copy, PartialEq, Eq)]
enum VisitState {
    Unvisited,
    Visiting,
    Valid,
    Invalid,
}

/// Marks manifests whose complete meta-layer dependency graph is present and acyclic.
pub(crate) fn valid_layer_mask(manifests: &[LayerManifest]) -> Result<Box<[bool]>, vk::VkResult> {
    fn visit(index: usize, manifests: &[LayerManifest], states: &mut [VisitState]) -> bool {
        match states[index] {
            VisitState::Visiting | VisitState::Invalid => return false,
            VisitState::Valid => return true,
            VisitState::Unvisited => {}
        }
        if vk::VK_API_VERSION_VARIANT(manifests[index].api_version) != 0
            || !manifests[index].architecture_supported
        {
            states[index] = VisitState::Invalid;
            return false;
        }
        states[index] = VisitState::Visiting;
        let valid = manifests[index].component_layers().iter().all(|name| {
            manifests
                .iter()
                .enumerate()
                .find(|(candidate_index, candidate)| {
                    candidate.name == *name && states[*candidate_index] != VisitState::Invalid
                })
                .map(|(candidate_index, _)| candidate_index)
                .is_some_and(|component| {
                    let meta = manifests[index].api_version;
                    let component_version = manifests[component].api_version;
                    // Upstream finds the first property for version checking,
                    // then resolves the last matching record for meta recursion.
                    let recursive_component = if manifests[component].is_meta_layer() {
                        manifests
                            .iter()
                            .enumerate()
                            .rfind(|(candidate_index, candidate)| {
                                candidate.name == *name
                                    && states[*candidate_index] != VisitState::Invalid
                            })
                            .map_or(component, |(candidate_index, _)| candidate_index)
                    } else {
                        component
                    };
                    vk::VK_API_VERSION_MAJOR(component_version) >= vk::VK_API_VERSION_MAJOR(meta)
                        && (vk::VK_API_VERSION_MAJOR(component_version)
                            > vk::VK_API_VERSION_MAJOR(meta)
                            || vk::VK_API_VERSION_MINOR(component_version)
                                >= vk::VK_API_VERSION_MINOR(meta))
                        && manifests[index].name != *name
                        && visit(recursive_component, manifests, states)
                })
        });
        states[index] = if valid {
            VisitState::Valid
        } else {
            VisitState::Invalid
        };
        valid
    }

    let mut stack = [VisitState::Unvisited; 32];
    let mut heap = if manifests.len() > stack.len() {
        Some(allocation::try_boxed_slice_filled(
            manifests.len(),
            VisitState::Unvisited,
        )?)
    } else {
        None
    };
    let states = match heap.as_deref_mut() {
        Some(states) => states,
        None => &mut stack[..manifests.len()],
    };
    for index in 0..manifests.len() {
        visit(index, manifests, states);
        // Upstream removes rejected entries in discovery order. A recursive
        // failure must not permanently reject a later entry: after the current
        // entry is removed, that entry may resolve a different same-name layer.
        states[index + 1..].fill(VisitState::Unvisited);
    }
    let mut valid = allocation::try_boxed_slice_filled(manifests.len(), false)?;
    for (valid, state) in valid.iter_mut().zip(states.iter()) {
        *valid = *state == VisitState::Valid;
    }
    Ok(valid)
}
