use std::{fmt::Display};

pub struct FizzBuzzNumber<N>(N);

impl<N> FizzBuzzNumber<N> {
    pub fn new(number: N) -> Self {
        Self(number)
    }
}

impl Display for FizzBuzzNumber<i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.0 % 3, self.0 % 5) {
            (0, 0) => write!(f, "FizzBuzz"),
            (0, _) => write!(f, "Fizz"),
            (_, 0) => write!(f, "Buzz"),
            (_, _) => write!(f, "{}", self.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FizzBuzzNumber;

    #[test]
    fn displays_fizzbuzz() {
        assert_eq!("FizzBuzz", format!("{}", FizzBuzzNumber::new(15)));
        assert_eq!("FizzBuzz", format!("{}", FizzBuzzNumber::new(30)));
        assert_eq!("FizzBuzz", format!("{}", FizzBuzzNumber::new(45)));
    }

    #[test]
    fn displays_fizz() {
        assert_eq!("Fizz", format!("{}", FizzBuzzNumber::new(3)));
        assert_eq!("Fizz", format!("{}", FizzBuzzNumber::new(6)));
        assert_eq!("Fizz", format!("{}", FizzBuzzNumber::new(9)));
    }

    #[test]
    fn displays_buzz() {
        assert_eq!("Buzz", format!("{}", FizzBuzzNumber::new(5)));
        assert_eq!("Buzz", format!("{}", FizzBuzzNumber::new(10)));
        assert_eq!("Buzz", format!("{}", FizzBuzzNumber::new(20)));
    }

    #[test]
    fn displays_number() {
        assert_eq!("2", format!("{}", FizzBuzzNumber::new(2)));
        assert_eq!("78", format!("{}", FizzBuzzNumber::new(78)));
        assert_eq!("43", format!("{}", FizzBuzzNumber::new(43)));
    }
}
