use serde::{Deserialize, Serialize};

pub trait Action {
    fn execute(&self);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionContext;
