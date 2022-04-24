use super::definitions::Aggregation;
use super::definitions::Dimension;
use super::definitions::Granularity;
use super::definitions::Interval;
use super::definitions::VirtualColumn;
use super::DataSource;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "queryType", rename = "topN")]
pub struct TopN {
    // todo: data_source would result in weird error message
    pub data_source: DataSource,
    pub dimension: Dimension,
    pub threshold: usize,
    pub metric: String,
    pub aggregations: Vec<Aggregation>,
    pub virtual_columns: Vec<VirtualColumn>,
    pub intervals: Vec<Interval>,
    pub granularity: Granularity,
    pub context: std::collections::HashMap<String, String>,
}
