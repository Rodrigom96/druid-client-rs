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

impl VirtualColumn {
    pub fn expression(name: &str, expression: &str, output_type: OutputType) -> VirtualColumn {
        VirtualColumn::Expression {
            name: name.to_string(),
            expression: expression.to_string(),
            output_type,
        }
    }
}
