use std::{fmt::Display, str::FromStr};

use crate::{validation_errors::InputError, validator::Validator};

pub struct NumberToWords<N>(N);

impl<N> NumberToWords<N> {
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

    fn validate(
        input: String,
        low: Self::Bounds,
        high: Self::Bounds,
    ) -> Result<Self::Output, Self::Error> {
        match input.parse::<N>() {
            Err(_) => Err(InputError::ParseError(input.clone())),
            Ok(val) if val < low || val > high => Err(InputError::ValidationError(val)),
            Ok(val) => Ok(NumberToWords::new(val)),
        }
    }
}

impl NumberToWords<i64> {
    fn render(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        mod_val: i64,
        delim_val: i64,
        mag: &str,
    ) -> std::fmt::Result {
        let mod_mag = self.0 % mod_val;
        if mod_mag == 0 {
            return write!(f, "{} {}", NumberToWords::new(self.0 / mod_val), mag);
        }

        let mut delim = "";
        if mod_mag < delim_val {
            delim = "and ";
        }

        write!(
            f,
            "{} {}{}",
            NumberToWords::new(self.0 - mod_mag),
            delim,
            NumberToWords::new(mod_mag)
        )
    }
}

impl Display for NumberToWords<i64> {
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
                write!(
                    f,
                    "{} {}",
                    NumberToWords::new(self.0 - mod_ten),
                    NumberToWords::new(mod_ten)
                )
            }
            100..=999 => {
                let mod_hundred = self.0 % 100;
                if mod_hundred == 0 {
                    return write!(f, "{} hundred", NumberToWords::new(self.0 / 100));
                }

                write!(
                    f,
                    "{} and {}",
                    NumberToWords::new(self.0 - mod_hundred),
                    NumberToWords::new(mod_hundred)
                )
            }
            1000..=9_99_999 => {
                self.render(f, 1000, 100, "thousand")
            }
            1_000_000..=999_999_999 => {
                self.render(f, 1_000_000, 1000, "million")
            }
            1_000_000_000..=999_999_999_999 => {
                self.render(f, 1_000_000_000, 1_000_000, "billion")
            }
            1_000_000_000_000..=999_999_999_999_999 => {
                self.render(f, 1_000_000_000_000, 1_000_000_000, "trillion")
            }
            1_000_000_000_000_000 => {
                self.render(f, 1_000_000_000_000_000, 1_000_000_000_000, "quadrillion")
            }
            _ => write!(f, "Invalid number"),
        }
    }
}
