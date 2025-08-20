use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dispatcher;

impl Dispatcher {
    pub fn new() -> Self {
        Self
    }
}
