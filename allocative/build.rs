fn main() {
    rust_nightly();
}

fn rust_nightly() {
    let rustc = std::env::var("RUSTC").unwrap();

    let nightly = version_is_nightly(&rustc) || sysroot_is_meta_internal_rustc(&rustc);
    if nightly {
        println!("cargo:rustc-cfg=rust_nightly");
    }
}

fn version_is_nightly(rustc: &str) -> bool {
    let version = std::process::Command::new(rustc)
        .arg("--version")
        .output()
        .unwrap();

    assert!(version.status.success());

    // Nightly output:
    // rustc 1.64.0-nightly (affe0d3a0 2022-08-05)
    // Stable output:
    // rustc 1.64.0 (a55dd71d5 2022-09-19)

    let stdout = String::from_utf8(version.stdout).unwrap();
    assert!(stdout.contains("rustc"), "Sanity check");
    stdout.contains("nightly")
}

/// Meta only.
///
/// Check for fbcode platform based rustc. These won't have nightly in their version string, but
/// have nightly features enabled anyway.
///
/// The path to RUSTC will be set to just "rustc" when using rustup. We can use the location of
/// sysroot to check if it points at a platform.
fn sysroot_is_meta_internal_rustc(rustc: &str) -> bool {
    let sysroot = std::process::Command::new(rustc)
        .arg("--print")
        .arg("sysroot")
        .output()
        .unwrap();

    assert!(sysroot.status.success());

    // Fbcode platform output:
    // /mnt/gvfs/third-party2/rust/60b8aa4c3d91d38d16a232de8079c2e75e4e6304/1.64.0/platform010/4eb61b6
    // Stable output:
    // /home/asm/.rustup/toolchains/stable-x86_64-unknown-linux-gnu

    let stdout = String::from_utf8(sysroot.stdout).unwrap();
    stdout.contains("platform010")
}
