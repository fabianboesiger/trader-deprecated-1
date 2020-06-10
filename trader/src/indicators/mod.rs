mod ema;
mod macd;
mod macd_histogram;
mod rsi;
mod sma;
mod value;

pub use ema::EMA;
pub use macd::MACD;
pub use macd_histogram::MACDHistogram;
pub use rsi::RSI;
pub use sma::SMA;
pub use value::Value;

use crate::economy::Monetary;

pub trait Indicator {
    type Output;

    fn initialize(value: Monetary) -> Self;
    fn evaluate(&mut self, value: Monetary) -> Self::Output;
}

pub trait MovingAverage: Indicator {}

macro_rules! peel {
    ( $name:ident, $($other:ident,)* ) => (tuple! { $($other,)* })
}

macro_rules! tuple {
    () => ();
    ( $($name:ident,)+ ) => {
        impl<$($name: Indicator,)+> Indicator for ($($name,)+) {
            type Output = ($($name::Output,)+);

            fn initialize(value: Monetary) -> Self {
                ($($name::initialize(value),)+)
            }

            #[allow(non_snake_case)]
            fn evaluate(&mut self, value: Monetary) -> Self::Output {
                let ($($name,)+) = self;
                ($($name.evaluate(value),)+)
            }
        }
        peel! { $($name,)+ }
    };
}

tuple! {T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,}
