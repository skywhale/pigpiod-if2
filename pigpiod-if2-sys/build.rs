use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-lib=pigpiod_if2");

    let bindings = bindgen::builder()
        .header("src/wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings");
}
