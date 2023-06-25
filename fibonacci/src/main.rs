fn main() {
    let fib1 = fibonacci(2);
    let fib2 = fibonacci(4);
    let fib3 = fibonacci(5);
    println!("The 2nd fibonacci number: {}", fib1);
    println!("The 4th fibonacci number: {}", fib2);
    println!("The 5th fibonacci number: {}", fib3);
}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 { return 1; }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

fn array_contains_element(arr: &mut [i32], x: i32) -> bool {
    let mut i: usize = 0;
    let arr_len: usize = arr.len();
    while i != arr_len - 1 {
        let element: i32 = arr[i];
        if element == x {
            return true;
        }
        i += 1;
    }
    return false;
}