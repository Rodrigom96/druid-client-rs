use serde::{Deserialize, Serialize};

use super::filters::Filter;
use super::hll_sketch::HllType;

#[rustfmt::skip]
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Aggregation {
    Count { name: String },
    #[serde(rename_all = "camelCase")]
    LongSum { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    DoubleSum { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    FloatSum { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    LongMax { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    DoubleMax { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    FloatMax { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    LongMin { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    FloatMin { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    DoubleMin { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    LongFirst { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    FloatFirst { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    DoubleFirst { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    LongLast { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    FloatLast { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    DoubleLast { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    StringFirst { name: String, field_name: String, max_string_bytes: usize },
    #[serde(rename_all = "camelCase")]
    StringLast { name: String, field_name: String, max_string_bytes: usize },

    #[serde(rename_all = "camelCase")]
    DoubleAny { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    FloatAny { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    LongAny { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    StringAny { name: String, field_name: String },

    #[serde(rename_all = "camelCase")]
    Javascript { name: String, field_names: Vec<String>, fn_aggregate: String, fn_combine: String, fn_reset: String},
    
    #[serde(rename_all = "camelCase")]
    ThetaSketch {name: String, field_name: String, is_input_theta_sketch: bool, size: usize},


    #[serde(rename_all = "camelCase")]
    HLLSketchBuild { name: String, field_name: String, lg_k: usize, lgt_hll_type: HllType, round: bool},

    #[serde(rename_all = "camelCase")]
    Cardinality { name: String, fields: Vec<String>, by_row: bool, round: bool},

    #[serde(rename_all = "camelCase")]
    HyperUnique { name: String, field_name: String, is_input_hyper_unique: bool, round: bool},

    Filtered { filter: Filter, aggregator: Box<Aggregation>}
}

impl Aggregation {
    pub fn count(name: &str) -> Aggregation {
        Aggregation::Count {
            name: name.to_string(),
        }
    }
    pub fn long_sum(name: &str, field_name: &str) -> Aggregation {
        Aggregation::LongSum {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn double_sum(name: &str, field_name: &str) -> Aggregation {
        Aggregation::DoubleSum {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn float_sum(name: &str, field_name: &str) -> Aggregation {
        Aggregation::FloatSum {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn long_max(name: &str, field_name: &str) -> Aggregation {
        Aggregation::LongMax {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn double_max(name: &str, field_name: &&str) -> Aggregation {
        Aggregation::DoubleMax {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn float_max(name: &str, field_name: &str) -> Aggregation {
        Aggregation::FloatMax {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn long_min(name: &str, field_name: &str) -> Aggregation {
        Aggregation::LongMin {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn float_min(name: &str, field_name: &str) -> Aggregation {
        Aggregation::FloatMin {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn double_min(name: &str, field_name: &str) -> Aggregation {
        Aggregation::DoubleMin {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn long_first(name: &str, field_name: &str) -> Aggregation {
        Aggregation::LongFirst {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn float_first(name: &str, field_name: &str) -> Aggregation {
        Aggregation::FloatFirst {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn double_first(name: &str, field_name: &str) -> Aggregation {
        Aggregation::DoubleFirst {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn long_last(name: &str, field_name: &str) -> Aggregation {
        Aggregation::LongLast {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn float_last(name: &str, field_name: &str) -> Aggregation {
        Aggregation::FloatLast {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn double_last(name: &str, field_name: &str) -> Aggregation {
        Aggregation::DoubleLast {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn string_first(name: &str, field_name: &str, max_string_bytes: usize) -> Aggregation {
        Aggregation::StringFirst {
            name: name.to_string(),
            field_name: field_name.to_string(),
            max_string_bytes,
        }
    }
    pub fn string_last(name: &str, field_name: &str, max_string_bytes: usize) -> Aggregation {
        Aggregation::StringLast {
            name: name.to_string(),
            field_name: field_name.to_string(),
            max_string_bytes,
        }
    }
}
