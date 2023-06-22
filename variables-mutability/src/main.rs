use std::io;

fn main() {
    // Array
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    let tuple_element = tup.1;

    println!("The value of the element at index {index} is:  {element}");
    println!("The value of the element at index 1 in the tuple is: {tuple_element}");

    let y = plus_one(5);
    println!("The function plus_one() returned: {y}");

    // Conditional assignment
    let condition: bool = true;
    let z: u8 = if condition { 5 } else { 6 };

    println!("The variable z was assigned: {z}")
}

// Expressions don't end with a semicolon
fn plus_one(x: i32) -> i32 {
    x + 1
}