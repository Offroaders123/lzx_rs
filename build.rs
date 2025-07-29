use cc::Build;

fn main() {
    Build::new()
        .include("c_src")
        .file("c_src/lzx.c")
        .compile("lzx_c");

    println!("cargo:rerun-if-changed=c_src/lzx.c");
    println!("cargo:rerun-if-changed=c_src/lzx.h");
}
