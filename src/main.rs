use std::io;

fn main() {
    println!("Enter number: ");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Error");
    let n = num
        .trim()
        .parse::<u64>()
        .expect("Invalid number. Please enter again.");

    let fact = factorial(n);

    println!("Factorial of {} is {}", n, fact);
}

fn factorial(n: u64) -> u64 {
    //base case
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
