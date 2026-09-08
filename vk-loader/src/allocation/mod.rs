//! Fallible allocation and Vulkan allocator ownership.

#[cfg(test)]
pub(crate) mod fault;
mod path;
mod storage;

pub(crate) use path::try_join_path;

pub(crate) use storage::{
    CStringError, LOADER_ALIGNMENT, LoaderAllocation, LoaderArray, LoaderBox, try_box, try_box_str,
    try_box_uninit, try_box_uninit_slice, try_boxed_slice_filled, try_c_string, try_c_string_bytes,
    try_collect, try_collect_results, try_into_boxed_slice, try_os_string, try_path, try_push,
    try_string,
};
