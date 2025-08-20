use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fallback;

impl Fallback {
    pub fn new() -> Self {
        Self
    }
}
