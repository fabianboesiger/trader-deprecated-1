use super::Indicator;
use bigdecimal::Num;

pub struct Value;

impl<N> Indicator<N> for Value
    where
        N: Num + 'static
{
    type Output<'a> = &'a N;

    fn initialize(_value: &N) -> Self {
        Value
    }

    fn evaluate<'a>(&mut self, value: &'a N) -> Self::Output<'a> {
        value
    }
}