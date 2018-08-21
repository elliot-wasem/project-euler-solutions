/*
A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
    a^2 + b^2 = c^2

For example, 32 + 42 = 9 + 16 = 25 = 52.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
*/

extern crate num;
use num::Num;

fn main() {
    for a in 0..999 {
        for b in a+1..999 {
            for c in b+1..999 {
                if a + b + c == 1000 && squared(a) + squared(b) == squared(c) {
                    println!("a: {:?} - b: {:?} - c: {:?} - product: {:?}", a, b, c, a * b * c)
                }
            }
        }
    }
}

fn squared<T: Num + Copy>(a: T) -> T { a * a }
