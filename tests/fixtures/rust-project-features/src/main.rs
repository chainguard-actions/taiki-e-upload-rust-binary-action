fn main() {
    #[cfg(feature = "greeting")]
    println!("Hello with greeting feature!");
    #[cfg(not(feature = "greeting"))]
    println!("Hello, world!");
}
