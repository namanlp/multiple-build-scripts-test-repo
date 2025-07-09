use std::env;
use std::fs;
use std::path::Path;

fn main(){
    cc::Build::new()
        .files(&["src/scanning_and_printing.c"])
        .compile("c_lib");
    cc::Build::new()
        .cpp(true)
        .files(&["src/my_gcd.cpp"])
        .compile("cpp_lib");
    println!("cargo::rustc-env=VAR=ThisIsVar1");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello1.rs");
    fs::write(
        &dest_path,
        "pub fn message1() -> &'static str {
            \"Hello, World1234!\"
        }
        "
    ).unwrap();
}