use std::{fmt::{Display}, str::FromStr};

use crate::{validator::Validator, validation_errors::InputError};

pub struct NumberToWords<N>(N);

impl<N> NumberToWords<N>  {
    fn new(input: N) -> Self {
        NumberToWords(input)
    }
}

impl<N> Validator for NumberToWords<N>
where
    N: Display + FromStr + PartialOrd,
{
    type Bounds = N;
    type Output = NumberToWords<N>;
    type Error = InputError<N>;

    fn validate(input: String, low: Self::Bounds, high: Self::Bounds) -> Result<Self::Output, Self::Error> {
        let parsed_input = match input.parse::<N>() {
            Ok(input) if input < low || input > high => Err(input),
            Ok(input) => Ok(input),
            Err(_) => Err(InputError::ParseError(input.clone()))?,
        };
    
        let valid_input = match parsed_input {
            Ok(input) => input,
            Err(err) => Err(InputError::ValidationError(err))?,
        };
    
       Ok(NumberToWords::new(valid_input))
    }
}

impl Display for NumberToWords<i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "zero"),
            1 => write!(f, "one"),
            2 => write!(f, "two"),
            3 => write!(f, "three"),
            4 => write!(f, "four"),
            5 => write!(f, "five"),
            6 => write!(f, "six"),
            7 => write!(f, "seven"),
            8 => write!(f, "eight"),
            9 => write!(f, "nine"),
            10 => write!(f, "ten"),
            11 => write!(f, "eleven"),
            12 => write!(f, "twelve"),
            13 => write!(f, "thirteen"),
            14 => write!(f, "fourteen"),
            15 => write!(f, "fifteen"),
            16 => write!(f, "sixteen"),
            17 => write!(f, "seventeen"),
            18 => write!(f, "eighteen"),
            19 => write!(f, "nineteen"),
            20 => write!(f, "twenty"),
            30 => write!(f, "thirty"),
            40 => write!(f, "forty"),
            50 => write!(f, "fifty"),
            60 => write!(f, "sixty"),
            70 => write!(f, "seventy"),
            80 => write!(f, "eighty"),
            90 => write!(f, "ninety"),
            21..=99 => {
                let mod_ten = self.0 % 10;
                write!(f, "{} {}",
                    NumberToWords::new(self.0 - mod_ten),
                    NumberToWords::new(mod_ten)
                )
            },
            100..=999 => {
                let mod_hundred = self.0 % 100;

                if mod_hundred == 0 {
                    return write!(f, "{} hundred", NumberToWords::new(self.0 / 100))
                }
                
                write!(f, "{} and {}",
                    NumberToWords::new(self.0 - mod_hundred),
                    NumberToWords::new(mod_hundred)
                )
            },
            1000..=9_99_999 => {
                let mod_thousand = self.0 % 1000;
                if mod_thousand == 0 {
                    return write!(f, "{} thousand", NumberToWords::new(self.0 / 1000))
                }
                
                let mut delim = ""; 
                if mod_thousand < 100 {
                    delim = "and ";
                }
                
                write!(f, "{} {}{}",
                    NumberToWords::new(self.0 - mod_thousand),
                    delim,
                    NumberToWords::new(mod_thousand)
                )
            },
            1_000_000 =>  write!(f, "{} million", NumberToWords::new(self.0 / 1_000_000)),
            _ => write!(f, "Unknown number")
        }
    }
}

