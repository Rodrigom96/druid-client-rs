extern crate druid_io;
extern crate tokio;

use chrono::NaiveDate;
use druid_io::{
    client::DruidClientBuilder,
    query::{
        definitions::VirtualColumn,
        definitions::{Interval, OutputType},
        scan::{ResultFormat, ScanBuilder},
        DataSource, JoinType,
    },
};
use serde::Deserialize;
use serde::Serialize;

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
    .result_format(ResultFormat::List)
    .virtual_columns(vec![VirtualColumn::expression(
        "foo_page",
        "concat('foo' + page)",
        OutputType::STRING,
    )])
    .limit(10)
    .build();

    let druid_client = DruidClientBuilder::new("http://localhost:8082").build();
    let result = tokio_test::block_on(druid_client.scan::<ScanEvent>(&scan));
    println!("{:?}", result.unwrap());
}
