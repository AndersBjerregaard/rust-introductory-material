// bring input/output library into scope
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // holding variable for user input
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

// in Rust, variables are immutable by default
fn foo() {
    let apples = 5; // immutable
    let mut bananas = 5; // mutable
}