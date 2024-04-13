use std::io::stdin;

fn main() {
    println!();
    println!("Type your answer and press enter.");
    println!("What is 3+2?");

    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Unable to read standard input");

    println!();
    println!("Response: {:#?}", input);
    println!("Trimmed: {:#?}", input.trim());
    println!();

    if input.trim() == "5" {
        println!("Correct!");
    } else {
        println!("Incorrect!");
    }
}
