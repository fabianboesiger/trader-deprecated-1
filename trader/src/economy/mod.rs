mod asset;
mod market;

pub use asset::Asset;
pub use market::Market;

use crate::{
    environments::{Environment, Event},
    indicators::Indicator,
    traders::{Action, Trader},
};
use std::collections::HashMap;
use std::time::Duration;

const REFERENCE_ASSET: &'static str = "USDT";

pub type Monetary = f64;

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
    traders: Vec<(T, Option<T::Indicators>)>,
    uptime: Duration,
    reference_asset: usize,
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
            uptime: Duration::from_secs(0),
            reference_asset: 0,
        }
    }

    pub async fn run(&mut self) -> Result<(), ()> {
        self.add_asset(String::from("USDT"));

        let markets = self.environment.initialize().await?;
        for market in markets {
            self.add_market(market.symbol, market.base_asset, market.quote_asset);
        }

        println!(
            "added {} markets: {:?}",
            self.markets.len(),
            self.markets
                .iter()
                .map(|market| market.get_symbol())
                .collect::<Vec<&str>>()
        );

        for market in &self.markets {
            self.traders.push((
                T::initialize(
                    &self.assets[market.get_base()].get_symbol(),
                    &self.assets[market.get_quote()].get_symbol(),
                ),
                None,
            ));
        }

        // Poll environment for events.
        let mut last_report = self.uptime;
        loop {
            let event = self.environment.poll().await;
            match event {
                Event::Evaluate => {
                    self.uptime += Duration::from_secs(60);
                    let mut actions = Vec::new();
                    for (market, (trader, indicator)) in
                        self.markets.iter().zip(self.traders.iter_mut())
                    {
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
                        actions.push(action);
                    }

                    for (market, action) in self.markets.iter().zip(actions.iter()) {
                        match action {
                            Action::Buy(fraction, price) => {
                                let mut sell_quantity = fraction
                                    * self.total_balance()
                                    * self.value_from_to(
                                        REFERENCE_ASSET,
                                        self.assets[market.get_quote()].get_symbol(),
                                    );
                                let quote_balance = self.assets[market.get_quote()].get_balance();
                                if quote_balance > 0.0 {
                                    if quote_balance < 2.0 * sell_quantity {
                                        sell_quantity = quote_balance;
                                    }
                                    println!("buy {} {:?}", market.get_symbol(), self.uptime);
                                    self.assets[market.get_base()]
                                        .add_balance(sell_quantity / price * 0.999);
                                    self.assets[market.get_quote()].add_balance(-sell_quantity);
                                }
                            }
                            Action::Sell(fraction, price) => {
                                let mut sell_quantity = fraction
                                    * self.total_balance()
                                    * self.value_from_to(
                                        REFERENCE_ASSET,
                                        self.assets[market.get_base()].get_symbol(),
                                    );
                                let base_balance = self.assets[market.get_base()].get_balance();
                                if base_balance > 0.0 {
                                    if base_balance < 2.0 * sell_quantity {
                                        sell_quantity = base_balance;
                                    }
                                    println!("sell {} {:?}", market.get_symbol(), self.uptime);
                                    self.assets[market.get_base()].add_balance(-sell_quantity);
                                    self.assets[market.get_quote()]
                                        .add_balance(sell_quantity * price * 0.999);
                                }
                            }
                            Action::Hold => {}
                        }
                    }

                    if self.uptime > last_report + Duration::from_secs(3600) {
                        last_report = self.uptime;
                        println!("total: {} USDT", self.total_balance());
                        for asset in &self.assets {
                            let balance = asset.get_balance();
                            if balance > 0.0 {
                                println!(
                                    "{} {} = {} {}",
                                    balance,
                                    asset.get_symbol(),
                                    balance
                                        * self.value_from_to(asset.get_symbol(), REFERENCE_ASSET),
                                    REFERENCE_ASSET
                                );
                            }
                        }
                    }
                }
                Event::SetMarketValue(symbol, value) => {
                    if let Some(market) = self.get_market_mut(&symbol) {
                        market.set_value(value);
                    }
                }
                Event::SetAssetBalance(symbol, balance) => {
                    if let Some(asset) = self.get_asset_mut(&symbol) {
                        asset.set_balance(balance);
                    }
                }
            }
        }
    }

    fn add_asset(&mut self, symbol: String) -> usize {
        if let Some(index) = self.asset_lookup.get(&symbol) {
            *index
        } else {
            let index = self.assets.len();

            if symbol == REFERENCE_ASSET {
                self.reference_asset = index;
            }

            self.asset_lookup.insert(symbol.clone(), index);
            self.assets.push(Asset::new(symbol));
            index
        }
    }

    fn add_market(&mut self, symbol: String, base: String, quote: String) -> usize {
        let base_index = self.add_asset(base);
        let quote_index = self.add_asset(quote);
        let index = self.markets.len();
        self.market_lookup.insert(symbol.clone(), index);
        self.markets
            .push(Market::new(symbol, base_index, quote_index));
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
    /*
    fn get_market(&self, symbol: &str) -> Option<&Market> {
        if let Some(index) = self.market_lookup.get(symbol) {
            Some(self.markets.get(*index).unwrap())
        } else {
            None
        }
    }

    fn get_asset(&self, symbol: &str) -> Option<&Asset> {
        if let Some(index) = self.asset_lookup.get(symbol) {
            Some(self.assets.get(*index).unwrap())
        } else {
            None
        }
    }
    */

    fn value_from_to(&self, from: &str, to: &str) -> Monetary {
        if from == to {
            return 1.0;
        }

        if let Some(index) = self.market_lookup.get(&format!("{}{}", from, to)) {
            if let Some(market) = self.markets.get(*index) {
                if let Some(value) = market.base_to_quote() {
                    return value;
                }
            }
        }

        if let Some(index) = self.market_lookup.get(&format!("{}{}", to, from)) {
            if let Some(market) = self.markets.get(*index) {
                if let Some(value) = market.quote_to_base() {
                    return value;
                }
            }
        }

        0.0
    }

    fn total_balance(&self) -> Monetary {
        let mut total = 0.0;

        for asset in &self.assets {
            let balance = asset.get_balance();
            if balance > 0.0 {
                total += balance * self.value_from_to(asset.get_symbol(), REFERENCE_ASSET);
            }
        }

        total
    }
}
