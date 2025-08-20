use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpanStatus {
    Success,
    Error,
    Running,
}
