use crate::economy::Monetary;
use crate::environments::Environment;
use crate::indicators::Indicator;

pub enum Action {
    Buy(Monetary, Monetary),
    Sell(Monetary, Monetary),
    Hold,
}

pub trait Trader
where
    Self::Subscriptions: Indicator
{
    type Subscriptions; 

    fn evaluate(subscriptions: Self::Subscriptions) -> Action;
}
