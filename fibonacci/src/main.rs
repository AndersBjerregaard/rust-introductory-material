fn main() {
    let fib1 = fibonacci(4, &mut []);
    let fib2 = fibonacci(6, &mut []);
    let fib3 = fibonacci(10, &mut []);
    println!("The 4th fibonacci number: {}", fib1);
    println!("The 6th fibonacci number: {}", fib2);
    println!("The 10th fibonacci number: {}", fib3);
}

fn fibonacci(n: i32, mut memo: &mut [i32]) -> i32 {
    if memo.len() == 0 {
        memo = &mut [0, 0, 0, 0];
    }
    return 0;
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

fn resize_array(arr: &mut [i32]) {
    if arr.len() == 0 {

    }
}

// const fib = (n, memo = {}) => {
//     if (n in memo) return memo[n];
//     if (n <= 2) return 1;
//     memo[n] = fib(n - 1, memo) + fib(n - 2, memo);
//     return memo[n];
// };