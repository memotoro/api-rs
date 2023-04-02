use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SimpleData {
    pub name: String,
    pub timestamp: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct ComplexData {
    pub name: String,
    pub data: Vec<SimpleData>,
}
