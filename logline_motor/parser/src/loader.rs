use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loader;

impl Loader {
    pub fn load(_input: &str) -> Self {
        Loader
    }
}
