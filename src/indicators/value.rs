use super::Indicator;
use crate::economy::Monetary;

pub struct Value;

impl Indicator for Value {
    type Output<'a> = &'a Monetary;

    fn initialize(_value: &Monetary) -> Self {
        Value
    }

    fn evaluate<'a>(&mut self, value: &'a Monetary) -> Self::Output<'a> {
        value
    }
}