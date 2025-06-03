fn main() {
    println!("cargo:rerun-if-changed=src/my_gcd.cpp");
    println!("cargo:rerun-if-changed=src/scanning_and_printing.c");

    cc::Build::new()
        .cpp(true)
        .files(&["src/my_gcd.cpp"])
        .compile("cpp_lib");

    cc::Build::new()
        .files(&["src/scanning_and_printing.c"])
        .compile("c_lib");
}
