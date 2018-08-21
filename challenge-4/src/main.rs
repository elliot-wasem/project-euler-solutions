/*
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
Find the largest palindrome made from the product of two 3-digit numbers.
*/

fn main() {
    let mut num_vec: Vec<u64> = Vec::new();
    for i in 0..1000 {
        for j in i..1000 {
            if is_palindrome(i * j) {
                num_vec.push(i * j);
            }
        }
    }
    let mut largest_num = 0;
    for i in 0..num_vec.len() {
        let curr = num_vec.pop().unwrap();
        if curr > largest_num {
            largest_num = curr;
        }
    }
    println!("largest: {:?}", largest_num);
}

fn is_palindrome(number: u64) -> bool {
    let word = num_to_string(number).into_bytes();
    for i in 0..word.len() {
        if word[i] != word[word.len() - i - 1] {
            return false;
        }
    }
    true
}

fn num_to_string(number: u64) -> String {
    number.to_string()
}
