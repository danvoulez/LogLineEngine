use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator;

impl Validator {
    pub fn validate(&self) -> bool {
        true
    }
}
