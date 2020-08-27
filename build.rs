use bindgen::Builder;
use std::{env, path::PathBuf};

fn main() {
    #[cfg(feature = "static")]
    {
        let dst = cmake::Config::new("source")
            .define("ICB", "ON")
            .define("BUILD_TESTING", "OFF")
            .define("CMAKE_Fortran_FLAGS", "-fPIC")
            .build();
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
    #[cfg(feature = "system")]
    {
        let library = pkg_config::probe_library("arpack").expect("Could not find arpack");
        let bindings = Builder::default()
            .header("arpack-sys.h")
            .generate()
            .expect("Could not generate bindings.");
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Could not write bindings!");
    }
}
