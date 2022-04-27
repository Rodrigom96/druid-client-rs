use super::definitions::Filter;
use super::DataSource;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "queryType", rename = "timeBoundary")]
#[serde(rename_all = "camelCase")]
pub struct TimeBoundary {
    pub data_source: DataSource,
    #[serde(skip_serializing_if = "TimeBoundType::is_both")]
    pub bound: TimeBoundType,
    pub filter: Option<Filter>,
    pub context: std::collections::HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TimeBoundType {
    MaxTime,
    MinTime,
    MinMaxTime,
}

impl TimeBoundType {
    pub fn is_both(&self) -> bool {
        matches!(self, TimeBoundType::MinMaxTime)
    }
}
