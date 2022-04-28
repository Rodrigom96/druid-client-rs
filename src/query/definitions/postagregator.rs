use serde::{Deserialize, Serialize};

use super::super::JsonAny;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum PostAggregation {
    #[serde(rename_all = "camelCase")]
    Arithmetic {
        name: String,
        #[serde(rename(serialize = "fn"))]
        function: String,
        fields: Vec<PostAggregator>,
        ordering: Option<String>,
    },
    DoubleGreatest {
        name: String,
        fields: Vec<PostAggregation>,
    },
    LongGreatest {
        name: String,
        fields: Vec<PostAggregation>,
    },
    LongLeast {
        name: String,
        fields: Vec<PostAggregation>,
    },
    DoubleLeast {
        name: String,
        fields: Vec<PostAggregation>,
    },
    #[serde(rename_all = "camelCase")]
    Javascript {
        name: String,
        field_names: Vec<String>,
        function: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum PostAggregator {
    #[serde(rename_all = "camelCase")]
    FieldAccess { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    FinalizingFieldAccess { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    Constant { name: String, value: JsonAny },
    #[serde(rename_all = "camelCase")]
    HyperUniqueCardinality { field_name: String },
}

impl PostAggregator {
    pub fn field_access(name: &str, field_name: &str) -> Self {
        PostAggregator::FieldAccess {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn finalized_field_access(name: &str, field_name: &str) -> Self {
        PostAggregator::FinalizingFieldAccess {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn constant(name: &str, value: JsonAny) -> Self {
        PostAggregator::Constant {
            name: name.to_string(),
            value,
        }
    }
    pub fn hyper_unique_cardinality(field_name: &str) -> Self {
        PostAggregator::HyperUniqueCardinality {
            field_name: field_name.to_string(),
        }
    }
}
