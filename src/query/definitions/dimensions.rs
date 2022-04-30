use serde::{Deserialize, Serialize};

use super::extraction_functions::ExtractFN;
use super::lookup::LookupMap;
use super::output_types::OutputType;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Dimension {
    #[serde(rename_all = "camelCase")]
    Default {
        dimension: String,
        output_name: String,
        output_type: OutputType,
    },
    #[serde(rename_all = "camelCase")]
    Extraction {
        dimenstion: String,
        output_name: String,
        output_type: OutputType,
        extraction_fn: ExtractFN,
    },
    #[serde(rename_all = "camelCase")]
    ListFiltered {
        delegate: Box<Dimension>,
        values: Vec<String>,
        is_whitelist: bool,
    },

    #[serde(rename_all = "camelCase")]
    RegexFiltered {
        delegate: Box<Dimension>,
        pattern: String,
    },
    #[serde(rename_all = "camelCase")]
    PrefixFiltered {
        delegate: Box<Dimension>,
        prefix: String,
    },
    #[serde(rename_all = "camelCase")]
    #[serde(rename(serialize = "lookup"))]
    LookupMap {
        dimension: String,
        output_name: String,
        replace_missing_value_with: String,
        retain_missing_value: bool,
        lookup: LookupMap,
    },

    Lookup {
        dimension: String,
        output_name: String,
        name: String,
    },
}

// todo: macro
impl Dimension {
    pub fn default(dimension: &str) -> Dimension {
        Dimension::Default {
            dimension: dimension.into(),
            output_name: dimension.into(),
            output_type: OutputType::STRING,
        }
    }

    pub fn regex(dimension: Dimension, pattern: &str) -> Dimension {
        Dimension::RegexFiltered {
            pattern: pattern.into(),
            delegate: Box::new(dimension),
        }
    }
    pub fn prefix(dimension: Dimension, prefix: &str) -> Dimension {
        Dimension::PrefixFiltered {
            prefix: prefix.into(),
            delegate: Box::new(dimension),
        }
    }
}
