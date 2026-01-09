use clap::{arg, command, value_parser};
use std::path::PathBuf;

fn main() {
    let matches = command!()
        .arg(arg!([name]))
        .arg(
            arg!(-o --out <FILE> "Set output file path")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let out_path = matches.get_one::<PathBuf>("out").unwrap();
    let bindings = bindgen::Builder::default()
        .header("include/libmodla-clean.h")
        .clang_arg("-Iinclude")
        .allowlist_type("MOD.*")
        .allowlist_type("_MOD.*")
        .allowlist_function("mod_license_.*")
        .layout_tests(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
