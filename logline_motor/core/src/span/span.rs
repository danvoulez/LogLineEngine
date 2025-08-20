use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::span_id::SpanId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    pub id: SpanId,
    pub input: String,
    pub output: Option<String>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl Span {
    pub fn new(id: SpanId, input: String, output: String) -> Self {
        Span {
            id,
            input,
            output: Some(output),
            error: None,
            timestamp: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_span() {
        let id = SpanId::new();
        let span = Span::new(id.clone(), "input".into(), "output".into());
        assert_eq!(span.id, id);
    }
}
