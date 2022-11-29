use std::{fmt::{Display, Debug}, error::Error};

#[derive(Debug)]
pub enum InputError<N: Display> {
    ParseError(String),
    ValidationError(N)
}

impl<N> Error for InputError<N>
where 
    N: Display + Debug 
{}

impl<N: Display> Display for InputError<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::<N>::ParseError(e) => {
                write!(f, "{} cannot be parsed", e)
            },
            InputError::<N>::ValidationError(e) => {
                write!(f, "{} is not a valid number", e)
            },
        }
    }
}
