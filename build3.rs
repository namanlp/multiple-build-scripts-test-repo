fn main() {
    cc::Build::new()
        .files(&["src/scanning_and_printing_copy.c"])
        .compile("c_lib");
    cc::Build::new()
        .cpp(true)
        .files(&["src/my_gcd.cpp"])
        .compile("cpp_lib");
    println!("cargo::rustc-env=VAR=ThisIsVar2");
}
