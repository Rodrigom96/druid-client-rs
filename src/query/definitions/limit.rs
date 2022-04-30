use serde::{Deserialize, Serialize};

use super::ordering::OrderByColumn;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", rename = "default")]
pub struct Limit {
    pub limit: usize,
    pub columns: Vec<OrderByColumn>,
}
