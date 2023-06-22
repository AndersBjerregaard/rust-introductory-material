use std::io;

fn main() {
    println!("Please input a number to convert between fahrenheit and celsius: ");
    
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let celsius = (input - 32) * 5 / 9;
    let fahrenheit = input * 9 / 5 + 32;
    println!("The input if converted from fahrenheit to celsius is: {celsius}");
    println!("The input if converted from celsius to fahrenheit is: {fahrenheit}");
}
