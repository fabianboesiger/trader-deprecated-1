#![allow(incomplete_features)]
#![feature(const_generics)]

mod economy;
mod environments;
mod indicators;
mod traders;

use economy::Economy;
use environments::Simulated;
use traders::{RSITrader, Backoff, StopLoss};

type MyTrader = StopLoss<Backoff<RSITrader<4200, 30.0, 70.0, 0.1>, 60>, "USDT", 0.95>;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let environment = Simulated::new().await;
    let mut economy = Economy::<_, MyTrader>::new(environment);
    economy.run().await?;

    Ok(())
}
