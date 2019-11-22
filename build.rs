use bindgen::Builder;
use cmake::Config;
use std::{env, path::PathBuf};

fn main() {
    let dst = Config::new("source").define("ICB", "ON").build();
    let bindings = Builder::default()
        .header(&format!("{}/include/arpack/arpack.h", dst.display()))
        .generate()
        .expect("Could not generate bindings.");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings!");

    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=arpack");
    println!("cargo:rustc-link-lib=gfortran");
    println!("cargo:rustc-link-lib=blas");
    println!("cargo:rustc-link-lib=lapack");
}
