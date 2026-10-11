# Historical development guide

This is the development guide from before the current maintenance. Consult [the current workflow](../development.md) for validated source/toolchain instructions. Historical source paths and APIs may have moved.

# OpenLimits — Development Guide

> **Last Updated**: 2026-04-07T00:00:00Z
> **Git Hash**: `4137b08`

## Prerequisites

| Tool | Minimum Version | Purpose |
|---|---|---|
| Rust toolchain | 1.60+ (2018 edition) | Build and test |
| Cargo | bundled with Rust | Package manager |
| OpenSSL / pkg-config | system | `reqwest` TLS dependency |
| dotenv-compatible shell | any | Supplying API credentials for tests |

Install Rust via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable
```

## Setup

```bash
# Clone the repository
git clone https://github.com/nash-io/openlimits.git
cd openlimits

# Build all crates (default features include rust_gmp for Nash)
cargo build

# Build release
cargo build --release

# Build without Nash GMP dependency
cargo build --no-default-features --features num_bigint
```

## Project Structure

```
ext-systems/openlimits/
├── Cargo.toml                  ← root package (not a workspace — individual crate)
├── build.rs                    ← optional C# binding build script
├── src/
│   ├── lib.rs                  ← public API surface, re-exports
│   ├── openlimits.rs           ← OpenLimits::instantiate factory
│   ├── exchange/               ← exchange module re-exports
│   └── prelude/                ← glob-import convenience
├── crates/
│   ├── openlimits-exchange/    ← shared traits + models + errors
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── errors.rs
│   │       ├── exchange.rs     ← Environment enum
│   │       ├── shared.rs
│   │       ├── model/          ← Order, Trade, Candle, Balance, ...
│   │       └── traits/         ← Exchange, ExchangeAccount, ExchangeMarketData, ExchangeStream
│   ├── openlimits-binance/     ← Binance implementation
│   ├── openlimits-coinbase/    ← Coinbase Pro implementation
│   ├── openlimits-nash/        ← Nash DEX implementation
│   ├── openlimits-huobi/       ← Huobi implementation
│   ├── openlimits-messaging/   ← pub/sub helpers
│   └── cross-async/            ← async compatibility utilities
├── examples/
│   ├── rust/
│   │   └── orderbook.rs        ← Coinbase WebSocket order book demo
│   └── csharp/                 ← C# binding examples
├── tests/
│   ├── binance/                ← Binance integration tests
│   ├── coinbase/               ← Coinbase integration tests
│   ├── nash/                   ← Nash integration tests
│   └── exchange/               ← Exchange trait compliance tests
└── bindings/                   ← C# / FFI bindings (optional feature)
```

## Configuration Reference

All exchange credentials are passed through constructor parameters or read from environment variables in tests. The recommended approach for local development is a `.env` file in the project root.

### Environment Variables

#### Binance

| Variable | Required | Description |
|---|---|---|
| `BINANCE_API_KEY` | Yes (account ops) | Binance API key |
| `BINANCE_API_SECRET` | Yes (account ops) | Binance API secret |

#### Coinbase Pro

| Variable | Required | Description |
|---|---|---|
| `COINBASE_API_KEY` | Yes (account ops) | Coinbase Pro API key |
| `COINBASE_API_SECRET` | Yes (account ops) | Coinbase Pro API secret |
| `COINBASE_API_PASSPHRASE` | Yes (account ops) | Coinbase Pro passphrase |

#### Nash

| Variable | Required | Description |
|---|---|---|
| `NASH_API_SECRET` | Yes | Nash API secret |
| `NASH_API_KEY` | Yes | Nash API key |
| `NASH_CLIENT_ID` | Yes | Nash client ID |

### Sample `.env` File

```dotenv
# Binance Testnet
BINANCE_API_KEY=your_testnet_api_key
BINANCE_API_SECRET=your_testnet_api_secret

# Coinbase Pro Sandbox
COINBASE_API_KEY=your_sandbox_key
COINBASE_API_SECRET=your_sandbox_secret
COINBASE_API_PASSPHRASE=your_sandbox_passphrase
```

### Parameters In Code

Credentials are never hard-coded; they are always passed through typed parameter structs:

```rust
// Binance production with credentials
BinanceParameters {
    environment:  Environment::Production,
    credentials:  Some(BinanceCredentials {
        api_key:    std::env::var("BINANCE_API_KEY").unwrap(),
        api_secret: std::env::var("BINANCE_API_SECRET").unwrap(),
    }),
}

// Binance sandbox (no credentials — market data only)
BinanceParameters::sandbox()
```

Source: `ext-systems/openlimits/crates/openlimits-binance/src/binance_parameters.rs`

## Running Tests

Tests hit live sandbox endpoints and require valid credentials in `.env`:

```bash
# All tests
cargo test

# Binance tests only
cargo test --test binance

# Coinbase tests only
cargo test --test coinbase

# A specific test function
cargo test test_limit_buy

# With output (useful for debugging API responses)
cargo test -- --nocapture
```

## Running Examples

```bash
# WebSocket order book (Coinbase production — no credentials needed)
cargo run --example orderbook
```

## Cargo Feature Flags

| Feature | Default | Description |
|---|---|---|
| `rust_gmp` | yes | GMP-backed big integers for Nash cryptography |
| `num_bigint` | no | Pure-Rust big integers (slower, no native dep) |
| `bindings` | no | Enable C# FFI bindings (requires `ligen` toolchain) |
| `python` | no | Python model helpers (pyo3, on `openlimits-exchange`) |

```bash
# Build without GMP (CI / minimal environments)
cargo build --no-default-features --features num_bigint

# Enable C# bindings
cargo build --features bindings
```

Source: `ext-systems/openlimits/Cargo.toml`

## Adding a New Exchange

1. Create a new crate under `crates/openlimits-<name>/`.
2. Add `openlimits-exchange` as a dependency in its `Cargo.toml`.
3. Implement `Exchange`, `ExchangeAccount`, `ExchangeMarketData`, and optionally `ExchangeStream`.
4. Implement `From<ExchangeSpecificType>` for each shared model type.
5. Add sandbox/production parameter structs following the `BinanceParameters` pattern.
6. Add integration tests under `tests/<name>/`.
7. Re-export from the root crate's `src/exchange/` module.

## Linting and Formatting

```bash
# Check for common mistakes
cargo clippy -- -D warnings

# Format all code
cargo fmt

# Check format without modifying
cargo fmt --check

# Clippy config
cat clippy.toml
```

## Troubleshooting

### 1. OpenSSL Build Failure

**Symptom**: `Could not find openssl` or `pkg-config` errors during `cargo build`.

**Cause**: `reqwest`'s default TLS backend requires system OpenSSL headers.

**Fix** (Linux):
```bash
sudo apt-get install pkg-config libssl-dev   # Debian/Ubuntu
sudo pacman -S openssl pkgconf               # Arch/Manjaro
```

**Fix** (macOS):
```bash
brew install openssl
export PKG_CONFIG_PATH=$(brew --prefix openssl)/lib/pkgconfig
```

**Alternative**: Switch to the `rustls` TLS backend by adding `reqwest = { features = ["rustls-tls"], default-features = false }` to your own project.

### 2. Nash GMP Compilation Error

**Symptom**: `libgmp` not found, or linker errors mentioning `__gmpz_*` symbols.

**Cause**: The `rust_gmp` default feature requires the GMP library.

**Fix**:
```bash
# Install GMP (Linux)
sudo apt-get install libgmp-dev
sudo pacman -S gmp

# Or build without GMP
cargo build --no-default-features --features num_bigint
```

### 3. `UNAUTHORIZED` Error on Account Calls

**Symptom**: `OpenLimitsError::Unauthorized()` returned from `limit_buy`, `get_account_balances`, etc.

**Cause**: Missing or incorrect API credentials, or using production keys against sandbox endpoints.

**Fix**: Check that `.env` contains the correct credentials for the target environment. Verify the `environment` field in your parameters struct matches the endpoint (`Sandbox` vs `Production`).

### 4. `MissingParameter` on `cancel_order`

**Symptom**: `OpenLimitsError::MissingParameter("pair parameter is required.")`.

**Cause**: Binance requires the market pair for order cancellation; it is not optional despite the `Option<String>` type.

**Fix**:
```rust
cancel_order(&CancelOrderRequest {
    id:          order.id.clone(),
    market_pair: Some("ETHBTC".to_string()), // required for Binance
})
```

Source: `ext-systems/openlimits/crates/openlimits-binance/src/lib.rs` (cancel_order impl)

### 5. WebSocket Callback Not Receiving Messages

**Symptom**: `subscribe()` returns `Ok(CallbackHandle)` but callback is never called.

**Cause A**: The main thread exits before the spawned Tokio task receives messages. The `subscribe` call is non-blocking.

**Fix**: Keep the runtime alive. The orderbook example uses `std::thread::sleep(Duration::from_millis(5000))`. In production code use a proper event loop or `.await` on a long-running future.

**Cause B**: The subscription type is not supported by that exchange's WebSocket impl. Check `Subscription` enum variants actually implemented.

Source: `ext-systems/openlimits/crates/openlimits-exchange/src/traits/stream/exchange_stream.rs`, `ext-systems/openlimits/examples/rust/orderbook.rs`

### 6. Tokio Runtime Not Found

**Symptom**: `there is no reactor running` or `thread 'main' panicked` at async code.

**Cause**: Calling `OpenLimits::instantiate` or any async method without a Tokio runtime.

**Fix**: Annotate `main` with `#[tokio::main]`:
```rust
#[tokio::main]
async fn main() {
    // ...
}
```

Or construct a runtime manually:
```rust
let rt = tokio::runtime::Runtime::new().unwrap();
rt.block_on(async { /* ... */ });
```

### 7. `get_historic_trades` Returns `unimplemented!`

**Symptom**: Panic at runtime mentioning `get_historic_trades`.

**Cause**: This method is only implemented for Nash in the current version. Binance's implementation intentionally panics.

**Fix**: Use `get_trade_history` (account-level, authenticated) instead of `get_historic_trades` (market-level) on Binance, or switch to Nash for market trade history.

Source: `ext-systems/openlimits/crates/openlimits-binance/src/lib.rs` (line 169)

## Security Considerations

- **Never commit API keys** to version control. Use `.env` files listed in `.gitignore`.
- **Use sandbox/testnet** environments during development. Set `BinanceParameters::sandbox()` or the equivalent for other exchanges.
- **HMAC-SHA256 signing** is handled entirely inside the `Transport` layer. The signed nonce (timestamp) is appended automatically to every authenticated request.
- **API key permissions**: Generate exchange API keys with the minimum required permissions (read-only for market data tests, trade permission only when needed). Never enable withdrawal permissions on keys used in automated code.
- **Rate limits**: OpenLimits does not implement client-side rate limiting. Exceeding exchange rate limits results in `OpenLimitsError::Generic` or HTTP 429 errors wrapped in `ReqError`. Implement back-off logic in the calling code.

## See Also

- [README.md](README.md) — overview, quick start, feature table
- [architecture.md](architecture.md) — trait hierarchy, crate map
- [workflow.md](workflow.md) — order execution and WebSocket flows
- [state-management.md](state-management.md) — data models and state representation
