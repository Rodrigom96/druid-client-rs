use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Ordering {
    Ascending,
    Descending,
    None,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SortingOrder {
    Lexicographic,
    Alphanumeric,
    Strlen,
    Numeric,
}
