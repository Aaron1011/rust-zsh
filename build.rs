extern crate bindgen;

use std::path::PathBuf;
use std::env;
use std::process::Command;


fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    compile_zsh();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .hide_type("max_align_t")
        .unstable_rust(true)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

}

fn compile_zsh() {
    let old_dir = env::current_dir().unwrap();
    let zsh_dir = old_dir.join("zsh");

    env::set_current_dir(zsh_dir.clone()).unwrap();

    Command::new(zsh_dir.join("Util/preconfig"))
        .status()
        .expect("Failed to run zsh/Util/preconfig");

    Command::new(zsh_dir.join("./configure"))
        .status()
        .expect("Failed to run zsh/configure");

    Command::new("make")
        .status()
        .expect("Failed to run make");


    env::set_current_dir(old_dir).unwrap();

}
