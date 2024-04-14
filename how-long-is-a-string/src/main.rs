fn main() {
    let words: [&str; 6] = [
        "Hello world!",
        "abc",
        "12345",
        "｡◕‿◕｡",
        "ᕙ༼◕ ᴥ ◕༽ᕗ",
        "Héllö Wórld!",
    ];

    println!();
    println!(
        "{:>15} | {:>6} | {:>6} | {:>6}",
        "string", "length", "bytes", "chars"
    );
    println!("------------------------------------------");

    for word in words {
        println!(
            "{:>15} | {:>6} | {:>6} | {:>6}",
            word,
            word.len(),
            word.bytes().len(),
            word.chars().count()
        );
    }
}
