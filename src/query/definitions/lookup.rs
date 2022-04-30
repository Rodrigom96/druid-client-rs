use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", rename = "map")]
pub struct LookupMap {
    map: std::collections::HashMap<String, String>,
    is_one_to_one: bool,
}
