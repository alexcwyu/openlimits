> Historical archive captured 2026-10-13 from source base `ef076a6f7e7b9cffbdc1d848a31636c50ac5f6c5`. Original document SHA256: `5e7acb33d34b1337ea08466b9a5b7db84acef11cdec678266f22c9882e21485d`. This superseded guide is not evidence of current capability support. Use [the source-reviewed guide](../ayu-develop.md).

# OpenLimits — Workflow

> **Last Updated**: 2026-04-07T00:00:00Z
> **Git Hash**: `4137b08`

## Order Execution Flow

This sequence covers how a `limit_buy` call travels from user code through the trait layer to the exchange REST API and back.

```mermaid
sequenceDiagram
    participant User as User Code
    participant OL as OpenLimits::instantiate
    participant Exchange as Exchange impl<br/>(e.g. Binance)
    participant Transport as Transport<br/>(reqwest + HMAC)
    participant API as Exchange REST API

    User->>OL: instantiate::<Binance>(BinanceParameters)
    OL->>Exchange: Binance::new(params)
    Exchange->>Transport: Transport::with_credential(api_key, api_secret, sandbox)
    Exchange->>API: GET /api/v3/exchangeInfo  (refresh_market_info)
    API-->>Exchange: Symbol filters (lot size, tick size)
    Exchange-->>OL: Result<Binance>
    OL-->>User: Result<Binance>

    User->>Exchange: limit_buy(&OpenLimitOrderRequest)
    Exchange->>Exchange: get_pair(market_pair) → MarketPairHandle
    Exchange->>Transport: limit_buy(pair, size, price, tif, post_only)
    Transport->>Transport: Sign request (HMAC-SHA256 timestamp + params)
    Transport->>API: POST /api/v3/order
    API-->>Transport: JSON order response
    Transport-->>Exchange: model::Transaction
    Exchange->>Exchange: Transaction → shared Order (From impl)
    Exchange-->>User: Result<Order>
```

Source references:
- `ExchangeAccount::limit_buy`: `ext-systems/openlimits/crates/openlimits-exchange/src/traits/exchange_account.rs`
- Binance implementation: `ext-systems/openlimits/crates/openlimits-binance/src/lib.rs`
- `Transport` (signing): `ext-systems/openlimits/crates/openlimits-binance/src/transport.rs`
- `OpenLimits::instantiate`: `ext-systems/openlimits/src/openlimits.rs`

## Order Cancellation Flow

```mermaid
sequenceDiagram
    participant User as User Code
    participant Exchange as Exchange impl
    participant Transport as Transport
    participant API as Exchange REST API

    User->>Exchange: cancel_order(&CancelOrderRequest { id, market_pair })
    Exchange->>Exchange: validate market_pair present
    Exchange->>Transport: cancel_order(pair, u64_order_id)
    Transport->>API: DELETE /api/v3/order
    API-->>Transport: JSON canceled order
    Transport-->>Exchange: model::OrderCanceled
    Exchange->>Exchange: OrderCanceled → shared OrderCanceled
    Exchange-->>User: Result<OrderCanceled>
```

Source: `ext-systems/openlimits/crates/openlimits-binance/src/lib.rs` (lines 214–229)

## Market Data Flow

```mermaid
sequenceDiagram
    participant User as User Code
    participant Exchange as Exchange impl
    participant Transport as Transport
    participant API as Exchange REST API

    Note over User,API: Order Book (REST)
    User->>Exchange: order_book(&OrderBookRequest)
    Exchange->>Transport: get_depth(market_pair, limit)
    Transport->>API: GET /api/v3/depth
    API-->>Transport: JSON { bids, asks, lastUpdateId }
    Transport-->>Exchange: model::OrderBook
    Exchange->>Exchange: OrderBook → OrderBookResponse (From impl)
    Exchange-->>User: Result<OrderBookResponse>

    Note over User,API: OHLCV Candles
    User->>Exchange: get_historic_rates(&GetHistoricRatesRequest)
    Exchange->>Exchange: Build KlineParams (interval, paginator, symbol)
    Exchange->>Transport: get_klines(&params)
    Transport->>API: GET /api/v3/klines
    API-->>Transport: JSON kline array
    Transport-->>Exchange: KlineSummaries
    Exchange->>Exchange: KlineSummary → Candle (From impl)
    Exchange-->>User: Result<Vec<Candle>>
```

Source: `ext-systems/openlimits/crates/openlimits-exchange/src/traits/exchange_market_data.rs`

## WebSocket Subscription Flow

```mermaid
sequenceDiagram
    participant User as User Code
    participant WS as ExchangeWebsocket<br/>(ExchangeStream impl)
    participant Tokio as Tokio Runtime
    participant WSConn as WebSocket Connection<br/>(tokio-tungstenite)
    participant Exchange as Exchange WS Endpoint

    User->>WS: new(InitParams)  [e.g. CoinbaseWebsocket::new]
    WS->>WSConn: connect to wss://...
    WSConn-->>WS: Connected
    WS-->>User: Result<WS>

    User->>WS: subscribe(OrderBookUpdates(market_pair), callback)
    WS->>WS: create_stream_specific([subscription])
    WS->>WSConn: send subscribe frame (exchange format)
    WSConn->>Exchange: subscribe message
    Exchange-->>WSConn: subscription confirmation

    WS->>Tokio: tokio::spawn(async loop)
    loop Every incoming frame
        WSConn-->>Tokio: Raw message bytes
        Tokio->>Tokio: deserialize → Exchange-specific Response
        Tokio->>Tokio: Response.try_into() → WebSocketResponse<R>
        Tokio->>User: callback(&Ok(WebSocketResponse))
    end

    Note over Tokio,User: On disconnect / error
    Tokio->>User: callback(&Err(OpenLimitsError::SocketError()))

    User->>WS: disconnect()
    WS->>WSConn: close WebSocket
```

Source references:
- `ExchangeStream` trait: `ext-systems/openlimits/crates/openlimits-exchange/src/traits/stream/exchange_stream.rs`
- `Subscriptions` wrapper: `ext-systems/openlimits/crates/openlimits-exchange/src/traits/stream/subscriptions.rs`
- `CallbackHandle`: `ext-systems/openlimits/crates/openlimits-exchange/src/traits/stream/callback_handle.rs`
- Orderbook example: `ext-systems/openlimits/examples/rust/orderbook.rs`
- WebSocket message types: `ext-systems/openlimits/crates/openlimits-exchange/src/model/websocket.rs`

## Exchange Initialization Flow

Each exchange crate follows this initialization pattern:

1. User calls `OpenLimits::instantiate::<E>(params)`.
2. The library calls `E::new(params)` which constructs an HTTP `Transport` with optional HMAC credentials.
3. `refresh_market_info()` is called immediately to populate the local `ExchangeInfo` cache (market pairs, lot sizes, tick sizes).
4. The initialized exchange struct is returned to the user.

This eager fetch ensures that all subsequent `get_pair()` lookups are synchronous reads from the in-memory cache rather than additional network calls.

Source: `ext-systems/openlimits/crates/openlimits-binance/src/lib.rs` (Binance::new impl)

## See Also

- [architecture.md](../architecture.md) — trait hierarchy and crate layout
- [state-management.md](../state-management.md) — data models used in these flows
- [development.md](../development.md) — environment setup for running examples
