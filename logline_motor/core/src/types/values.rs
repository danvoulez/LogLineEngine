use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
}
