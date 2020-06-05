use bigdecimal::Num;

#[derive(Debug)]
pub struct Market<N>
    where
        N: Num
{
    symbol: String,
    value: Option<N>,
    base: usize,
    quote: usize,
}

impl<N> Market<N>
    where
        N: Num
{
    pub fn new(symbol: String, base: usize, quote: usize) -> Market<N> {
        Market {
            symbol,
            value: None,
            base,
            quote,
        }
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    pub fn get_value(&self) -> Option<&N> {
        self.value.as_ref()
    }

    pub fn set_value(&mut self, value: N) {
        self.value = Some(value);
    }

    pub fn get_base(&self) -> usize {
        self.base
    }

    pub fn get_quote(&self) -> usize {
        self.quote
    }
}
