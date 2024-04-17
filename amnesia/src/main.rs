fn main() {
    println!("Using std::mem::forget() to intentionally leak memory");

    let mut counter: u64 = 0;

    loop {
        let buffer = (0..100_000).collect::<Vec<u64>>();

        if counter % 1_000 == 0 {
            println!("counter: {}", counter);
        }

        counter += 1;

        std::mem::forget(buffer);
    }
}
