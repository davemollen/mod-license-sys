fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let lib_path = format!("{}/include", manifest_dir);
    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:rustc-link-lib=static=modla");
}
