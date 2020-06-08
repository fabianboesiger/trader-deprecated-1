use super::Monetary;

#[derive(Debug)]
pub struct Market {
    symbol: String,
    value: Option<Monetary>,
    base: usize,
    quote: usize,
    fee: Monetary,
}

impl Market {
    pub fn new(symbol: String, base: usize, quote: usize) -> Market {
        Market {
            symbol,
            value: None,
            base,
            quote,
            fee: 0.001,
        }
    }

    pub fn get_symbol(&self) -> &str {
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
}