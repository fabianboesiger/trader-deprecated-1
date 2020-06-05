use super::Monetary;

pub struct Market {
    value: Option<Monetary>,
    base: usize,
    quote: usize,
}

impl Market {
    pub fn new(base: usize, quote: usize) -> Market {
        Market {
            value: None,
            base,
            quote,
        }
    }

    pub fn set_value(&mut self, value: Monetary) {
        self.value = Some(value);
    }
}
