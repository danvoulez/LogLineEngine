use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Token {
    Ident(String),
    Symbol(char),
    String(String),
    Number(f64),
}
