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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderByColumn {
    pub dimension: String,
    pub direction: Ordering,
    pub dimension_order: SortingOrder,
}

impl OrderByColumn {
    pub fn new(dimension: &str, direction: Ordering, dimension_order: SortingOrder) -> Self {
        OrderByColumn {
            dimension: dimension.to_string(),
            direction,
            dimension_order,
        }
    }
}
