mod asset;
mod market;
mod symbols;

pub use asset::Asset;
pub use market::Market;
pub use symbols::{AssetSymbol, MarketSymbol};

use crate::{
    environments::{Environment, Event},
    indicators::Indicator,
    traders::{Order, Action, Trader},
};
use std::collections::HashMap;
use std::time::Duration;
use binance_async::model::Side;

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
                .map(|market| market.get_symbol().as_str())
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
        loop {
            let event = self.environment.poll().await;
            match event {
                Event::Evaluate(timestamp) => {
                    let mut actions = Vec::new();
                    for (market, (trader, indicator)) in
                        self.markets.iter().zip(self.traders.iter_mut())
                    {
                        let action = if let Some(value) = market.get_value() {
                            if let Some(indicator) = indicator {
                                trader.evaluate(indicator.evaluate(value))
                            } else {
                                *indicator = Some(T::Indicators::initialize(value));
                                None
                            }
                        } else {
                            None
                        };
                        actions.push(action);
                    }

                    for (market, order) in self.markets.iter().zip(actions.iter()) {
                        if let Some(order) = order {
                            match order {
                                Order::Limit(Action::Buy, fraction, price) => {
                                    let mut base_quantity = fraction
                                        * self.total_balance()
                                        * self.value_from_to(
                                            REFERENCE_ASSET,
                                            self.assets[market.get_base()].get_symbol(),
                                        );
                                    let quote_balance = self.assets[market.get_quote()].get_balance();
                                    if quote_balance > 0.0 {
                                        if quote_balance / price < 2.0 * base_quantity {
                                            base_quantity = quote_balance / price;
                                        }

                                        println!("buy {} {:?}", market.get_symbol(), timestamp);
                                        if let Ok(order) = market.apply_filters(Order::Limit(Action::Buy, base_quantity, *price)) {
                                            self.environment.order(market.get_symbol(), order).await.unwrap();
                                        }
                                        /*
                                        self.assets[market.get_base()]
                                            .add_balance(sell_quantity / price * (1.0 - market.get_fee()));
                                        self.assets[market.get_quote()].add_balance(-sell_quantity);
                                        */
                                    }
                                },
                                Order::Limit(Action::Sell, fraction, price) => {
                                    let mut base_quantity = fraction
                                        * self.total_balance()
                                        * self.value_from_to(
                                            REFERENCE_ASSET,
                                            self.assets[market.get_base()].get_symbol(),
                                        );
                                    let base_balance = self.assets[market.get_base()].get_balance();
                                    if base_balance > 0.0 {
                                        if base_balance < 2.0 * base_quantity {
                                            base_quantity = base_balance;
                                        }
                                        println!("sell {} {:?}", market.get_symbol(), timestamp);
                                        if let Ok(order) = market.apply_filters(Order::Limit(Action::Sell, base_quantity, *price)) {
                                            self.environment.order(market.get_symbol(), order).await.unwrap();
                                        }
                                        /*
                                        self.assets[market.get_base()].add_balance(-sell_quantity);
                                        self.assets[market.get_quote()]
                                            .add_balance(sell_quantity * price * (1.0 - market.get_fee()));
                                        */
                                    }
                                },
                                _ => {}
                            }
                        }
                    }

                    if timestamp % 3600 == 0 {
                        println!("{} total: {} USDT", timestamp, self.total_balance());
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
                Event::ExecutedOrder(query_order, quantity) => {
                    let market = self.get_market(&query_order.symbol).unwrap();
                    let base = market.get_base();
                    let quote = market.get_quote();
                    let fee = market.get_fee();
                    match query_order.side {
                        Side::Buy => {
                            self.assets[base]
                                .add_balance(query_order.executed_qty * (1.0 - fee));
                            self.assets[quote].add_balance(-query_order.executed_qty * query_order.price);
                        },
                        Side::Sell => {
                            self.assets[base]
                                .add_balance(-query_order.executed_qty);
                            self.assets[quote].add_balance(query_order.executed_qty * query_order.price * (1.0 - fee));
                        },
                    }
                    self.environment.update_balances(self.assets
                        .iter()
                        .map(|asset|
                            (asset.get_symbol(), asset.get_balance())
                        )
                        .collect::<Vec<(&AssetSymbol, Monetary)>>()
                    ).await;
                },
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
        let base_index = self.add_asset(base.clone());
        let quote_index = self.add_asset(quote.clone());
        let index = self.markets.len();
        self.market_lookup.insert(symbol.clone(), index);
        self.markets
            .push(Market::new((base, quote), base_index, quote_index));
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
