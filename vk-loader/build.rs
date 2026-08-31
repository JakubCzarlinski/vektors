use std::{path::Path, process::Command};

fn git_value(repository: &Path, arguments: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repository)
        .args(arguments)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let value = String::from_utf8(output.stdout).ok()?;
    let value = value.trim();
    (!value.is_empty()).then(|| value.to_owned())
}

fn main() {
    let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR")
        .map(std::path::PathBuf::from)
        .expect("Cargo supplies CARGO_MANIFEST_DIR");
    let repository = manifest_dir.parent().unwrap_or(&manifest_dir);
    let branch = std::env::var("VK_LOADER_BUILD_GIT_BRANCH_NAME")
        .ok()
        .or_else(|| git_value(repository, &["branch", "--show-current"]))
        .unwrap_or_else(|| "unknown".to_owned());
    let revision = std::env::var("VK_LOADER_BUILD_GIT_TAG_INFO")
        .ok()
        .or_else(|| git_value(repository, &["rev-parse", "--short=7", "HEAD"]))
        .unwrap_or_else(|| "unknown".to_owned());
    println!("cargo:rustc-env=VK_LOADER_GIT_BRANCH_NAME={branch}");
    println!("cargo:rustc-env=VK_LOADER_GIT_TAG_INFO={revision}");
    println!("cargo:rerun-if-env-changed=VK_LOADER_BUILD_GIT_BRANCH_NAME");
    println!("cargo:rerun-if-env-changed=VK_LOADER_BUILD_GIT_TAG_INFO");
    println!(
        "cargo:rerun-if-changed={}",
        repository.join(".git/HEAD").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        repository.join(".git/refs/heads").join(&branch).display()
    );

    let target_arch = std::env::var_os("CARGO_CFG_TARGET_ARCH");
    let target_os = std::env::var_os("CARGO_CFG_TARGET_OS");
    let target_env = std::env::var_os("CARGO_CFG_TARGET_ENV");

    if target_os.as_deref() == Some(std::ffi::OsStr::new("linux")) {
        // Match Vulkan-Loader's ELF identity. Without a SONAME, a consumer
        // which also has a DT_NEEDED on libvulkan.so.1 may load a second loader
        // object into the process instead of reusing this one.
        println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,libvulkan.so.1");
    }

    if target_arch.as_deref() == Some(std::ffi::OsStr::new("x86"))
        && target_os.as_deref() == Some(std::ffi::OsStr::new("windows"))
        && target_env.as_deref() == Some(std::ffi::OsStr::new("gnu"))
    {
        // Vulkan's Windows ABI exports undecorated names while 32-bit GNU
        // objects use stdcall decoration. This is the same linker contract as
        // upstream Vulkan-Loader's MinGW build.
        println!("cargo:rustc-link-arg=-Wl,--enable-stdcall-fixup");
    }
}
