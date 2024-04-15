fn main() {
    // behavior between debug and release is different
    // use a vector instead
    let c = Box::new([0u32; 10_000_000]);

    println!();
    println!("Box Length: {}", c.len());
}
