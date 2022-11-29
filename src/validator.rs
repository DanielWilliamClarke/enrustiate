pub trait Validator {
    type Bounds;
    type Output;
    type Error;

    fn validate(input: String, low: Self::Bounds, high: Self::Bounds) -> Result<Self::Output, Self::Error>;
}