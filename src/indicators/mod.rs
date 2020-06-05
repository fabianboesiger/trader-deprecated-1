mod value;
mod ema;
mod macd;

pub use value::Value;
pub use ema::EMA;
pub use macd::MACD;

use crate::economy::Monetary;

pub trait Indicator {
    type Output<'a>;

    fn initialize(value: &Monetary) -> Self;
    fn evaluate<'a>(&'a mut self, value: &'a Monetary) -> Self::Output<'a>;
}

macro_rules! peel {
    ( $name:ident, $($other:ident,)* ) => (tuple! { $($other,)* })
}

macro_rules! tuple {
    () => ();
    ( $($name:ident,)+ ) => {
        impl<$($name: Indicator,)+> Indicator for ($($name,)+) {
            type Output<'a> = ($($name::Output<'a>,)+);

            fn initialize(value: &Monetary) -> Self {
                ($($name::initialize(value),)+)
            }
            
            #[allow(non_snake_case)]
            fn evaluate<'a>(&'a mut self, value: &'a Monetary) -> Self::Output<'a> {
                let ($($name,)+) = self;
                ($($name.evaluate(value),)+)
            }
        }
        peel! { $($name,)+ }
    };
}

tuple! {T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,}