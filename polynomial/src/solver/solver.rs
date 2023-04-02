use super::Polynomial;

pub struct Solver;

impl Solver {   
    pub fn solve(polynomial: &Polynomial) -> Vec<i32> {
        (-10..=10)
            .filter(|&x| polynomial.has_x_intercept(x))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::{Polynomial, Term, Solver};

    #[test]
    fn solve_polynomials() {
        let polynomials = vec![
            (Polynomial(vec![
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
            ]), vec![-1, 2]),
            (Polynomial(vec![
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
            ]), vec![-3, 1, 2]),
            (Polynomial(vec![
                Term {
                    value: 4,
                    power: 3
                },
                Term {
                    value: 17,
                    power: 1
                },
                Term {
                    value: -3,
                    power: 0
                },
            ]), vec![]) // no integer solutions
        ];

        polynomials.iter().for_each(|(polynomial, expected)| {
            let intercepts = Solver::solve(polynomial);
            assert_eq!(intercepts, *expected);
        });
    }
}