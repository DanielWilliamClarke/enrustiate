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