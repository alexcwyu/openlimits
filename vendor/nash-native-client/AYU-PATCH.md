# nash-native-client compatibility patch

Source: [crates.io nash-native-client 0.3.0](https://crates.io/crates/nash-native-client/0.3.0), copied from its published crate. Original sources and licensing headers are retained.

Only `Cargo.toml` changes: use the checked-out `openlimits-exchange` 0.3.1-alpha.0 package. The upstream Nash dependencies otherwise select the published 0.3.0 package, producing two incompatible identities for `MarketPair`, exchange errors, and order structures. Root Cargo patches apply this correction to both direct and transitive dependencies. No trading behavior is changed.
