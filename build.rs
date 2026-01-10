fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    if target_os == "linux" && target_arch == "aarch64" {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let lib_path = format!("{}/include", manifest_dir);
        println!("cargo:rustc-link-search=native={}", lib_path);
        println!("cargo:rustc-link-lib=static=modla");
    }
}
