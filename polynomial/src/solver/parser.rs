use std::fmt::Display;

use regex::{Captures, Regex};

use super::{Polynomial, Term};

#[derive(Debug)]
pub enum ParserError {
    EmptyPolynomial,
    IsScalar(i32),
    InvalidTerm(String),
    MissingTerm(usize),
}

impl std::error::Error for ParserError {}
impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::EmptyPolynomial => write!(f, "Parser Error: polynomial is empty"),
            ParserError::IsScalar(err) => write!(f, "Parser Error: polynomial is scaler {}", err),
            ParserError::InvalidTerm(err) => write!(f, "Parser Error: invalid term {}", err),
            ParserError::MissingTerm(err) => {
                write!(f, "Parser Error: Missing Term in group {}", err)
            }
        }
    }
}

pub struct Parser;

impl Parser {
    pub fn parse(input: &str) -> Result<Polynomial, ParserError> {
        let captures =
            // capture groups   ( 1 )     (           2            )
            //                             (  3  ) ( 4 )   (  5  )
            Regex::new(r"\s*([-+])?\s*(([0-9]*)(\w?)\^?([0-9]*))?").expect("valid regex");

        // clean up input
        let input = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        // Parse each capture
        let terms = captures
            .captures_iter(&input)
            .enumerate()
            .map(|(index, caps)| {
                Ok(Term {
                    value: Parser::parse_value(index, &caps)?,
                    power: Parser::parse_power(&caps),
                })
            })
            .collect::<Result<Vec<Term>, ParserError>>();

        // Handle result
        match Parser::validate_polynomial(terms) {
            Ok(terms) => Ok(Polynomial(terms)),
            Err(err) => Err(err),
        }
    }

    fn parse_value(index: usize, caps: &Captures) -> Result<i32, ParserError> {
        // Grab raw term for validation
        let raw_term = match caps.get(2) {
            Some(raw_term) => raw_term.as_str().to_string(),
            None => return Err(ParserError::MissingTerm(index)),
        };

        // Parse term sign, if no sign is seen after the first term, this polynomial is invalid
        let sign = match caps.get(1) {
            Some(sign) => if sign.as_str() == "-" { -1 } else { 1 } ,
            None => if index == 0 { 1 } else { return Err(ParserError::InvalidTerm(raw_term)) }
        };

        // Parse value applying the sign
        let value = caps
            .get(3)
            .map_or(1, |c| c.as_str().parse::<i32>().unwrap_or(1))
            * sign;

        Ok(value)
    }

    fn parse_power(caps: &Captures) -> u32 {
        // Determine implicit power
        let default_power = caps
            .get(4)
            .map_or(0, |c| if c.as_str().is_empty() { 0 } else { 1 });

        // Parse explicit power
        let power = caps.get(5).map_or(default_power, |c| {
            c.as_str().parse::<u32>().unwrap_or(default_power)
        });

        power
    }

    fn validate_polynomial(
        terms: Result<Vec<Term>, ParserError>,
    ) -> Result<Vec<Term>, ParserError> {
        let terms = match terms {
            Ok(terms) => terms,
            Err(err) => Err(err)?,
        };

        match terms.len() {
            0 => Err(ParserError::EmptyPolynomial),
            1 => match terms.first() {
                Some(Term { value, power }) if *power == 0 => {
                    Err(ParserError::IsScalar(*value))
                }
                _ => Ok(terms),
            },
            _ => Ok(terms),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::{Parser, Polynomial, Term};

    #[test]
    fn parse_terms() {
        let polynomials = vec![
            ("2x^2", 1),
            ("2x^2 + 2x", 2),
            ("2x^2 + 2x - 4x^3", 3),
            ("2x^2 + 2x - 4x^3 - 123x^123", 4),
            ("2x^2 + 2x - 4x^3 - 123x^123 + 10", 5),
        ];

        polynomials.iter().for_each(|(polynomial, expected)| {
            let actual = Parser::parse(polynomial).expect("valid polynomial");
            println!("{:?}", actual);
            assert_eq!(actual.0.len(), *expected as usize);
        });
    }

    #[test]
    fn can_parse_terms_with_uneven_spacing() {
        let polynomials = vec![
            ("   2x^2    ", 1),
            ("2x^2  +       2x", 2),
            ("2x^2   +       2x-4x^3", 3),
            ("2x^2+ 2x      -4x^3-    123x^123", 4),
            ("   2x^2-2x-4x^3-123x^123+10   ", 5),
        ];

        polynomials.iter().for_each(|(polynomial, expected)| {
            let actual = Parser::parse(polynomial).expect("valid polynomial");
            println!("{:?}", actual);
            assert_eq!(actual.0.len(), *expected as usize);
        });
    }

    #[test]
    fn can_parse_terms_with_leading_sign() {
        let polynomials = vec![
            ("-x^2", 1),
            ("-x^2 + 2x", 2),
            ("-x^2 + 2x - 4x^3", 3),
            ("-2x^2 + 2x - 4x^3 - 123x^123", 4),
            ("-2x^2 + 2x - 4x^3 - 123x^123 + 10", 5),
        ];

        polynomials.iter().for_each(|(polynomial, expected)| {
            let actual = Parser::parse(polynomial).expect("valid polynomial");
            println!("{:?}", actual);
            assert_eq!(actual.0.len(), *expected as usize);
        });
    }

    #[test]
    fn parse_terms_into_polynomial() {
        let polynomials = vec![
            (
                "2x^2 - 2x - 4",
                Polynomial(vec![
                    Term { value: 2, power: 2 },
                    Term {
                        value: -2,
                        power: 1,
                    },
                    Term {
                        value: -4,
                        power: 0,
                    },
                ]),
            ),
            (
                "21x^3 - 147x + 126",
                Polynomial(vec![
                    Term {
                        value: 21,
                        power: 3,
                    },
                    Term {
                        value: -147,
                        power: 1,
                    },
                    Term {
                        value: 126,
                        power: 0,
                    },
                ]),
            ),
            (
                "-x^2 - 2",
                Polynomial(vec![
                    Term {
                        value: -1,
                        power: 2,
                    },
                    Term {
                        value: -2,
                        power: 0,
                    },
                ]),
            ),
            (
                "x^2 - 2",
                Polynomial(vec![
                    Term { value: 1, power: 2 },
                    Term {
                        value: -2,
                        power: 0,
                    },
                ]),
            ),
            (
                "-18x^3 + 4",
                Polynomial(vec![
                    Term {
                        value: -18,
                        power: 3,
                    },
                    Term { value: 4, power: 0 },
                ]),
            ),
        ];

        polynomials.iter().for_each(|(polynomial, expected)| {
            let actual = Parser::parse(polynomial).expect("valid polynomial");
            println!("{:?}", actual);
            assert_eq!(actual, *expected);
        });
    }

    #[test]
    fn will_return_invalid_term_for_malformed_polynomial() {
        let polynomials = vec![
            "",
            "1000",
            "asdasd",
            "-",
            "+",
            "2x - ^2  2x",
            "2x + 2 - 2x asd",
            "2x  / 2 - 2x",
            "2x  + 2 * 2x",
            "(2x  + 2)(2x  - 2)",
            "[[[[[[]]]]]",
        ];

        polynomials.iter().for_each(|polynomial| {
            let terms = Parser::parse(polynomial);
            match terms {
                Ok(terms) => {
                    println!("{:?}", terms);
                    panic!("should return ParserError::InvalidTerm")
                }
                Err(err) => println!("{} | {}", err, polynomial),
            }
        });
    }
}
