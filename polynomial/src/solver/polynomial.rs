#[derive(PartialEq, Debug)]
pub struct Term {
    pub value: i32,
    pub power: u32,
}

#[derive(PartialEq, Debug)]
pub struct Polynomial(pub Vec<Term>);

impl Polynomial {
    pub fn has_x_intercept (&self, x: i32) -> bool {
        self.0.iter().fold(0, |acc, &Term{ value, power }| {
            acc + value * x.pow(power)
        }) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::{Polynomial, Term};

    #[test]
    fn has_x_intercept() {
        let p = Polynomial(vec![
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
        ]);

        assert_eq!(p.has_x_intercept(-1), true);
        assert_eq!(p.has_x_intercept(5), false);
    }
}