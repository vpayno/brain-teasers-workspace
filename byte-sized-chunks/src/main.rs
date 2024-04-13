use std::num::Wrapping;

fn main() {
    println!("Properly handling arithmetic overflow using .checked_add() ...");
    let mut counter: i8 = 1;
    while counter > 0 {
        println!("{}", counter);
        counter = counter.checked_add(8).unwrap_or(0);
    }
    println!();

    println!("Properly handling arithmetic overflow using Wrapping() ...");
    let mut wc = Wrapping(1i8);
    while wc > Wrapping(0i8) {
        println!("{}", wc);
        wc += Wrapping(8i8);
    }
    println!();

    println!("Improperly handling arithmetic overflow...");
    counter = 1;
    while counter > 0 {
        println!("{}", counter);
        counter += 8;
    }
}
