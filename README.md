# Trader

A trading platform for Binance.

## TODOs

- [ ] Web interface for easy information lookup
  - [ ] Insert inforation into database
  - [ ] Build the website
    - [x] Get the basics working
    - [ ] Make it beautiful
- [ ] Realtime simulated test
- [ ] Implementing order functionality
  - [ ] Filter implementation
  - [ ] Filter testing
  - [ ] API implementation
  - [ ] Testing real orders
- [ ] Earn some money
- [ ] Code cleanup
  - [x] Proper market and asset symbol types

## Getting Started

### Binance Account

Create a Binance account and obtain your API key and secret key.

### PostgreSQL Database Setup

Set up a PostgreSQL Database to connect the trader to and add the following tables.

```sql
CREATE TABLE tickers (
    symbol VARCHAR(16),
    value DOUBLE PRECISION,
    timestamp BIGINT,
    PRIMARY KEY(symbol, timestamp)
);
```

### Environment Setup

Set the following environment variables either by editing the system environment or by adding a `.env` file to the project root.

```text
BINANCE_API_KEY=<API KEY HERE>
BINANCE_SECRET_KEY=<SECRET KEY HERE>
DATABASE_URL=<postgresql://user:password@localhost/database>
DATA_INTERVAL=<SECONDS>
```

### Choose your Trading Strategy

Choose an existing trading strategy or implement a new one.

```rust
type MyTrader = StopLoss<Backoff<RSITrader<4200, 30.0, 70.0, 0.1>, 60>, "USDT", 0.95>;
```

### Compilation

Compile the trading bot using `cargo run --release`.
