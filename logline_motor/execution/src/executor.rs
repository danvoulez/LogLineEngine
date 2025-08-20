use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Self
    }
}
