use std::{fmt::Display};

pub struct FizzBuzz<N>(N);

impl<N> FizzBuzz<N> {
    pub fn new(number: N) -> Self {
        Self(number)
    }
}

impl Display for FizzBuzz<i32> {
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
    use super::FizzBuzz;

    #[test]
    fn displays_fizzbuzz() {
        assert_eq!("FizzBuzz", format!("{}", FizzBuzz::new(15)));
        assert_eq!("FizzBuzz", format!("{}", FizzBuzz::new(30)));
        assert_eq!("FizzBuzz", format!("{}", FizzBuzz::new(45)));
    }

    #[test]
    fn displays_fizz() {
        assert_eq!("Fizz", format!("{}", FizzBuzz::new(3)));
        assert_eq!("Fizz", format!("{}", FizzBuzz::new(6)));
        assert_eq!("Fizz", format!("{}", FizzBuzz::new(9)));
    }

    #[test]
    fn displays_buzz() {
        assert_eq!("Buzz", format!("{}", FizzBuzz::new(5)));
        assert_eq!("Buzz", format!("{}", FizzBuzz::new(10)));
        assert_eq!("Buzz", format!("{}", FizzBuzz::new(20)));
    }

    #[test]
    fn displays_number() {
        assert_eq!("2", format!("{}", FizzBuzz::new(2)));
        assert_eq!("79", format!("{}", FizzBuzz::new(79)));
        assert_eq!("43", format!("{}", FizzBuzz::new(43)));
    }
}
