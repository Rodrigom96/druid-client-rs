use super::definitions::Filter;
use super::definitions::Granularity;
use super::definitions::Interval;
use super::definitions::VirtualColumn;
use super::definitions::PostAggregation;
use super::DataSource;
use crate::query::definitions::Aggregation;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "queryType", rename = "timeseries")]
pub struct Timeseries {
    pub data_source: DataSource,
    pub granularity: Granularity,
    pub descending: bool,
    pub intervals: Vec<Interval>,
    pub filter: Option<Filter>,
    pub aggregations: Vec<Aggregation>,
    pub post_aggregations: Vec<PostAggregation>,
    pub virtual_columns: Vec<VirtualColumn>,
    pub limit: Option<usize>,
    pub context: std::collections::HashMap<String, String>,
}
