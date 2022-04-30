use serde::{Deserialize, Serialize};

use super::granularitys::Granularity;
use super::lookup::LookupMap;

#[rustfmt::skip]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ExtractFN {
    #[serde(rename_all = "camelCase")]
    Regex { expr: String, index: usize, replace_missing_value: bool, replace_missing_value_with: Option<String>},
    #[serde(rename_all = "camelCase")]
    Partial { expr: String },
    // SearchQuery { query: SearchQuerySpec }
    #[serde(rename_all = "camelCase")]
    Substring { index: usize, length: Option<usize> },
    #[serde(rename_all = "camelCase")]
    Strlen,
    #[serde(rename_all = "camelCase")]
    TimeFormat { format: Option<String>, time_zone: Option<String>, locale: Option<String>, granularity: Option<Granularity>, as_millis: bool },
    #[serde(rename_all = "camelCase")]
    Time { time_format: String, result_format: String, joda: bool },
    #[serde(rename_all = "camelCase")]
    Javascript { function: String },
    #[serde(rename_all = "camelCase")]
    RegisteredLookup { lookup: String, retain_missing_value: bool },
    #[serde(rename_all = "camelCase")]
    Lookup { lookup: LookupMap, retain_missing_value: bool, injective: bool, replace_missing_value_with: String },

    #[serde(rename_all = "camelCase")]
    Cascade { extraction_fns: Vec<ExtractFN> },
    #[serde(rename_all = "camelCase")]
    StringFormat {format: String, null_handling: Option<NullHandling>},

    #[serde(rename_all = "camelCase")]
    Upper { locale: Option<String> },
    //todo
    #[serde(rename_all = "camelCase")]
    Lower { locale: Option<String> },

    #[serde(rename_all = "camelCase")]
    Bucket { size: usize, offset: usize },
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum NullHandling {
    NullString,
    EmptyString,
    ReturnNull,
}
