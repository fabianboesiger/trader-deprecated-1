mod asset;
mod market;

use crate::{
    environments::{Environment, Event},
    traders::{Trader, Action},
    indicators::Indicator,
};
use asset::Asset;
use bigdecimal::Num;
use market::Market;
use std::collections::HashMap;
use binance_api::{Binance, exchange_info::symbol::Status};


pub struct Economy<N, E, T>
where
    N: Num,
    E: Environment<N>,
    T: Trader<N>,
{
    environment: E,
    markets: Vec<Market<N>>,
    assets: Vec<Asset<N>>,
    market_lookup: HashMap<String, usize>,
    asset_lookup: HashMap<String, usize>,
    traders: Vec<(T, Option<T::Indicators>)>,
    binance: Binance
}

impl<N, E, T> Economy<N, E, T>
where
    N: Num,
    E: Environment<N>,
    T: Trader<N>,
{
    pub fn new(environment: E) -> Economy<N, E, T> {
        Economy {
            environment,
            markets: Vec::new(),
            assets: Vec::new(),
            market_lookup: HashMap::new(),
            asset_lookup: HashMap::new(),
            traders: Vec::new(),
            binance: Binance::new()
        }
    }

    pub async fn run(&mut self) -> Result<(), ()> {
        let exchange_info = self.binance.exchange_info().await.unwrap();
        for symbol in exchange_info.symbols {
            if symbol.status == Status::Trading
                && (symbol.base_asset == "USDT" || symbol.quote_asset == "USDT")
            {
                self.add_market(symbol.symbol, symbol.base_asset, symbol.quote_asset);
            }
        }

        for market in &self.markets {
            self.traders.push((
                T::initialize(
                    &self.assets[market.get_base()].get_symbol(),
                    &self.assets[market.get_quote()].get_symbol()
                ),
                None
            ));
        }

        // Poll environment for events.
        loop {
            let event = self.environment.poll().await;
            match event {
                Event::UpdateTime(duration) => {
                    for _ in 0..duration.as_secs() {
                        for (market, (trader, indicator)) in self.markets.iter().zip(self.traders.iter_mut()) {
                            let action = if let Some(value) = market.get_value() {
                                if let Some(indicator) = indicator {
                                    trader.evaluate(indicator.evaluate(value))
                                } else {
                                    *indicator = Some(T::Indicators::initialize(value));
                                    Action::Hold
                                }
                            } else {
                                Action::Hold
                            };
                            match action {
                                Action::Buy(_, _) => {
                                    println!("buy {}", market.get_symbol());
                                }
                                Action::Sell(_, _) => {
                                    println!("sell {}", market.get_symbol());
                                },
                                Action::Hold => {
    
                                }
                            }
                        }
                    }
                }
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
        self.asset_lookup.insert(symbol.clone(), index);
        self.assets.push(Asset::new(symbol));
        index
    }

    fn add_market(&mut self, symbol: String, base: String, quote: String) -> usize {
        let base_index = self.add_asset(base);
        let quote_index = self.add_asset(quote);
        let index = self.markets.len();
        self.market_lookup.insert(symbol.clone(), index);
        self.markets.push(Market::new(symbol, base_index, quote_index));
        index
    }

    fn get_market_mut(&mut self, symbol: &str) -> Option<&mut Market<N>> {
        if let Some(index) = self.market_lookup.get(symbol) {
            Some(self.markets.get_mut(*index).unwrap())
        } else {
            None
        }
    }

    fn get_asset_mut(&mut self, symbol: &str) -> Option<&mut Asset<N>> {
        if let Some(index) = self.asset_lookup.get(symbol) {
            Some(self.assets.get_mut(*index).unwrap())
        } else {
            None
        }
    }
}
