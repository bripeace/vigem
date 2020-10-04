use std::path::PathBuf;

fn main() {
    let project_dir = std::env::var("OUT_DIR").unwrap();

    let project_path = PathBuf::from(&project_dir);

    cc::Build::new()
        .file("src/binds/ViGEmClient.cpp")
        .include("src/binds")
        .compile("vigemclient");

    let bindings = bindgen::Builder::default()
        .header("src/binds/ViGEmClient.cpp")
        .whitelist_function("vigem_.*")
        .whitelist_type("_XUSB.*")
        .whitelist_type("_DS$.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate().expect("failed to generate bindings");

    bindings.write_to_file(project_path.join("bindings.rs")).expect("could not write bindings");

    println!("cargo:rustc-link-lib=setupapi");
}
