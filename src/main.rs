fn main() {
    #[cfg(feature = "print-42")]
    println!("42");

    #[cfg(not(feature = "print-42"))]
    println!("Hello, world!");
}
