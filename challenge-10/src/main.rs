/*
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
*/
fn main() {
    let mut sum_primes = 0;
    for i in 1..2000001 {
        if is_prime(i) {
            sum_primes += i;
        }
    }
    println!("sum of primes: {:?}", sum_primes);
}

fn is_prime(num: u64) -> bool {
    if num == 2 || num == 3 || num == 5 || num == 7 {
        return true;
    }
    if ((num as f64).sqrt() as u64) <= 2 {
        return false;
    }
    for i in 2..((num as f64).sqrt() as u64 + 1) {
        if num % i == 0 {
            return false;
        }
    }
    true
}
