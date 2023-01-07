use super::definitions::Filter;
use super::definitions::Interval;
use super::definitions::Ordering;
use super::definitions::VirtualColumn;
use super::DataSource;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "queryType", rename = "scan")]
#[serde(rename_all = "camelCase")]
pub struct Scan {
    pub data_source: DataSource,
    pub intervals: Vec<Interval>,
    pub result_format: ResultFormat,
    pub filter: Option<Filter>,
    pub columns: Vec<String>,
    pub virtual_columns: Vec<VirtualColumn>,
    pub batch_size: usize,
    pub limit: Option<usize>,
    pub order: Ordering,
    pub context: std::collections::HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ResultFormat {
    List,
    CompactedList,
    ValueVector,
}

pub struct ScanBuilder {
    data_source: DataSource,
    intervals: Vec<Interval>,
    result_format: ResultFormat,
    filter: Option<Filter>,
    columns: Vec<String>,
    virtual_columns: Vec<VirtualColumn>,
    batch_size: usize,
    limit: Option<usize>,
    order: Ordering,
    context: std::collections::HashMap<String, String>,
}

impl ScanBuilder {
    pub fn new(data_source: DataSource) -> Self {
        ScanBuilder {
            data_source,
            intervals: vec![],
            result_format: ResultFormat::List,
            filter: None,
            columns: vec![],
            virtual_columns: vec![],
            batch_size: 20480,
            limit: None,
            order: Ordering::None,
            context: std::collections::HashMap::new(),
        }
    }

    pub fn intervals(mut self, intervals: Vec<Interval>) -> Self {
        self.intervals = intervals;
        self
    }

    pub fn result_format(mut self, result_format: ResultFormat) -> Self {
        self.result_format = result_format;
        self
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }

    pub fn columns(mut self, columns: Vec<String>) -> Self {
        self.columns = columns;
        self
    }

    pub fn virtual_columns(mut self, virtual_columns: Vec<VirtualColumn>) -> Self {
        self.virtual_columns = virtual_columns;
        self
    }

    pub fn batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn order(mut self, order: Ordering) -> Self {
        self.order = order;
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

    pub fn build(self) -> Scan {
        Scan {
            data_source: self.data_source,
            intervals: self.intervals,
            result_format: self.result_format,
            filter: self.filter,
            columns: self.columns,
            virtual_columns: self.virtual_columns,
            batch_size: self.batch_size,
            limit: self.limit,
            order: self.order,
            context: self.context,
        }
    }
}
