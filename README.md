# Trader

A trading platform for Binance.

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

### Compilation

Compile the trading bot using `cargo run --release`.
