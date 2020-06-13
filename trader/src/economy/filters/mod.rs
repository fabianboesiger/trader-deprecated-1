use binance_async::model::ExchangeFilter;
use crate::traders::Order;

fn apply(filter: ExchangeFilter, order: Order) -> Result<Order, ()> {
    Ok(order)
}