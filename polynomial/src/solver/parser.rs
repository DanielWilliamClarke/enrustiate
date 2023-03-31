use std::{fmt::Display};

use regex::Regex;

use super::{Polynomial, Term};

#[derive(Debug)]
pub enum ParserError {
    InvalidTerm(String),
    MissingTerm(usize),
}

impl std::error::Error for ParserError {}
impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::InvalidTerm(err) => write!(f, "Parser Error: invalid term {}", err),
            ParserError::MissingTerm(err) => write!(f, "Parser Error: Missing Term in group {}", err),
        }
    }
}

pub struct Parser;

impl Parser {
    fn parse(input: &str) -> Result<Polynomial, ParserError> {
        // them capture groups yo
        let captures =
            Regex::new(r"\s*([-+])?\s*(([0-9]*)(\w?)\^?([0-9]*))?").expect("valid regex");

        let terms = captures
            .captures_iter(input)
            .enumerate()
            .map(|(index, caps)| {
                // grab raw term for validation
                let raw_term = caps.get(2);
                let raw_term = match raw_term {
                    Some(raw_term) => raw_term.as_str().to_string(),
                    None => return Err(ParserError::MissingTerm(index))
                };

                // Parse term sign, if no sign is seen after the first term, this polynomial is invalid
                let sign = match caps.get(1) {
                    Some(sign) => sign.as_str(),
                    None => if index == 0 { "+" } else { return Err(ParserError::InvalidTerm(raw_term)) },
                };

                // parse value, applying sign
                let value = match caps.get(3) {
                    Some(value) => format!("{}{}", sign, value.as_str()).parse::<i32>().unwrap(),
                    None => 1,
                };

                // Determine implicit power
                let default_power = caps
                    .get(4)
                    .map_or(0, |c| if c.as_str().len() == 0 { 0 } else { 1 } );

                // parse explicit power
                let power = match caps.get(5) {
                    Some(p) => {
                        match p.as_str().parse::<u32>() {
                            Ok(p) => p,
                            Err(_) => default_power,
                        }
                    },
                    None => return Err(ParserError::InvalidTerm(raw_term))
                };

                Ok(Term {
                    value, power
                })
            })
            .collect::<Result<Vec<Term>, ParserError>>();

        match terms {
            Ok(terms) => Ok(Polynomial(terms)),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::{Parser, Polynomial, Term};

    #[test]
    fn parse_terms() {
        let terms = Parser::parse("2x^2 + 2x - 4x^3 - 123x^123 + 10").expect("valid polynomial");

        println!("{:?}", terms);

        assert_eq!(terms.0.len(), 5);
    }

    #[test]
    fn can_parse_terms_with_uneven_spacing() {
        let terms = Parser::parse("2x^2   -2x  -  4x^3 -     123x^123+10").expect("valid polynomial");

        println!("{:?}", terms);

        assert_eq!(terms.0.len(), 5);
    }

    #[test]
    fn will_return_invalid_term_for_malformed_polynomial() {
        let terms = Parser::parse("2x  ^2   -2x  -  4x^3 -     123x    ^123+10");

        match terms {
            Ok(_) => panic!("should return ParserError::InvalidTerm"),
            Err(err) => println!("{}", err),
        }
    }

    #[test]
    fn parse_terms_into_polynomial () {
        let terms = Parser::parse("2x^2 - 2x - 4").expect("valid polynomial");

        assert_eq!(terms, Polynomial(vec![
            Term {
                value: 2,
                power: 2
            },
            Term {
                value: -2,
                power: 1
            },
            Term {
                value: -4,
                power: 0
            },
        ]));

        let terms = Parser::parse("21x^3 - 147x + 126").expect("valid polynomial");

        assert_eq!(terms, Polynomial(vec![
            Term {
                value: 21,
                power: 3
            },
            Term {
                value: -147,
                power: 1
            },
            Term {
                value: 126,
                power: 0
            },
        ]));
    }
}
