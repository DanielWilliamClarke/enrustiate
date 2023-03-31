use super::Polynomial;

pub struct Solver;

impl Solver {   
    fn solve(polynomial: &Polynomial) -> Vec<i32> {
        (-10..10)
            .filter(|&x| polynomial.has_x_intercept(x))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::{Polynomial, Term, Solver};

    #[test]
    fn parse_terms_case_1() {
        let intercepts = Solver::solve(&Polynomial(vec![
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

        assert_eq!(intercepts, vec![-1, 2]);
    }

    #[test]
    fn parse_terms_case_2() {
        let intercepts = Solver::solve(&Polynomial(vec![
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

        assert_eq!(intercepts, vec![-3, 1, 2]);
    }
}