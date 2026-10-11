# openlimits: development workflow and layout

Use branch `feature/ayu_develop`. The actual upstream default branch and all fetched branch/tag identities are recorded in the maintenance sync evidence. Merge existing work; do not reset divergent branches or replace conflicting tags.

## Source synchronization

```sh
git fetch origin
git fetch upstream
git switch feature/ayu_develop
git merge origin/feature/ayu_develop
# Merge the actual default branch documented in the sync result.
```

This maintenance used namespaced remote tags before comparing tag identities. Missing upstream tags/branches were pushed to the fork; divergent branches were merged when conflict-free, and conflicting identities were retained for explicit reconciliation. The feature branch contains the default branch plus the maintenance changes. Source snapshot and architecture are documented in [ayu-develop.md](ayu-develop.md).

## Repository layout

- `docs/ayu-develop.md` and `docs/ayu-develop.html`: engineering guide with architecture, workflow, class and sequence diagrams and implementation links.
- `docs/development.md`: toolchain, layout and contributor workflow.
- `docs/validation.md`: exact executed commands, outcomes and limitations.

## Native Rust toolchain

Rust stable1.99.0 is installed through rustup. Compile all applicable targets and run the existing test suite. Optional Python features require the validated managed interpreter through `PYO3_PYTHON`. Source locks are checked in where appropriate for application/workspace reproducibility; build outputs remain ignored.

## Verification and documentation

Use the commands and results in [validation.md](validation.md). Markdown is the editable source of the guide; HTML embeds rendered SVG diagrams for offline browsing. Keep actual source links valid after moving modules. Preserve existing cases/assertions while correcting incompatible APIs, genuine resource leaks or invalid fixtures. No additional test cases were requested or added.

## Historical reference

The [previous development guide](archive/development-before-maintenance.md) is retained for source history; use current validation commands for this branch.

## Workspace ownership and native prerequisites

The root workspace builds its six checked-out support/exchange crates, rather than silently exercising their published registry copies. The incomplete Huobi crate remains outside the supported workspace because its original sibling `nash-ws` dependency is unavailable. Nash protocol/client are the latest published 0.3.0 sources with their original licenses and documented local exchange type identity patches: [protocol provenance](../vendor/nash-protocol/AYU-PATCH.md), [client provenance](../vendor/nash-native-client/AYU-PATCH.md).

On macOS, install GNU GMP and expose its native library directory before building:

```sh
export LIBRARY_PATH="$(brew --prefix gmp)/lib${LIBRARY_PATH:+:$LIBRARY_PATH}"
cargo build --workspace --all-targets --locked
cargo test --workspace --no-fail-fast --locked
```

Coinbase product size limits removed by the exchange are optional fields, preserving the distinction between absent limits and zero. Public order-book subscriptions use `level2_batch`, accepting its actual `level2_50` acknowledgement alias. Binance exchange metadata includes current side-specific percentage, notional, amendment and order-list filters as typed variants. Consult [Coinbase channel/schema documentation](https://docs.cdp.coinbase.com/exchange/websocket-feed/channels) and [Binance filter documentation](https://developers.binance.com/en/docs/products/spot/filters).

Authenticated account cases require the credentials declared in their existing fixtures. Their missing prerequisites are recorded separately from public-data failures in validation; they are not replaced with fabricated responses.
