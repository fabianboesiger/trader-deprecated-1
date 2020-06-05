use super::{Indicator, EMA, MACD};
use std::time::Duration;
use bigdecimal::{Num, Zero};

pub struct MACDHistogram<N, const SHORT: Duration, const LONG: Duration, const PERIOD: Duration>
    where
        N: Num
{
    histogram: N,
    macd: MACD<N, SHORT, LONG>,
    signal: EMA<N, PERIOD>,
}

impl<N, const SHORT: Duration, const LONG: Duration, const PERIOD: Duration> Indicator<N> for MACDHistogram<N, SHORT, LONG, PERIOD>
    where
        N: Num + From<f32> + 'static
{
    type Output<'a> = &'a N;

    fn initialize(value: &N) -> Self {
        MACDHistogram {
            histogram: N::zero(),
            macd: MACD::initialize(value),
            signal: EMA::initialize(&N::zero()),
        }
    }

    fn evaluate<'a>(&'a mut self, value: &'a N) -> Self::Output<'a> {
        let macd = self.macd.evaluate(value);
        let signal = self.signal.evaluate(macd);
        self.histogram = *macd - *signal;
        &self.histogram
    }
}
