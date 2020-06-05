mod asset;
mod market;

use crate::{
    environments::{Environment, Event},
    traders::Trader,
};
use asset::Asset;
use bigdecimal::BigDecimal;
use market::Market;
use std::collections::HashMap;

pub type Monetary = BigDecimal;

pub struct Economy<E, T>
where
    E: Environment,
    T: Trader,
{
    environment: E,
    markets: Vec<Market>,
    assets: Vec<Asset>,
    market_lookup: HashMap<String, usize>,
    asset_lookup: HashMap<String, usize>,
    traders: Vec<T>,
}

impl<E, T> Economy<E, T>
where
    E: Environment,
    T: Trader,
{
    pub fn new(environment: E) -> Economy<E, T> {
        Economy {
            environment,
            markets: Vec::new(),
            assets: Vec::new(),
            market_lookup: HashMap::new(),
            asset_lookup: HashMap::new(),
            traders: Vec::new(),
        }
    }

    pub async fn run(&mut self) -> Result<(), ()> {
        // Poll environment for events.
        loop {
            let event = self.environment.poll().await;
            println!("{:?}", event);
            match event {
                Event::UpdateTime(date_time) => {}
                Event::UpdateMarketValue(symbol, value) => {
                    if let Some(market) = self.get_market_mut(&symbol) {
                        market.set_value(value);
                    }
                }
            }
        }
    }

    fn add_asset(&mut self, symbol: String) -> usize {
        let index = self.assets.len();
        self.asset_lookup.insert(symbol, index);
        self.assets.push(Asset::new());
        index
    }

    fn add_market(&mut self, symbol: String, base: String, quote: String) -> usize {
        let base_index = self.add_asset(base);
        let quote_index = self.add_asset(quote);
        let index = self.markets.len();
        self.market_lookup.insert(symbol, index);
        self.markets.push(Market::new(base_index, quote_index));
        index
    }

    fn get_market_mut(&mut self, symbol: &str) -> Option<&mut Market> {
        if let Some(index) = self.market_lookup.get(symbol) {
            Some(self.markets.get_mut(*index).unwrap())
        } else {
            None
        }
    }

    fn get_asset_mut(&mut self, symbol: &str) -> Option<&mut Asset> {
        if let Some(index) = self.asset_lookup.get(symbol) {
            Some(self.assets.get_mut(*index).unwrap())
        } else {
            None
        }
    }
}
