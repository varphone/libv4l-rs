use std::env;
use std::path::PathBuf;

const DEFAULT_TARGET_SYSROOT_DIR: &str = "/opt/fullv/2021.02.8-rklaser1/staging";

fn main() {
    println!("cargo:rerun-if-env-changed=TARGET_SYSROOT_DIR");

    let target_sysroot_dir =
        env::var("TARGET_SYSROOT_DIR").unwrap_or_else(|_| DEFAULT_TARGET_SYSROOT_DIR.into());

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("--sysroot={}", target_sysroot_dir))
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("v4l2_bindings.rs"))
        .expect("Failed to write bindings");
}
