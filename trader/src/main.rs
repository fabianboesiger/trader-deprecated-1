#![allow(incomplete_features)]
#![feature(const_generics)]

mod economy;
mod environments;
mod indicators;
mod traders;

use economy::{Economy, Monetary};
use environments::Simulated;
use traders::{Trader, RSITrader, MACDTrader, Order, Action, Backoff, StopLoss, FullStop, GobbleBadLongterm, And, SafeAnd};
use indicators::{Indicator, Value, StretchedRSI, SMA, EMA, MACD};

/*
pub struct CustomTrader {
    previous_rsi: Monetary,
}

impl Trader for CustomTrader {
    type Indicators = (Value, StretchedRSI<SMA<4200>, EMA<300>>, MACD<43200, 93600>);

    fn initialize(base: &str, quote: &str) -> CustomTrader {
        CustomTrader {
            previous_rsi: 50.0
        }
    }

    fn evaluate(&mut self, (value, rsi, macd): <Self::Indicators as Indicator>::Output) -> Option<Order> {
        if let (Some(rsi), Some(macd)) = (rsi, macd) {
            let action = if
                self.previous_rsi < 30.0 &&
                rsi >= 30.0 &&
                macd < 0.0
            {
                Some(Order::Limit(Action::Buy, 0.05, value))
            } else if
                self.previous_rsi > 70.0 &&
                rsi <= 70.0 &&
                macd > 0.0
            {
                Some(Order::Limit(Action::Sell, 0.05, value))
            } else {
                None
            };

            self.previous_rsi = rsi;

            action
        } else {
            None
        }
    }
}
*/

//type MyTrader = StopLoss<FullStop<GobbleBadLongterm<Backoff<RSITrader<4200, 300, 30.0, 70.0, 0.05>, 300>, "USDT">, "USDT">, "USDT", 0.98, 1200>;
//type MyTrader = Backoff<RSITrader<4200, 300, 30.0, 70.0, 0.05>, 300>;
type MyTrader = StopLoss<Backoff<RSITrader<4200, 300, 30.0, 70.0, 0.05>, 300>, "USDT", 0.95, 300>;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let environment = Simulated::new().await;
    let mut economy = Economy::<_, MyTrader>::new(environment);
    economy.run().await?;

    Ok(())
}
