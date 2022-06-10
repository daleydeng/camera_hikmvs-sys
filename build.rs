// we need 2.17 conda sysroot due to libMvCameraControl need 2.15 glibc

extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let prefix =
        PathBuf::from(env::var("PREFIX").unwrap_or_else(|_| env::var("CONDA_PREFIX").unwrap()));

    match env::consts::OS {
        "windows" => {
            println!(
                "cargo:rustc-link-search={}",
                prefix.join("bin").to_str().unwrap()
            );
        },
        _ => {}
    }

    let incdir = prefix.join("include");
    let incdir = incdir.to_str().unwrap();

    println!("cargo:rustc-link-lib=MvCameraControl");

    let bindings = bindgen::builder()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", incdir))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
