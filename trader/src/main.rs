#![allow(incomplete_features)]
#![feature(const_generics)]

mod economy;
mod environments;
mod indicators;
mod traders;

use economy::{Economy, Monetary};
use environments::Simulated;
use traders::{Trader, RSITrader, MACDTrader, Order, Action, Backoff, StopLoss, FullStop, GobbleBadLongterm, And, SafeAnd};
use indicators::{Indicator, Value, StretchedRSI, SMA, SMMA, EMA, MACD, MACDHistogram};


pub struct CustomTrader {
    previous_rsi: Monetary,
}

impl Trader for CustomTrader {
    type Indicators = (Value, StretchedRSI<SMMA<4200>, SMMA<300>>, MACDHistogram<21600, 46800, 16200>);

    fn initialize(base: &str, quote: &str) -> CustomTrader {
        CustomTrader {
            previous_rsi: 50.0
        }
    }

    fn evaluate(&mut self, (value, rsi, macd): <Self::Indicators as Indicator>::Output) -> Option<Order> {
        if let (Some(rsi), Some((macd, macdh))) = (rsi, macd) {
            let action = if
                self.previous_rsi < 30.0 &&
                rsi >= 30.0
            {
                Some(Order::Limit(Action::Buy, 0.01 * (1.0 + macdh.abs() / value * 10000.0), value))
            } else if
                self.previous_rsi > 70.0 &&
                rsi <= 70.0
            {
                Some(Order::Limit(Action::Sell, 0.01 * (1.0 + macdh.abs() / value * 10000.0), value))
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

//type MyTrader = StopLoss<Backoff<GobbleBadLongterm<RSITrader<4200, 300, 30.0, 70.0, 0.05>, "USDT">, 60>, "USDT", 0.97, 300>;
type MyTrader = StopLoss<FullStop<Backoff<RSITrader<4200, 300, 20.0, 80.0, 0.05>, 60>, "USDT">, "USDT", 0.95, 300>;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let environment = Simulated::new().await;
    let mut economy = Economy::<_, MyTrader>::new(environment);
    economy.run().await?;

    Ok(())
}
