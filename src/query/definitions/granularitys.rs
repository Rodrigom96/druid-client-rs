use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Granularity {
    All,
    None,
    Second,
    Minute,
    FifteenMinute,
    ThirtyMinute,
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
    Duration { duration: usize },
    Null,
}

impl Default for Granularity {
    fn default() -> Self {
        Granularity::Null
    }
}
