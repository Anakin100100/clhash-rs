extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    //cc -fPIC -std=c99 -O3 -msse4.2 -mpclmul -march=native -funroll-loops -Wstrict-overflow -Wstrict-aliasing
    //-Wall -Wextra -pedantic -Wshadow -c clhash.c -Iinclude
    cc::Build::new()
        .file("clhash/clhash.c")
        .flag("-fPIC")
        .flag("-std=c99")
        .flag("-O3")
        .flag("-msse4.2")
        .flag("-mpclmul")
        .flag("-march=native")
        .flag("-funroll-loops")
        .flag("-Wstrict-overflow")
        .flag("-Wstrict-aliasing")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-pedantic")
        .flag("-Wshadow")
        .compile("clhash");

    println!("cargo:rerun-if-changed=clhash/clhash.h");

    let bindings = bindgen::Builder::default()
        .header("clhash/clhash.c")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
