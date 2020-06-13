use super::{Monetary, Order, symbols::{MarketSymbol, AssetSymbol}};
use binance_async::model::ExchangeFilter;

const FEE: Monetary = 0.001;

pub struct Market {
    symbol: MarketSymbol,
    value: Option<Monetary>,
    base: usize,
    quote: usize,
    fee: Monetary,
    filters: Vec<ExchangeFilter>
}

impl Market {
    pub fn new(symbol: (String, String), base: usize, quote: usize) -> Market {
        Market {
            symbol: symbol.into(),
            value: None,
            base,
            quote,
            fee: FEE,
            filters: Vec::new()
        }
    }

    pub fn get_symbol(&self) -> &MarketSymbol {
        &self.symbol
    }

    pub fn get_value(&self) -> Option<Monetary> {
        self.value
    }

    pub fn base_to_quote(&self) -> Option<Monetary> {
        if let Some(value) = self.value {
            Some(value * (1.0 - self.fee))
        } else {
            None
        }
    }

    pub fn quote_to_base(&self) -> Option<Monetary> {
        if let Some(value) = self.value {
            Some((1.0 / value) * (1.0 - self.fee))
        } else {
            None
        }
    }

    pub fn set_value(&mut self, value: Monetary) {
        debug_assert!(value >= 0.0);
        self.value = Some(value);
    }

    pub fn get_base(&self) -> usize {
        self.base
    }

    pub fn get_quote(&self) -> usize {
        self.quote
    }

    pub fn get_fee(&self) -> Monetary {
        self.fee
    }

    pub fn apply_filters(&self, order: Order) -> Result<Order, ()> {
        let mut output = Ok(order);
        for filter in &self.filters {
            if let Ok(order) = output {
                output = apply_filter(filter, order);
            } else {
                return output;
            }
        }
        output
    }
}

fn apply_filter(filter: &ExchangeFilter, order: Order) -> Result<Order, ()> {
    Ok(order)
}