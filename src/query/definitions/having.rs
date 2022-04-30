use serde::{Deserialize, Serialize};

use super::super::{JsonAny, JsonNumber};
use crate::query::definitions::{Dimension, Filter};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Having {
    Filter {
        filter: Filter,
    },
    GreaterThan {
        aggregation: String,
        value: JsonNumber,
    },
    EqualTo {
        aggregation: String,
        value: JsonNumber,
    },
    LessThan {
        aggregation: String,
        value: JsonNumber,
    },
    DimSelector {
        dimension: Dimension,
        value: JsonAny,
    }, //todo
    #[serde(rename_all = "camelCase")]
    And {
        having_specs: Vec<Having>,
    },
    #[serde(rename_all = "camelCase")]
    Or {
        having_specs: Vec<Having>,
    },
    #[serde(rename_all = "camelCase")]
    Not {
        having_specs: Box<Having>,
    },
}

impl Having {
    pub fn filter(filter: Filter) -> Self {
        Having::Filter { filter }
    }
    pub fn greater_than(aggregation: &str, value: JsonNumber) -> Self {
        Having::GreaterThan {
            aggregation: aggregation.to_string(),
            value,
        }
    }
    pub fn equal_to(aggregation: &str, value: JsonNumber) -> Self {
        Having::EqualTo {
            aggregation: aggregation.to_string(),
            value,
        }
    }
    pub fn less_than(aggregation: &str, value: JsonNumber) -> Self {
        Having::LessThan {
            aggregation: aggregation.to_string(),
            value,
        }
    }
}
