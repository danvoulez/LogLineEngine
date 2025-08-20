use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parallel;

impl Parallel {
    pub fn new() -> Self {
        Self
    }
}
