use serde::{Deserialize, Serialize};

use super::output_types::OutputType;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum VirtualColumn {
    #[serde(rename_all = "camelCase")]
    Expression {
        name: String,
        expression: String,
        output_type: OutputType,
    },
}
