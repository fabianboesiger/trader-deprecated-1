use super::Filter;
use crate::economy::Monetary;
use binance_async::model::OrderRequest;

pub struct PriceFilter {
    min_price: Monetary,
    max_price: Monetary,
    tick_size: Monetary
}

impl Filter for PriceFilter {
    fn apply(&self, mut input: OrderRequest) -> Result<OrderRequest, ()> {
        if input.price < self.min_price {
            return Err(());
        }
        if input.price > self.max_price {
            return Err(());
        }

        input.price = self.min_price + ((input.price - self.min_price) / self.tick_size).round() * self.tick_size;
        debug_assert!((input.price - self.min_price) % self.tick_size == 0.0);

        Ok(input)
    }
}