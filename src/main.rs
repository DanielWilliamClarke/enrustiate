use std::env;

use numbers_to_words::{NumberToWords, Validator};

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let input = args[1].clone();

    match NumberToWords::<i64>::validate(input, 0, 1_000_000_000_000_000) {
        Ok(renderer) => print!("Output: {}", renderer),
        Err(err) => println!("Error: {}", err),
    }
}
