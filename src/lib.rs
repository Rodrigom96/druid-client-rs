//! # Async rust client for Apache Druid
//!
//! Fully asynchronous, future-enabled [Apache Druid](http://druid.io/) client library for rust programming language.
//!
//! The library provides staticly typed API for [Native Queries](https://druid.apache.org/docs/latest/querying/querying.html)
//!
//! ## Installation
//! The library is hosted on [crates.io](https://crates.io/crates/druid-io/).
//!
//! ```toml
//! [dependencies]
//! druid-io = "*"
//! ```
//!
//! ## Supported Native Queries
//!
//! * Timeseries
//! * TopN
//! * GroupBy
//! * Scan
//! * Search
//! * TimeBoundary
//! * SegmentMetadata
//! * DataSourceMetadata
//!
//! ## Usage
//!
//! ### Client
//!
//! Connect to a druid cluster throughly staticly provided list of brokers:
//!
//! ```rust
//! use druid_io::client::DruidClientBuilder;
//!
//! let druid_client = DruidClientBuilder::new("http://localhost:8082").build();
//! ```
//!
//! ### Querying
//!
//! #### Timeseries
//!
//! See [Timeseries query documentation](https://druid.apache.org/docs/latest/querying/timeseriesquery.html)
//!
//! ```rust
//! use chrono::NaiveDate;
//! use druid_io::client::DruidClientBuilder;
//! use serde::Deserialize;
//! use serde::Serialize;
//! use druid_io::{
//!     query::timeseries::Timeseries,
//!     query::{
//!         definitions::{Aggregation, VirtualColumn},
//!         definitions::{
//!             Dimension, Filter, Granularity, Interval, Ordering, OutputType, SortingOrder,
//!             PostAggregation, PostAggregator
//!         },
//!         DataSource
//!     },
//! };
//!
//! #[derive(Serialize, Deserialize, Debug)]
//! pub struct TimeAggr {
//!     count: usize,
//!     count_fraction: f32,
//!     user: String,
//!     foo_user: String,
//! }
//!
//! let druid_client = DruidClientBuilder::new("http://localhost:8082").build();
//!
//! let timeseries = Timeseries {
//!     data_source: DataSource::table("wikipedia"),
//!     limit: Some(10),
//!     descending: false,
//!     granularity: Granularity::all(),
//!     filter: Some(Filter::selector("user", "Taffe316")),
//!     aggregations: vec![
//!         Aggregation::count("count"),
//!         Aggregation::string_first("user", "user", 1024),
//!         Aggregation::string_first("foo_user", "foo_user", 1024),
//!     ],
//!     post_aggregations: vec![PostAggregation::arithmetic(
//!         "count_fraction",
//!         "/",
//!         vec![
//!             PostAggregator::field_access("count_percent", "count"),
//!             PostAggregator::constant("hundred", 100.into()),
//!         ],
//!         None,
//!     )],
//!     virtual_columns: vec![VirtualColumn::expression(
//!         "foo_user",
//!         "concat('foo' + user)",
//!         OutputType::STRING,
//!     )],
//!     intervals: vec![Interval{
//!         from: NaiveDate::from_ymd(2015,9,12).and_hms_milli(8, 23, 32, 96),
//!         to: NaiveDate::from_ymd(2015,9,12).and_hms_milli(15, 36, 27, 96),
//!    }],
//!     context: Default::default(),
//! };
//! let result = druid_client.timeseries::<TimeAggr>(&timeseries);
//!
//! ```

extern crate serde_json;

pub mod client;
pub mod query;
pub mod serialization;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
