use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpanMeta {
    pub name: Option<String>,
    pub description: Option<String>,
}
