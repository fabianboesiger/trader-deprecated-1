use crate::environments::Environment;
use crate::indicators::Indicator;
use bigdecimal::Num;

#[derive(Debug)]
pub enum Action<N>
    where
        N: Num
{
    Buy(N, N),
    Sell(N, N),
    Hold,
}

pub trait Trader<N>
where
    N: Num,
    Self::Indicators: Indicator<N>
{
    type Indicators; 

    fn initialize(base: &str, quote: &str) -> Self;

    fn evaluate<'a>(&mut self, subscriptions: <Self::Indicators as Indicator<N>>::Output<'a>) -> Action<N>;
}
