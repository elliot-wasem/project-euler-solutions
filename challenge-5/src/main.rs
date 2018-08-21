/*
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/

fn main() {
    let mut number: u64 = 20;
    loop {
        println!("\t\tnumber: {:?}", number);
        for i in 11..21 {
            println!("\t\t\ti: {:?}", i);
            if number % i != 0 {
                break;
            }
            if i == 20 {
                println!("result: {:?}", number);
                return ();
            }
        }
        number += 20;
    }
}
