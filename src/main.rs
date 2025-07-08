unsafe extern "C" {
    fn gcd(a: isize, b: isize) -> isize;
    fn scan_and_print() -> isize;
}


fn main() {
    let a;
    let b;
    let res;

    // FFI calls require an unsafe block.
    unsafe {
        a = scan_and_print();
        b = scan_and_print();
        res = gcd(a, b);
    }

    println!("The gcd of {} and {} is {}", a, b, res);

    let key = "VAR";

    match std::env::var(key) {
        Ok(val) => println!("OK! {}: {:?}", key, val),
        Err(e) => println!("Error {}: {}", key, e),
    }
}
