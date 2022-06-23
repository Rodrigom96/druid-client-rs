use super::definitions::Dimension;
use super::definitions::Filter;
use super::definitions::{Granularity, GranularityTyped};
use super::definitions::Interval;
use super::definitions::VirtualColumn;
use super::DataSource;
use crate::query::definitions::Aggregation;
use crate::query::definitions::Having;
use crate::query::definitions::PostAggregation;
use crate::query::definitions::Limit;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "queryType", rename = "groupBy")]
pub struct GroupBy {
    pub data_source: DataSource,
    pub dimensions: Vec<Dimension>,
    pub limit_spec: Option<Limit>,
    pub having: Option<Having>,
    pub granularity: Granularity,
    pub filter: Option<Filter>,
    pub aggregations: Vec<Aggregation>,
    pub post_aggregations: Vec<PostAggregation>,
    pub virtual_columns: Vec<VirtualColumn>,
    pub intervals: Vec<Interval>,
    pub subtotal_spec: Vec<Vec<String>>,
    pub context: std::collections::HashMap<String, String>,
}

pub struct GroupByBuilder {
    data_source: DataSource,
    dimensions: Vec<Dimension>,
    limit_spec: Option<Limit>,
    having: Option<Having>,
    granularity: Granularity,
    filter: Option<Filter>,
    aggregations: Vec<Aggregation>,
    post_aggregations: Vec<PostAggregation>,
    virtual_columns: Vec<VirtualColumn>,
    intervals: Vec<Interval>,
    subtotal_spec: Vec<Vec<String>>,
    context: std::collections::HashMap<String, String>,
}

impl GroupByBuilder {
    pub fn new(data_source: DataSource) -> Self {
        GroupByBuilder {
            data_source,
            dimensions: vec![],
            limit_spec: None,
            having: None,
            granularity: Granularity::all(),
            filter: None,
            aggregations: vec![],
            post_aggregations: vec![],
            virtual_columns: vec![],
            intervals: vec![],
            subtotal_spec: vec![],
            context: std::collections::HashMap::new(),
        }
    }
    pub fn dimensions(mut self, dimensions: Vec<Dimension>) -> Self {
        self.dimensions = dimensions;
        self
    }
    pub fn limit(mut self, limit: Limit) -> Self {
        self.limit_spec = Some(limit);
        self
    }
    pub fn having(mut self, having: Having) -> Self {
        self.having = Some(having);
        self
    }
    pub fn granularity(mut self, granularity: Granularity) -> Self {
        self.granularity = granularity;
        self
    }
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }
    pub fn aggregations(mut self, aggr: Vec<Aggregation>) -> Self {
        self.aggregations = aggr;
        self
    }
    pub fn post_aggregations(mut self, aggr: Vec<PostAggregation>) -> Self {
        self.post_aggregations = aggr;
        self
    }
    pub fn virtual_columns(mut self, columns: Vec<VirtualColumn>) -> Self {
        self.virtual_columns = columns;
        self
    }
    pub fn intervals(mut self, intervals: Vec<Interval>) -> Self {
        self.intervals = intervals;
        self
    }
    pub fn subtotal_spec(mut self, subtotals: Vec<Vec<String>>) -> Self {
        self.subtotal_spec = subtotals;
        self
    }
    pub fn context(mut self, context: std::collections::HashMap<String, String>) -> Self {
        self.context = context;
        self
    }

    pub fn add_context(mut self, key: &str, value: &str) -> Self {
        self.context.insert(key.to_string(), value.to_string());
        self
    }
    pub fn build(self) -> GroupBy {
        GroupBy {
            data_source: self.data_source,
            dimensions: self.dimensions,
            limit_spec: self.limit_spec,
            having: self.having,
            granularity: self.granularity,
            filter: self.filter,
            aggregations: self.aggregations,
            post_aggregations: self.post_aggregations,
            virtual_columns: self.virtual_columns,
            intervals: self.intervals,
            subtotal_spec: self.subtotal_spec,
            context: self.context,
        }
    }
}
