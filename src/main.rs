mod numbers_to_words;
mod validation_errors;
mod validator;

use std::env;
use crate::{numbers_to_words::NumberToWords, validator::Validator};

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let input = args[1].clone();
    // let input = "12055".to_string();

    let validation = NumberToWords::validate(input, 0, 1_000_000);

    match validation {
        Ok(mapper) => {
            print!("{}", mapper);
        }
        Err(err) => println!("{}", err),
    }
}
