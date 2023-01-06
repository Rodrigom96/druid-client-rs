extern crate druid_io;
extern crate tokio;

use chrono::NaiveDate;
use druid_io::{
    client::DruidClientBuilder,
    query::search::Search,
    query::timeseries::Timeseries,
    query::top_n::TopN,
    query::{
        definitions::{Aggregation, VirtualColumn},
        definitions::{
            Dimension, Filter, Granularity, Having, Interval, Limit, OrderByColumn, Ordering,
            OutputType, PostAggregation, PostAggregator, SortingOrder,
        },
        group_by::{GroupBy, GroupByBuilder},
        scan::{ResultFormat, Scan},
        search::SearchQuerySpec,
        segment_metadata::{AnalysisType, SegmentMetadata, ToInclude},
        time_boundary::{TimeBoundType, TimeBoundary},
        DataSource, JoinType,
    },
};
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct WikiPage {
    page: String,
    foo_page: String,
    user: Option<String>,
    count: usize,
}

#[test]
fn test_top_n_query() {
    let mut context = HashMap::new();
    context.insert("resultAsArray".to_string(), "true".to_string());
    let top_n = TopN {
        data_source: DataSource::table("wikipedia"),
        dimension: Dimension::default("page"),
        threshold: 10,
        metric: "count".into(),
        aggregations: vec![
            Aggregation::count("count"),
            Aggregation::string_first("user", "user", 1024),
            Aggregation::string_first("foo_page", "foo_page", 1024),
        ],
        virtual_columns: vec![VirtualColumn::expression(
            "foo_page",
            "concat('foo' + page)",
            OutputType::STRING,
        )],
        intervals: vec![Interval {
            from: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(8, 23, 32, 96),
            to: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(15, 36, 27, 96),
        }],
        granularity: Granularity::all(),
        context: context,
    };
    let druid_client = DruidClientBuilder::new("http://localhost:8082").build();
    let result = tokio_test::block_on(druid_client.top_n::<WikiPage>(&top_n));
    println!("{:?}", result.unwrap());
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ScanEvent {
    #[serde(rename(deserialize = "__time"))]
    time: usize,
    city_name: Option<String>,
    comment: Option<String>,
    namespace: Option<String>,
    page: Option<String>,
    foo_page: Option<String>,
    region_iso_code: Option<String>,
    user: String,

    #[serde(rename(deserialize = "c.languages"))]
    languages: Option<String>,
    // count: usize,
}
#[test]
fn test_scan_join() {
    let scan = Scan {
        data_source: DataSource::join(JoinType::Inner)
            .left(DataSource::table("wikipedia"))
            .right(
                DataSource::query(
                    Scan {
                        data_source: DataSource::table("countries"),
                        batch_size: 10,
                        intervals: vec![Interval {
                            from: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(8, 23, 32, 96),
                            to: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(15, 36, 27, 96),
                        }],
                        result_format: ResultFormat::List,
                        columns: vec!["Name".into(), "languages".into()],
                        virtual_columns: vec![],
                        limit: None,
                        filter: None,
                        ordering: Some(Ordering::None),
                        context: std::collections::HashMap::new(),
                    }
                    .into(),
                ),
                "c.",
            )
            .condition("countryName == \"c.Name\"")
            .build()
            .unwrap(),
        batch_size: 10,
        intervals: vec![Interval {
            from: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(8, 23, 32, 96),
            to: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(15, 36, 27, 96),
        }],
        result_format: ResultFormat::List,
        columns: vec![],
        virtual_columns: vec![VirtualColumn::expression(
            "foo_page",
            "concat('foo' + page)",
            OutputType::STRING,
        )],
        limit: Some(10),
        filter: None,
        ordering: Some(Ordering::None),
        context: std::collections::HashMap::new(),
    };

    let druid_client = DruidClientBuilder::new("http://localhost:8082").build();
    let result = tokio_test::block_on(druid_client.scan::<ScanEvent>(&scan));
    println!("{:?}", result.unwrap());
}
#[test]
fn test_group_by() {
    let group_by = GroupBy {
        data_source: DataSource::table("wikipedia"),
        dimensions: vec![
            Dimension::Default {
                dimension: "page".into(),
                output_name: "page".into(),
                output_type: OutputType::STRING,
            },
            Dimension::Default {
                dimension: "foo_page".into(),
                output_name: "foo_page".into(),
                output_type: OutputType::STRING,
            },
        ],
        limit_spec: Some(Limit {
            limit: 10,
            columns: vec![OrderByColumn::new(
                "page",
                Ordering::Descending,
                SortingOrder::Alphanumeric,
            )],
        }),
        granularity: Granularity::all(),
        filter: Some(Filter::selector("user", "Taffe316")),
        aggregations: vec![
            Aggregation::count("count"),
            Aggregation::string_first("user", "user", 1024),
        ],
        post_aggregations: vec![PostAggregation::arithmetic(
            "count_fraction",
            "/",
            vec![
                PostAggregator::field_access("count_percent", "count"),
                PostAggregator::constant("hundred", 100.into()),
            ],
            None,
        )],
        virtual_columns: vec![VirtualColumn::expression(
            "foo_page",
            "concat('foo' + page)",
            OutputType::STRING,
        )],
        having: Some(Having::greater_than("count_fraction", 0.01.into())),
        intervals: vec![Interval {
            from: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(8, 23, 32, 96),
            to: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(15, 36, 27, 96),
        }],
        subtotal_spec: Default::default(),
        context: Default::default(),
    };
    let druid_client = DruidClientBuilder::new("http://localhost:8082").build();
    let result = tokio_test::block_on(druid_client.group_by::<WikiPage>(&group_by));
    println!("{:?}", result.unwrap());
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeAggr {
    count: usize,
    count_ololo: f32,
    user: String,
    foo_user: String,
}
#[test]
fn test_timeseries() {
    let mut context = HashMap::new();
    context.insert("grandTotal".to_string(), "true".to_string());

    let timeseries = Timeseries {
        data_source: DataSource::table("wikipedia"),
        limit: Some(10),
        descending: false,
        granularity: Granularity::all(),
        filter: None, //Some(Filter::selector("user", "Taffe316")),
        aggregations: vec![
            Aggregation::count("count"),
            Aggregation::string_first("user", "user", 1024),
            Aggregation::string_first("foo_user", "foo_user", 1024),
        ],
        post_aggregations: vec![PostAggregation::arithmetic(
            "count_ololo",
            "/",
            vec![
                PostAggregator::field_access("count_percent", "count"),
                PostAggregator::constant("hundred", 100.into()),
            ],
            None,
        )],
        virtual_columns: vec![VirtualColumn::expression(
            "foo_user",
            "concat('foo' + user)",
            OutputType::STRING,
        )],
        intervals: vec![Interval {
            from: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(8, 23, 32, 96),
            to: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(15, 36, 27, 96),
        }],
        context: context,
    };
    let druid_client = DruidClientBuilder::new("http://localhost:8082").build();
    let result = tokio_test::block_on(druid_client.timeseries::<TimeAggr>(&timeseries));
    println!("{:?}", result.unwrap());
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    count: usize,
    count_ololo: f32,
    title: String,
    user: String,
    foo_page: String,
}
#[test]
fn test_group_by_builder() {
    let group_by = GroupByBuilder::new(DataSource::table("wikipedia"))
        .dimensions(vec![
            Dimension::Default {
                dimension: "page".into(),
                output_name: "title".into(),
                output_type: OutputType::STRING,
            },
            Dimension::Default {
                dimension: "foo_page".into(),
                output_name: "foo_page".into(),
                output_type: OutputType::STRING,
            },
        ])
        .limit(Limit {
            limit: 10,
            columns: vec![OrderByColumn::new(
                "title",
                Ordering::Descending,
                SortingOrder::Alphanumeric,
            )],
        })
        .having(Having::greater_than("count_ololo", 0.001.into()))
        .filter(Filter::selector("user", "Taffe316"))
        .aggregations(vec![
            Aggregation::count("count"),
            Aggregation::string_first("user", "user", 1024),
        ])
        .post_aggregations(vec![PostAggregation::arithmetic(
            "count_ololo",
            "/",
            vec![
                PostAggregator::field_access("count_percent", "count"),
                PostAggregator::constant("hundred", 100.into()),
            ],
            None,
        )])
        .virtual_columns(vec![VirtualColumn::expression(
            "foo_page",
            "concat('foo' + page)",
            OutputType::STRING,
        )])
        .intervals(vec![Interval {
            from: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(8, 23, 32, 96),
            to: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(15, 36, 27, 96),
        }])
        .add_context("groupByStrategy", "v2")
        // .add_context("resultAsArray", "true")
        .build();
    let druid_client = DruidClientBuilder::new("http://localhost:8082").build();
    let result = tokio_test::block_on(druid_client.group_by::<Page>(&group_by));
    println!("{:?}", result.unwrap());
}

#[test]
fn test_search() {
    let search = Search {
        data_source: DataSource::table("wikipedia"),
        search_dimensions: vec!["page".into(), "user".into()],
        query: SearchQuerySpec::contains_insensitive("500"),
        sort: None,
        filter: None,
        limit: 20,
        intervals: vec![Interval {
            from: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(8, 23, 32, 96),
            to: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(15, 36, 27, 96),
        }],
        context: Default::default(),
        granularity: Granularity::all(),
    };
    let druid_client = DruidClientBuilder::new("http://localhost:8082").build();
    let result = tokio_test::block_on(druid_client.search::<WikiPage>(&search));
    println!("{:?}", result.unwrap());
}
#[test]
fn test_time_boundary() {
    let top_n = TimeBoundary {
        data_source: DataSource::table("wikipedia"),
        filter: None,
        context: Default::default(),
        bound: TimeBoundType::MinMaxTime,
    };
    let druid_client = DruidClientBuilder::new("http://localhost:8082").build();
    let result = tokio_test::block_on(druid_client.time_boundary::<WikiPage>(&top_n));
    println!("{:?}", result.unwrap());
}
#[test]
fn test_data_source_metadata() {
    let druid_client = DruidClientBuilder::new("http://localhost:8082").build();
    let result =
        tokio_test::block_on(druid_client.datasource_metadata(DataSource::table("wikipedia")));
    println!("{:?}", result.unwrap());
}
#[test]
fn test_segment_metadata() {
    let segment_query = SegmentMetadata {
        data_source: DataSource::table("countries"),
        intervals: vec![Interval {
            from: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(8, 23, 32, 96),
            to: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(15, 36, 27, 96),
        }],
        to_include: ToInclude::All,
        merge: false,
        analysis_types: vec![
            AnalysisType::Minmax,
            AnalysisType::Size,
            AnalysisType::Interval,
            AnalysisType::TimestampSpec,
            AnalysisType::QueryGranularity,
            AnalysisType::Aggregators,
            AnalysisType::Rollup,
            AnalysisType::Cardinality,
        ],
        lenient_aggregator_merge: false,
    };

    let druid_client = DruidClientBuilder::new("http://localhost:8082").build();
    let result = tokio_test::block_on(druid_client.segment_metadata(&segment_query));
    println!("{:?}", result.unwrap());
}

#[test]
fn test_client_builder() {
    let segment_query = SegmentMetadata {
        data_source: DataSource::table("countries"),
        intervals: vec![Interval {
            from: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(8, 23, 32, 96),
            to: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(15, 36, 27, 96),
        }],
        to_include: ToInclude::All,
        merge: false,
        analysis_types: vec![
            AnalysisType::Minmax,
            AnalysisType::Size,
            AnalysisType::Interval,
            AnalysisType::TimestampSpec,
            AnalysisType::QueryGranularity,
            AnalysisType::Aggregators,
            AnalysisType::Rollup,
            AnalysisType::Cardinality,
        ],
        lenient_aggregator_merge: false,
    };

    let client = ClientBuilder::new(Client::new()).build();
    let druid_client = DruidClientBuilder::new("http://localhost:8082")
        .endpoint("druid/v2")
        .client(client)
        .build();
    let result = tokio_test::block_on(druid_client.segment_metadata(&segment_query));
    println!("{:?}", result.unwrap());
}
