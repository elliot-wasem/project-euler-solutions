/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
What is the 10,001st prime number?
*/

fn main() {
    let mut vec_primes: Vec<u64> = Vec::new();
    let mut cur_num: u64 = 2;
    while vec_primes.len() < 10001 {
        if is_prime(cur_num) {
            vec_primes.push(cur_num);
        }
        cur_num += 1;
    }
    println!("10,001st prime: {:?}", vec_primes.pop());
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
