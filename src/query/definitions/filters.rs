use serde::{Deserialize, Serialize};

use super::extraction_functions::ExtractFN;
use super::ordering::SortingOrder;

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Filter {
    #[serde(rename_all = "camelCase")]
    Selector {
        dimension: String,
        value: String,
        extract_fn: Option<ExtractFN>,
    },
    ColumnComparison {
        dimensions: Vec<String>,
    },
    Regex {
        dimension: String,
        pattern: String,
    },
    And {
        fields: Vec<Filter>,
    },
    Or {
        fields: Vec<Filter>,
    },
    Not {
        field: Box<Filter>,
    },
    Javascript {
        dimension: String,
        function: String,
    },
    Search {
        dimension: String,
        query: FilterQuerySpec,
    },
    In {
        dimension: String,
        values: Vec<String>,
    },
    #[serde(rename_all = "camelCase")]
    Like {
        dimension: String,
        pattern: String,
        escape: Option<String>,
        extraction_fn: Option<ExtractFN>,
    },
    #[serde(rename_all = "camelCase")]
    Bound {
        dimension: String,
        lower: String,
        upper: String,
        lower_strict: bool,
        upper_strict: bool,
        ordering: SortingOrder,
        extraction_fn: Option<ExtractFN>,
    },
    #[serde(rename_all = "camelCase")]
    Interval {
        dimension: String,
        intervals: Vec<String>,
        extraction_fn: Option<ExtractFN>,
    },
    True,
}

impl Filter {
    pub fn selector(dimension: &str, value: &str) -> Filter {
        Filter::Selector {
            dimension: dimension.to_string(),
            value: value.to_string(),
            extract_fn: None,
        }
    }

    pub fn column_comparison(dimensions: Vec<&str>) -> Self {
        Filter::ColumnComparison {
            dimensions: dimensions.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn regex(dimension: &str, pattern: &str) -> Self {
        Filter::Regex {
            dimension: dimension.to_string(),
            pattern: pattern.to_string(),
        }
    }

    pub fn javascript(dimension: &str, javascript: &str) -> Self {
        Filter::Javascript {
            dimension: dimension.to_string(),
            function: javascript.to_string(),
        }
    }

    pub fn in_values(dimension: &str, values: Vec<&str>) -> Self {
        Filter::In {
            dimension: dimension.to_string(),
            values: values.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn like(dimension: &str, pattern: &str) -> Self {
        Filter::Like {
            dimension: dimension.to_string(),
            pattern: pattern.to_string(),
            escape: None,
            extraction_fn: None,
        }
    }
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum FilterQuerySpec {
    #[serde(rename_all = "camelCase")]
    Contains { value: String, case_sensitive: bool },
    #[serde(rename_all = "camelCase")]
    InsensitiveContains { value: String },
    #[serde(rename_all = "camelCase")]
    Fragment {
        values: Vec<String>,
        case_sensitive: bool,
    },
}
