# OpenLimits

> **Last Updated**: 2026-04-07T00:00:00Z
> **Git Hash**: `4137b08`

**Version**: 0.3.1-alpha.0
**License**: BSD-2-Clause
**Language**: Rust

## Overview

OpenLimits is an open-source Rust high-performance cryptocurrency trading API library with support for multiple exchanges and language wrappers. It is focused on safety, correctness, and speed, providing a unified abstraction layer over exchange-specific APIs.

**Key Points**:
- Purpose: Unified exchange API client for spot cryptocurrency trading
- Target audience: Algorithmic traders, quant developers, exchange integration engineers
- Primary trading focus: Spot cryptocurrency markets (Binance, Coinbase, Nash, Huobi)

## Summary

OpenLimits defines a trait-based abstraction that normalizes the different REST and WebSocket APIs of major cryptocurrency exchanges into a single coherent interface. User code calls standard Rust async methods (`limit_buy`, `order_book`, `get_price_ticker`, etc.) and the library translates those into exchange-specific HTTP or WebSocket calls, then maps the responses back into shared model types.

The library is structured as a Cargo workspace. The `openlimits-exchange` crate defines all shared traits (`Exchange`, `ExchangeAccount`, `ExchangeMarketData`, `ExchangeStream`) and model types (`Order`, `Trade`, `Candle`, `Balance`, `OrderBookResponse`). Individual exchange crates (`openlimits-binance`, `openlimits-coinbase`, `openlimits-nash`, `openlimits-huobi`) implement those traits against their respective endpoints. The root `openlimits` crate re-exports everything through a single entry point and provides the `OpenLimits::instantiate` factory function.

WebSocket streaming is supported through the `ExchangeStream` trait and the callback-based `subscribe` API, which runs a Tokio background task and forwards messages to a user-supplied closure. The library uses `rust_decimal` for all price and quantity arithmetic to avoid floating-point rounding errors, `tokio` for async I/O, and `serde` for JSON serialization.

## Key Features

| Feature | Detail |
|---|---|
| Supported Exchanges | Binance, Coinbase Pro, Nash, Huobi |
| Order Types | Limit, Market |
| Market Data | Order book, price ticker, OHLCV candles, trade history |
| WebSocket Streaming | Order book updates, trade feed |
| Account Operations | Balances, open orders, order history, trade history |
| Error Handling | Typed `OpenLimitsError` enum covering all failure modes |
| Decimal Precision | `rust_decimal::Decimal` — no floating-point rounding |
| Async Runtime | Tokio 1.x |
| Language Bindings | C# (via `bindings/` crate, optional feature) |
| Environment | Production and Sandbox (testnet) for each exchange |

## Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
openlimits = "0.3.0"
tokio     = { version = "1", features = ["full"] }
dotenv    = "0.15"
```

Place API credentials in a `.env` file:

```
BINANCE_API_KEY=your_key
BINANCE_API_SECRET=your_secret
```

Place a limit buy order on Binance and subscribe to the ETH/BTC order book:

```rust
use openlimits::prelude::*;
use openlimits::exchange::binance::{Binance, BinanceParameters, BinanceCredentials};
use openlimits::exchange::coinbase::client::stream::CoinbaseWebsocket;
use openlimits::exchange::coinbase::CoinbaseParameters;
use openlimits::model::websocket::{OpenLimitsWebSocketMessage::OrderBook, Subscription::OrderBookUpdates, WebSocketResponse::Generic};
use openlimits_exchange::model::{market_pair::MarketPair, currency::Currency, OpenLimitOrderRequest, TimeInForce};
use rust_decimal_macros::dec;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    // --- REST: place a limit buy on Binance ---
    let binance: Binance = OpenLimits::instantiate(BinanceParameters {
        credentials: Some(BinanceCredentials {
            api_key:    std::env::var("BINANCE_API_KEY")?,
            api_secret: std::env::var("BINANCE_API_SECRET")?,
        }),
        ..BinanceParameters::sandbox()
    })
    .await?;

    let pair = MarketPair(Currency::ETH, Currency::BTC);
    let order = binance.limit_buy(&OpenLimitOrderRequest {
        market_pair:     pair.clone(),
        size:            dec!(0.01),
        price:           dec!(0.03),
        time_in_force:   TimeInForce::GoodTillCancelled,
        post_only:       false,
        client_order_id: None,
    })
    .await?;
    println!("Order placed: {:?}", order);

    // --- WebSocket: stream Coinbase order book ---
    let ws = CoinbaseWebsocket::new(CoinbaseParameters::production()).await?;
    ws.subscribe(OrderBookUpdates(pair), |m| {
        if let Ok(Generic(OrderBook(book))) = m.as_ref() {
            println!("{:?}", book);
        }
    })
    .await?;

    std::thread::sleep(std::time::Duration::from_secs(5));
    Ok(())
}
```

## Architecture Summary

OpenLimits follows a **trait + crate-per-exchange** pattern:

```
openlimits (root)
├── openlimits-exchange   — shared traits, models, errors
├── openlimits-binance    — Binance REST + WebSocket
├── openlimits-coinbase   — Coinbase Pro REST + WebSocket
├── openlimits-nash       — Nash DEX REST + WebSocket
├── openlimits-huobi      — Huobi REST + WebSocket
└── openlimits-messaging  — pub/sub utilities
```

User code calls `Exchange`, `ExchangeAccount`, or `ExchangeMarketData` trait methods. Exchange-specific crates translate those calls into HTTP requests via `Transport` (built on `reqwest`), deserialize exchange JSON into shared model types, and return typed `Result<T, OpenLimitsError>`.

## Documentation Index

| Document | Contents |
|---|---|
| [architecture.md](architecture.md) | System architecture diagrams, trait hierarchy, crate map |
| [workflow.md](workflow.md) | Order execution flow, WebSocket subscription flow |
| [state-management.md](state-management.md) | Order book state, connection state, data models |
| [development.md](development.md) | Setup guide, project structure, config reference, troubleshooting |

## Source References

- Root crate entry point: `ext-systems/openlimits/src/lib.rs`
- `OpenLimits` factory struct: `ext-systems/openlimits/src/openlimits.rs`
- Shared traits: `ext-systems/openlimits/crates/openlimits-exchange/src/traits/mod.rs`
- `Exchange` supertrait: `ext-systems/openlimits/crates/openlimits-exchange/src/traits/exchange.rs`
- `ExchangeAccount` trait: `ext-systems/openlimits/crates/openlimits-exchange/src/traits/exchange_account.rs`
- `ExchangeMarketData` trait: `ext-systems/openlimits/crates/openlimits-exchange/src/traits/exchange_market_data.rs`
- `ExchangeStream` trait: `ext-systems/openlimits/crates/openlimits-exchange/src/traits/stream/exchange_stream.rs`
- Shared data models: `ext-systems/openlimits/crates/openlimits-exchange/src/model/mod.rs`
- Error types: `ext-systems/openlimits/crates/openlimits-exchange/src/errors.rs`
- Binance implementation: `ext-systems/openlimits/crates/openlimits-binance/src/lib.rs`
- Order book WebSocket example: `ext-systems/openlimits/examples/rust/orderbook.rs`
- Workspace manifest: `ext-systems/openlimits/Cargo.toml`
