use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;
use super::shared::{string_to_decimal, string_to_opt_decimal};

/// This struct represents a product
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: String,
    pub display_name: String,
    pub quote_currency: String,
    pub base_currency: String,
    #[serde(with = "string_to_decimal")]
    pub base_increment: Decimal,
    #[serde(with = "string_to_decimal")]
    pub quote_increment: Decimal,
    #[serde(default, with = "string_to_opt_decimal")]
    pub base_min_size: Option<Decimal>,
    #[serde(default, with = "string_to_opt_decimal")]
    pub base_max_size: Option<Decimal>,
    pub min_market_funds: String,
    #[serde(default)]
    pub max_market_funds: Option<String>,
    pub status: String,
    pub status_message: String,
    pub cancel_only: bool,
    pub limit_only: bool,
    pub post_only: bool,
    pub trading_disabled: bool,
}