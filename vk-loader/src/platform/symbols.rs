//! Interposed C runtime services and function-pointer conversion.

#[cfg(all(test, unix))]
mod tests {
    use super::{SymbolCache, interposed_symbol};

    type Function = unsafe extern "C" fn() -> libc::pid_t;

    unsafe extern "C" fn missing_process() -> libc::pid_t {
        -1
    }

    #[test]
    fn missing_symbol_is_cached_even_when_a_later_fallback_is_available() {
        let cache = SymbolCache::new();
        let name = c"vk_loader_missing_symbol_allocation_test";
        assert!(cache.resolve(name, core::ptr::null_mut()).is_null());
        let mut fallback = 0_u8;
        assert!(cache.resolve(name, (&raw mut fallback).cast()).is_null());
    }

    #[test]
    fn function_resolution_and_fallback_need_no_rust_allocation() {
        for name in [c"getpid", c"vk_loader_missing_symbol_allocation_test"] {
            crate::allocation::fault::sweep_operation(|| {
                let cache = SymbolCache::new();
                // SAFETY: This local cache exclusively stores the requested
                // zero-argument pid_t function, including its typed fallback.
                let function =
                    unsafe { interposed_symbol(&cache, name, missing_process as Function) };
                // SAFETY: Both getpid and the fallback have no preconditions.
                let actual = unsafe { function() };
                let expected = if name == c"getpid" {
                    // SAFETY: getpid has no preconditions.
                    unsafe { libc::getpid() }
                } else {
                    -1
                };
                assert_eq!(actual, expected);
                vk::VkResult::SUCCESS
            });
        }
    }

    #[test]
    fn concurrent_function_resolution_publishes_one_callable_address() {
        let cache = SymbolCache::new();
        let start = std::sync::Barrier::new(4);
        std::thread::scope(|scope| {
            let handles = core::array::from_fn::<_, 4, _>(|_| {
                scope.spawn(|| {
                    start.wait();
                    // SAFETY: Every caller uses getpid's native ABI and this
                    // shared cache is dedicated to that symbol.
                    let function = unsafe {
                        interposed_symbol(&cache, c"getpid", missing_process as Function)
                    };
                    // SAFETY: Resolved getpid takes no arguments.
                    assert_eq!(unsafe { function() }, unsafe { libc::getpid() });
                    function as usize
                })
            });
            let addresses = handles.map(|handle| handle.join().unwrap());
            assert!(addresses.iter().all(|address| *address == addresses[0]));
        });
    }
}

#[cfg(unix)]
use core::{
    ffi::c_void,
    sync::atomic::{AtomicPtr, AtomicU8, Ordering},
};

union FunctionPointer<T: Copy, U: Copy> {
    source: T,
    target: U,
}

pub(super) unsafe fn reinterpret_function_pointer<T: Copy, U: Copy>(source: T) -> U {
    unsafe { FunctionPointer { source }.target }
}

// A private, address-unique object distinguishes unresolved from a cached null
// result. Its value is never read or written; no pointer sentinel is dereferenced.
#[cfg(unix)]
static UNRESOLVED: AtomicU8 = AtomicU8::new(0);

#[cfg(unix)]
struct SymbolCache(AtomicPtr<c_void>);

#[cfg(unix)]
impl SymbolCache {
    const fn new() -> Self {
        Self(AtomicPtr::new(
            core::ptr::from_ref(&UNRESOLVED).cast_mut().cast(),
        ))
    }

    fn resolve(&self, name: &core::ffi::CStr, fallback: *mut c_void) -> *mut c_void {
        let unresolved = core::ptr::from_ref(&UNRESOLVED).cast_mut().cast();
        let cached = self.0.load(Ordering::Acquire);
        if cached != unresolved {
            return cached;
        }
        // SAFETY: `RTLD_DEFAULT` searches the executable before its dependencies,
        // matching the interposition behavior used by Vulkan-Loader's test shim.
        let address = unsafe { libc::dlsym(libc::RTLD_DEFAULT, name.as_ptr()) };
        let address = if address.is_null() { fallback } else { address };
        // Concurrent first callers may resolve redundantly, but all use the first
        // published result. No native lock, waiter allocation, or TLS is needed.
        match self
            .0
            .compare_exchange(unresolved, address, Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(_) => address,
            Err(published) => published,
        }
    }
}

#[cfg(unix)]
/// Resolves a function without allocating thread-parking state on contention.
///
/// # Safety
/// T must be the native function-pointer type for name. The cache must be
/// dedicated to that symbol, and fallback must have the same ABI.
unsafe fn interposed_symbol<T: Copy>(
    cache: &SymbolCache,
    name: &core::ffi::CStr,
    fallback: T,
) -> T {
    // SAFETY: The caller supplies a native function pointer. POSIX permits
    // conversion between dlsym's pointer representation and this type.
    let fallback = unsafe { reinterpret_function_pointer(fallback) };
    let published = cache.resolve(name, fallback);
    // SAFETY: Both the resolved address and fallback have T's function ABI.
    unsafe { reinterpret_function_pointer(published) }
}

#[cfg(unix)]
pub(super) fn fopen()
-> unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> *mut libc::FILE {
    type Function =
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> *mut libc::FILE;
    static FUNCTION: SymbolCache = SymbolCache::new();
    // SAFETY: This dedicated cache and fallback describe fopen's native ABI.
    unsafe { interposed_symbol(&FUNCTION, c"fopen", libc::fopen as Function) }
}

#[cfg(unix)]
pub(super) fn access() -> unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int {
    type Function = unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int;
    static FUNCTION: SymbolCache = SymbolCache::new();
    // SAFETY: This dedicated cache and fallback describe access's native ABI.
    unsafe { interposed_symbol(&FUNCTION, c"access", libc::access as Function) }
}

#[cfg(unix)]
pub(super) fn fputs() -> unsafe extern "C" fn(*const libc::c_char, *mut libc::FILE) -> libc::c_int {
    type Function = unsafe extern "C" fn(*const libc::c_char, *mut libc::FILE) -> libc::c_int;
    static FUNCTION: SymbolCache = SymbolCache::new();
    // SAFETY: This dedicated cache and fallback describe fputs's native ABI.
    unsafe { interposed_symbol(&FUNCTION, c"fputs", libc::fputs as Function) }
}

#[cfg(unix)]
pub(super) fn stderr_stream() -> *mut libc::FILE {
    static ADDRESS: SymbolCache = SymbolCache::new();
    let address = ADDRESS.resolve(c"stderr", core::ptr::null_mut());
    if address.is_null() {
        return core::ptr::null_mut();
    }
    // SAFETY: `stderr` is an exported C-runtime `FILE *` object.
    unsafe { address.cast::<*mut libc::FILE>().read() }
}

#[cfg(unix)]
pub(super) fn opendir() -> unsafe extern "C" fn(*const libc::c_char) -> *mut libc::DIR {
    type Function = unsafe extern "C" fn(*const libc::c_char) -> *mut libc::DIR;
    static FUNCTION: SymbolCache = SymbolCache::new();
    // SAFETY: This dedicated cache and fallback describe opendir's native ABI.
    unsafe { interposed_symbol(&FUNCTION, c"opendir", libc::opendir as Function) }
}

#[cfg(unix)]
pub(super) fn readdir() -> unsafe extern "C" fn(*mut libc::DIR) -> *mut libc::dirent {
    type Function = unsafe extern "C" fn(*mut libc::DIR) -> *mut libc::dirent;
    static FUNCTION: SymbolCache = SymbolCache::new();
    // SAFETY: This dedicated cache and fallback describe readdir's native ABI.
    unsafe { interposed_symbol(&FUNCTION, c"readdir", libc::readdir as Function) }
}

#[cfg(unix)]
pub(super) fn closedir() -> unsafe extern "C" fn(*mut libc::DIR) -> libc::c_int {
    type Function = unsafe extern "C" fn(*mut libc::DIR) -> libc::c_int;
    static FUNCTION: SymbolCache = SymbolCache::new();
    // SAFETY: This dedicated cache and fallback describe closedir's native ABI.
    unsafe { interposed_symbol(&FUNCTION, c"closedir", libc::closedir as Function) }
}
