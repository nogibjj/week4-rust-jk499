use std::io;

fn factorial(n: u64) -> u64 {
    if n == 0 {
        //base case
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    println!("Enter number: ");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Error");
    let n = num
        .trim()
        .parse::<u64>()
        .expect("Invalid number. Please enter again.");
    println!("Factorial of {} is {}", n, factorial(n));
}
