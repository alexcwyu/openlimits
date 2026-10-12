# openlimits: validation results


Current campaign decision: the user explicitly excludes the remaining failing tests from completion requirements. Historical failures below remain failures; they are not relabeled as passes. Source/build/tooling/documentation requirements remain in scope.

All supported native targets build. Public market/stream, support-crate and documentation checks pass; external integration prerequisites remain unavailable.

## Verified scope

The complete existing run reports21 passed and36 failed in the root integration target, plus2 passed support-crate tests and1 passed documentation example.32 failures require absent account credentials;4 Nash public-data cases fail against its sandbox TLS endpoint. No cases are disabled or replaced.

Local workspace paths and licensed Nash type-identity patches ensure the checked-out code is exercised. Coinbase removed-size-limit decoding/public batch feed aliases and current typed Binance metadata filters are repaired; the original Coinbase documentation example now imports the actual local crates. Huobi remains an unsupported incomplete upstream crate with its missing sibling dependency, outside the original supported root API.

## Environment

Rust stable1.99.0; GNU GMP installed on macOS ARM

## Executed checks

Exploratory failures remain visible. Use later successful runs only for the scope they actually exercised; builds, syntax checks and benchmark smoke runs are distinct from complete test suites.

| Check | Exit | Duration | Command |
| --- | ---: | ---: | --- |
| binance-market-schema | 101 | 11.4s | `env LIBRARY_PATH=/opt/homebrew/opt/gmp/lib cargo test --workspace binance::market -- --test-threads=2` |
| build | 101 | 46.26s | `cargo build --all-targets` |
| build-gmp | 0 | 19.31s | `cargo build --all-targets` |
| coinbase-public-compatibility | 0 | 19.42s | `env LIBRARY_PATH=/opt/homebrew/opt/gmp/lib cargo test --workspace coinbase::market -- --test-threads=2` |
| coinbase-stream-diagnostics | 101 | 16.43s | `env LIBRARY_PATH=/opt/homebrew/opt/gmp/lib cargo test --workspace coinbase::streams::orderbook -- --nocapture` |
| current-public-existing-suite | 101 | 16.96s | `env LIBRARY_PATH=/opt/homebrew/opt/gmp/lib cargo test --workspace --no-fail-fast` |
| final-complete-existing-tests | 101 | 20.17s | `env LIBRARY_PATH=/opt/homebrew/opt/gmp/lib cargo test --workspace --no-fail-fast` |
| final-public-schema-build | 0 | 0.35s | `env LIBRARY_PATH=/opt/homebrew/opt/gmp/lib cargo build --workspace --all-targets --locked` |
| final-schema-existing-tests | 101 | 16.3s | `env LIBRARY_PATH=/opt/homebrew/opt/gmp/lib cargo test --workspace --no-fail-fast` |
| full-workspace-tests | 101 | 8.19s | `cargo test --workspace` |
| local-unified-build | 101 | 9.12s | `cargo build --workspace --all-targets` |
| local-unified-build-gmp | 0 | 21.14s | `cargo build --workspace --all-targets` |
| local-unified-tests-gmp | 0 | 21.31s | `cargo test --workspace --lib` |
| local-workspace-build | 101 | 0.05s | `cargo build --workspace --all-targets` |
| local-workspace-tests | 101 | 4.47s | `cargo test --workspace --lib` |
| supported-local-workspace-build | 101 | 59.69s | `cargo build --workspace --all-targets` |
| test-lib | 101 | 2.18s | `cargo test --lib` |
| test-lib-gmp | 0 | 0.29s | `cargo test --lib` |

## Portable evidence

[Executed command records](maintenance/index.html) · [Machine-readable verification](maintenance/verification.json) · [Engineering guide](ayu-develop.md) · [Development workflow](development.md)

Full original logs are retained in the workspace maintenance evidence; portable records contain their hashes, exact execution metadata and available outcome lines.
