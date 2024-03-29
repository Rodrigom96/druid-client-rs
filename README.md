# Async rust client for Apache Druid 

<div style="text-align:center"><img src="https://user-images.githubusercontent.com/502482/92421491-c26ab800-f146-11ea-80af-0da8ce4a457d.png" width="10%"/></div>

Fully asynchronous, future-enabled [Apache Druid](http://druid.io/) client library for rust programming language.

The library provides staticly typed API for [Native Queries](https://druid.apache.org/docs/latest/querying/querying.html) 

## Installation
The library is hosted on [crates.io](https://crates.io/crates/druid-io/).

```toml
[dependencies]
druid-io = "*"
```

## Supported Native Queries

* Timeseries
* TopN
* GroupBy
* Scan
* Search
* TimeBoundary
* SegmentMetadata
* DataSourceMetadata

## Usage

### Client

Connect to a druid cluster

```rust
let druid_client = DruidClientBuilder::new("http://localhost:8082").build();
```

### Querying

#### Timeseries

See [Timeseries query documentation](https://druid.apache.org/docs/latest/querying/timeseriesquery.html)

```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct TimeAggr {
    count: usize,
    count_fraction: f32,
    user: String,
}

let timeseries = Timeseries {
    data_source: DataSource::table("wikipedia"),
    limit: Some(10),
    descending: false,
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
    virtual_columns: vec![],
    intervals: vec![Interval{
        from: NaiveDate::from_ymd(2015,9,12).and_hms_milli(8, 23, 32, 96),
        to: NaiveDate::from_ymd(2015,9,12).and_hms_milli(15, 36, 27, 96),
    }],
    context: context,
};
let result = tokio_test::block_on(druid_client.timeseries::<TimeAggr>(&timeseries));

```

#### TopN
See [Apache Druid TopN query documentation](https://druid.apache.org/docs/latest/querying/topnquery.html)

```rust
#[derive(Serialize, Deserialize, Debug)]
struct WikiPage {
    page: String,
    user: Option<String>,
    count: usize,
}

let top_n = TopN {
    data_source: DataSource::table("wikipedia"),
    dimension: Dimension::default("page"),
    threshold: 10,
    metric: "count".into(),
    aggregations: vec![
        Aggregation::count("count"),
        Aggregation::string_first("user", "user", 1024),
    ],
    virtual_columns: vec![],
    intervals: vec![Interval{
        from: NaiveDate::from_ymd(2015,9,12).and_hms_milli(8, 23, 32, 96),
        to: NaiveDate::from_ymd(2015,9,12).and_hms_milli(15, 36, 27, 96),
    }],
    granularity: Granularity::all(),
    context: Default::default(),
};
let druid_client = DruidClientBuilder::new("http://localhost:8082").build();
let result = tokio_test::block_on(druid_client.top_n::<WikiPage>(&top_n));

```

#### GroupBy
See [Apache Druid GroupBy query documentation](https://druid.apache.org/docs/latest/querying/groupbyquery.html)

```rust

let group_by = GroupBy {
    data_source: DataSource::table("wikipedia"),
    dimensions: vec![Dimension::Default {
        dimension: "page".into(),
        output_name: "page".into(),
        output_type: OutputType::STRING,
    }],
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
    virtual_columns: vec![],
    having: Some(Having::greater_than("count_fraction", 0.01.into())),
    intervals: vec![Interval{
        from: NaiveDate::from_ymd(2015,9,12).and_hms_milli(8, 23, 32, 96),
        to: NaiveDate::from_ymd(2015,9,12).and_hms_milli(15, 36, 27, 96),
    }],
    subtotal_spec: Default::default(),
    context: Default::default(),
};
let result = tokio_test::block_on(druid_client.group_by::<WikiPage>(&group_by));

```

#### Scan (with inner join)
See [Apache Druid TimeBoundary query documentation](https://druid.apache.org/docs/latest/querying/scan-query.html)

Let's try something more complex: inner join

```rust
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ScanEvent {
    #[serde(rename(deserialize = "__time"))]
    time: usize,
    city_name: Option<String>,
    comment: Option<String>,
    namespace: Option<String>,
    page: Option<String>,
    region_iso_code: Option<String>,
    user: String,
    #[serde(rename(deserialize = "c.languages"))]
    languages: Option<String>,
}

let scan = ScanBuilder::new(
    DataSource::join(JoinType::Inner)
        .left(DataSource::table("wikipedia"))
        .right(
            DataSource::query(
                ScanBuilder::new(DataSource::table("countries"))
                    .batch_size(10)
                    .intervals(vec![Interval {
                        from: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(8, 23, 32, 96),
                        to: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(15, 36, 27, 96),
                    }])
                    .columns(vec!["Name".into(), "languages".into()])
                    .build()
                    .into(),
            ),
            "c.",
        )
        .condition("countryName == \"c.Name\"")
        .build()
        .unwrap(),
    )
    .batch_size(10)
    .intervals(vec![Interval {
        from: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(8, 23, 32, 96),
        to: NaiveDate::from_ymd(2015, 9, 12).and_hms_milli(15, 36, 27, 96),
    }])
    .limit(10)
    .build();

let result = tokio_test::block_on(druid_client.scan::<ScanEvent>(&scan));

```
