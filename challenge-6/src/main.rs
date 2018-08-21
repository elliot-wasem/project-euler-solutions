/*
The sum of the squares of the first ten natural numbers is,

    1^2 + 2^2 + ... + 10^2 = 385

The square of the sum of the first ten natural numbers is,

    (1 + 2 + ... + 10)^2 = 552 = 3025

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
*/

fn main() {
    let sum_squares: u64 = sum_squares();
    let square_sums: u64 = square_sums();
    println!("answer: {:?}", square_sums - sum_squares);
}

fn sum_squares() -> u64 {
    let mut return_val = 1;
    for i in 2..101 {
        return_val += i * i;
    }
    return_val
}

fn square_sums() -> u64 {
    let mut return_val = 0;
    for i in 1..101 {
        return_val += i;
    }
    return_val * return_val
}
