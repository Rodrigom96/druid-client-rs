use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum HllType {
    #[allow(non_camel_case_types)]
    HLL_4,
    #[allow(non_camel_case_types)]
    HLL_6,
    #[allow(non_camel_case_types)]
    HLL_8,
}
