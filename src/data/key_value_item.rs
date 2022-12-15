use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct KeyValueItem {
    pub key: String,
    pub value: String,
}
