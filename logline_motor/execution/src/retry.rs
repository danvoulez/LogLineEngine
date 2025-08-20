use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Retry;

impl Retry {
    pub fn new() -> Self {
        Self
    }
}
