use std::env;

use number_renderer::{NumbersToWords, Validator};
use fizz_buzz::{FizzBuzz};

fn main() {
    println!("Numbers to words");
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let input = args[1].clone();

    match NumbersToWords::<i64>::validate(input, 0, 1_000_000_000_000_000) {
        Ok(renderer) => println!("Output: {}", renderer),
        Err(err) => println!("Error: {}", err),
    }

    println!("Fizzbuzz");
    for i in 1..=100 {
        print!("{}, ", FizzBuzz::new(i))
    }
}
