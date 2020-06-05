use super::{Indicator, EMA};
use std::time::Duration;
use bigdecimal::{Zero, Num};

pub struct MACD<N, const SHORT: Duration, const LONG: Duration>
    where
        N: Num
{
    macd: N,
    ema_short: EMA<N, SHORT>,
    ema_long: EMA<N, LONG>,
}

impl<N, const SHORT: Duration, const LONG: Duration> Indicator<N> for MACD<N, SHORT, LONG>
    where
        N: Num + From<f32> + 'static
{
    type Output<'a> = &'a N;

    fn initialize(value: &N) -> Self {
        MACD {
            macd: N::zero(),
            ema_short: EMA::initialize(value),
            ema_long: EMA::initialize(value),
        }
    }

    fn evaluate<'a>(&'a mut self, value: &'a N) -> Self::Output<'a> {
        let short = self.ema_short.evaluate(value);
        let long = self.ema_long.evaluate(value);
        self.macd = *short - *long;
        &self.macd
    }
}
