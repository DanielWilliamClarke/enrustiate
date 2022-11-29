use std::{fmt::Display, str::FromStr};

use crate::{validation_errors::InputError, validator::Validator};

pub struct NumberToWords<N>(N);

impl<N> NumberToWords<N> {
    pub fn new(input: N) -> Self {
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


#[cfg(test)]
mod tests {
    use super::NumberToWords;

    #[test]
    fn displays_ones() {
        let actual = NumberToWords::new(1);
        assert_eq!(format!("{actual}"), "one");   
             
        let actual = NumberToWords::new(9);
        assert_eq!(format!("{actual}"), "nine");    

        let actual = NumberToWords::new(8);
        assert_eq!(format!("{actual}"), "eight");        
    }

    #[test]
    fn displays_teens() {
        let actual = NumberToWords::new(10);
        assert_eq!(format!("{actual}"), "ten");   

        let actual = NumberToWords::new(19);
        assert_eq!(format!("{actual}"), "nineteen");    

        let actual = NumberToWords::new(17);
        assert_eq!(format!("{actual}"), "seventeen");     
    }

    #[test]
    fn displays_hundreds() {
        let actual = NumberToWords::new(100);
        assert_eq!(format!("{actual}"), "one hundred");   

        let actual = NumberToWords::new(999);
        assert_eq!(format!("{actual}"), "nine hundred and ninety nine");   

        let actual = NumberToWords::new(579);
        assert_eq!(format!("{actual}"), "five hundred and seventy nine");        

        let actual = NumberToWords::new(301);
        assert_eq!(format!("{actual}"), "three hundred and one");     
    }

    #[test]
    fn displays_thousands() {
        let actual = NumberToWords::new(1_000);
        assert_eq!(format!("{actual}"), "one thousand");   

        let actual = NumberToWords::new(9_999);
        assert_eq!(format!("{actual}"), "nine thousand nine hundred and ninety nine");   

        let actual = NumberToWords::new(9999);
        assert_eq!(format!("{actual}"), "nine thousand nine hundred and ninety nine");        

        let actual = NumberToWords::new(6047);
        assert_eq!(format!("{actual}"), "six thousand and forty seven");     
    }

    #[test]
    fn displays_ten_thousands() {
        let actual = NumberToWords::new(10_000);
        assert_eq!(format!("{actual}"), "ten thousand");   

        let actual = NumberToWords::new(99_999);
        assert_eq!(format!("{actual}"), "ninety nine thousand nine hundred and ninety nine");   

        let actual = NumberToWords::new(67232);
        assert_eq!(format!("{actual}"), "sixty seven thousand two hundred and thirty two");        

        let actual = NumberToWords::new(86047);
        assert_eq!(format!("{actual}"), "eighty six thousand and forty seven");     
    }

    #[test]
    fn displays_hundred_thousands() {
        let actual = NumberToWords::new(100_000);
        assert_eq!(format!("{actual}"), "one hundred thousand");   

        let actual = NumberToWords::new(999_999);
        assert_eq!(format!("{actual}"), "nine hundred and ninety nine thousand nine hundred and ninety nine");   

        let actual = NumberToWords::new(267232);
        assert_eq!(format!("{actual}"), "two hundred and sixty seven thousand two hundred and thirty two");        

        let actual = NumberToWords::new(900047);
        assert_eq!(format!("{actual}"), "nine hundred thousand and forty seven");     
    }

    #[test]
    fn displays_millions() {
        let actual = NumberToWords::new(1_000_000);
        assert_eq!(format!("{actual}"), "one million");   

        let actual = NumberToWords::new(9_999_999);
        assert_eq!(format!("{actual}"), "nine million nine hundred and ninety nine thousand nine hundred and ninety nine");   

        let actual = NumberToWords::new(6267232);
        assert_eq!(format!("{actual}"), "six million two hundred and sixty seven thousand two hundred and thirty two");        

        let actual = NumberToWords::new(2000590);
        assert_eq!(format!("{actual}"), "two million and five hundred and ninety");     
    }

    #[test]
    fn displays_ten_millions() {
        let actual = NumberToWords::new(10_000_000);
        assert_eq!(format!("{actual}"), "ten million");   

        let actual = NumberToWords::new(99_999_999);
        assert_eq!(format!("{actual}"), "ninety nine million nine hundred and ninety nine thousand nine hundred and ninety nine");   

        let actual = NumberToWords::new(16267232);
        assert_eq!(format!("{actual}"), "sixteen million two hundred and sixty seven thousand two hundred and thirty two");        

        let actual = NumberToWords::new(12000590);
        assert_eq!(format!("{actual}"), "twelve million and five hundred and ninety");     
    }

    #[test]
    fn displays_hundred_millions() {
        let actual = NumberToWords::new(100_000_000);
        assert_eq!(format!("{actual}"), "one hundred million");   

        let actual = NumberToWords::new(999_999_999);
        assert_eq!(format!("{actual}"), "nine hundred and ninety nine million nine hundred and ninety nine thousand nine hundred and ninety nine");   

        let actual = NumberToWords::new(416267232);
        assert_eq!(format!("{actual}"), "four hundred and sixteen million two hundred and sixty seven thousand two hundred and thirty two");        

        let actual = NumberToWords::new(112000590);
        assert_eq!(format!("{actual}"), "one hundred and twelve million and five hundred and ninety");     
    }

    #[test]
    fn displays_billions() {
        let actual = NumberToWords::new(1_000_000_000);
        assert_eq!(format!("{actual}"), "one billion");   

        let actual = NumberToWords::new(9_999_999_999);
        assert_eq!(format!("{actual}"), "nine billion nine hundred and ninety nine million nine hundred and ninety nine thousand nine hundred and ninety nine");   

        let actual = NumberToWords::new(8416267232);
        assert_eq!(format!("{actual}"), "eight billion four hundred and sixteen million two hundred and sixty seven thousand two hundred and thirty two");        

        let actual = NumberToWords::new(1000000590);
        assert_eq!(format!("{actual}"), "one billion and five hundred and ninety");     
    }

    #[test]
    fn displays_ten_billions() {
        let actual = NumberToWords::new(10_000_000_000);
        assert_eq!(format!("{actual}"), "ten billion");   

        let actual = NumberToWords::new(99_999_999_999);
        assert_eq!(format!("{actual}"), "ninety nine billion nine hundred and ninety nine million nine hundred and ninety nine thousand nine hundred and ninety nine");   

        let actual = NumberToWords::new(78416267232);
        assert_eq!(format!("{actual}"), "seventy eight billion four hundred and sixteen million two hundred and sixty seven thousand two hundred and thirty two");        

        let actual = NumberToWords::new(11000000590);
        assert_eq!(format!("{actual}"), "eleven billion and five hundred and ninety");     
    }

    #[test]
    fn displays_hundred_billions() {
        let actual = NumberToWords::new(100_000_000_000);
        assert_eq!(format!("{actual}"), "one hundred billion");   

        let actual = NumberToWords::new(999_999_999_999);
        assert_eq!(format!("{actual}"), "nine hundred and ninety nine billion nine hundred and ninety nine million nine hundred and ninety nine thousand nine hundred and ninety nine");   

        let actual = NumberToWords::new(478416267232);
        assert_eq!(format!("{actual}"), "four hundred and seventy eight billion four hundred and sixteen million two hundred and sixty seven thousand two hundred and thirty two");        

        let actual = NumberToWords::new(111000000590);
        assert_eq!(format!("{actual}"), "one hundred and eleven billion and five hundred and ninety");     
    }

    #[test]
    fn displays_trillions() {
        let actual = NumberToWords::new(1_000_000_000_000);
        assert_eq!(format!("{actual}"), "one trillion");   

        let actual = NumberToWords::new(9_999_999_999_999);
        assert_eq!(format!("{actual}"), "nine trillion nine hundred and ninety nine billion nine hundred and ninety nine million nine hundred and ninety nine thousand nine hundred and ninety nine");   

        let actual = NumberToWords::new(3478416267232);
        assert_eq!(format!("{actual}"), "three trillion four hundred and seventy eight billion four hundred and sixteen million two hundred and sixty seven thousand two hundred and thirty two");        

        let actual = NumberToWords::new(1000000000590);
        assert_eq!(format!("{actual}"), "one trillion and five hundred and ninety");     
    }

    #[test]
    fn displays_ten_trillions() {
        let actual = NumberToWords::new(10_000_000_000_000);
        assert_eq!(format!("{actual}"), "ten trillion");   

        let actual = NumberToWords::new(99_999_999_999_999);
        assert_eq!(format!("{actual}"), "ninety nine trillion nine hundred and ninety nine billion nine hundred and ninety nine million nine hundred and ninety nine thousand nine hundred and ninety nine");   

        let actual = NumberToWords::new(93478416267232);
        assert_eq!(format!("{actual}"), "ninety three trillion four hundred and seventy eight billion four hundred and sixteen million two hundred and sixty seven thousand two hundred and thirty two");        

        let actual = NumberToWords::new(10000000000590);
        assert_eq!(format!("{actual}"), "ten trillion and five hundred and ninety");     
    }

    #[test]
    fn displays_hundred_trillions() {
        let actual = NumberToWords::new(100_000_000_000_000);
        assert_eq!(format!("{actual}"), "one hundred trillion");   

        let actual = NumberToWords::new(999_999_999_999_999);
        assert_eq!(format!("{actual}"), "nine hundred and ninety nine trillion nine hundred and ninety nine billion nine hundred and ninety nine million nine hundred and ninety nine thousand nine hundred and ninety nine");   

        let actual = NumberToWords::new(293478416267232);
        assert_eq!(format!("{actual}"), "two hundred and ninety three trillion four hundred and seventy eight billion four hundred and sixteen million two hundred and sixty seven thousand two hundred and thirty two");        

        let actual = NumberToWords::new(700000000000590);
        assert_eq!(format!("{actual}"), "seven hundred trillion and five hundred and ninety");     
    }

    #[test]
    fn displays_quadrillions() {
        let actual = NumberToWords::new(1_000_000_000_000_000);
        assert_eq!(format!("{actual}"), "one quadrillion");             
    }
}