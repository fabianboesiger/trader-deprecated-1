use super::Indicator;
use crate::economy::Monetary;

pub struct Value;

impl Indicator for Value {
    type Output = Monetary;

    fn initialize(_value: Monetary) -> Self {
        Value
    }

    fn evaluate(&mut self, value: Monetary) -> Self::Output {
        value
    }
}