#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(generic_associated_types)]

mod economy;
mod environments;
mod indicators;
mod traders;

use chrono::{DateTime, NaiveDateTime, Utc};
use economy::{Economy};
use environments::Historical;
use traders::{Action, Trader};
use indicators::{Indicator, Value, EMA, MACD, MACDHistogram};
use std::time::Duration;
use bigdecimal::{Num, Zero};

const D12M: Duration = Duration::from_secs(12);
const D26M: Duration = Duration::from_secs(26);
const D9M: Duration = Duration::from_secs(9);


struct MyTrader<N>
    where
        N: Num
{
    previous_macd_histogram: N,
    buy_margin: N,
    sell_margin: N,
}

impl<N> Trader<N> for MyTrader<N>
    where
        N: Num + From<f32> + 'static
{
    type Indicators = (Value, MACDHistogram<N, D12M, D26M, D9M>);

    fn initialize(base: &str, quote: &str) -> MyTrader<N> {
        MyTrader {
            previous_macd_histogram: N::zero(),
            buy_margin: if base == "USDT" {
                N::from(0.0001)
            } else {
                N::from(0.001)
            },
            sell_margin: if quote == "USDT" {
                N::from(0.0001)
            } else {
                N::from(0.001)
            },
        }
    }

    fn evaluate<'a>(&mut self, (value, macd_histogram): <Self::Indicators as Indicator<N>>::Output<'a>) -> Action<N> {
        let distance = (self.previous_macd_histogram - *macd_histogram) / *value;

        let output =
            if self.previous_macd_histogram < N::zero() &&
                *macd_histogram >= N::zero() &&
                distance > self.buy_margin
            {
                Action::Buy(distance, value.clone())
            } else
            if self.previous_macd_histogram > N::zero() &&
                *macd_histogram <= N::zero() &&
                distance > self.sell_margin
            {
                Action::Sell(distance, value.clone())
            } else {
                Action::Hold
            };

        self.previous_macd_histogram = *macd_histogram;

        output
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let environment = Historical::new(DateTime::from_utc(
        NaiveDateTime::from_timestamp(26518062 * 60, 0),
        Utc,
    ))
    .await;

    let mut economy = Economy::<Historical, MyTrader>::new(environment);
    economy.run().await?;

    Ok(())
}
