use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Runner;

impl Runner {
    pub fn new() -> Self {
        Self
    }
}
