fn main() {
    println!("cargo:rerun-if-changed=bindings/bindings.h");

    let bindings = bindgen::Builder::default()
        .header("bindings/bindings.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("failed to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings");

    cc::Build::new()
        .file("bindings/bindings.cpp")
        .include("bindings")
        .compile("basis_universal");
    println!("cargo:rustc-link-lib=static=basis_universal");
    println!("cargo:rustc-flags=-l dylib=stdc++");
}
