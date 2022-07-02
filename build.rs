extern crate cc;

use cc::Build;

fn main() {
    Build::new()
        .file("hello.c")
        .compile("libhello.a");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=hello.c");
}
