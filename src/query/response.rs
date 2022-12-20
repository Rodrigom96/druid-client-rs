use super::{definitions::Granularity, JsonAny};
use crate::serialization::default_for_null;
use crate::serialization::tagged_or_untagged;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct DruidListResponse<T: DeserializeOwned> {
    pub timestamp: String,
    #[serde(bound(deserialize = ""))]
    pub result: Vec<T>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MetadataResponse<T: DeserializeOwned> {
    pub timestamp: String,
    #[serde(bound(deserialize = ""))]
    pub result: T,
}

pub type TopNResponse<T> = DruidListResponse<T>;

#[derive(Deserialize, Serialize, Debug)]
pub struct GroupByResponse<T: DeserializeOwned> {
    pub timestamp: String,
    #[serde(bound(deserialize = ""))]
    pub event: T,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DimValue {
    pub dimension: String,
    pub value: JsonAny,
    pub count: usize,
}

pub type SearchResponse = DruidListResponse<DimValue>;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScanResponse<T: DeserializeOwned> {
    pub segment_id: String,
    pub columns: Vec<String>,
    #[serde(bound(deserialize = ""))]
    pub events: Vec<T>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MinMaxTime {
    pub max_time: Option<String>,
    pub min_time: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeBoundaryResponse {
    timestamp: String,
    result: MinMaxTime,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ColumnDefinition {
    #[serde(rename(deserialize = "type"))]
    column_type: String,
    has_multiple_values: bool,
    size: usize,
    cardinality: Option<f32>,
    min_value: Option<JsonAny>,
    max_value: Option<JsonAny>,
    error_message: Option<String>,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AggregatorDefinition {
    #[serde(rename(deserialize = "type"))]
    aggr_type: String,
    name: String,
    field_name: String,
    expression: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimestampSpec {
    column: String,
    format: String,
    missing_value: Option<String>,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SegmentMetadataResponse {
    id: String,
    #[serde(default, deserialize_with = "default_for_null")]
    intervals: Vec<String>,
    columns: HashMap<String, ColumnDefinition>,

    #[serde(deserialize_with = "tagged_or_untagged")]
    query_granularity: Granularity,
    rollup: Option<bool>,
    size: Option<usize>,
    num_rows: Option<usize>,
    timestamp_spec: TimestampSpec,
    #[serde(default, deserialize_with = "default_for_null")]
    aggregators: HashMap<String, AggregatorDefinition>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeseriesResponse<T: DeserializeOwned> {
    timestamp: Option<String>,
    #[serde(bound(deserialize = ""))]
    result: T,
}
