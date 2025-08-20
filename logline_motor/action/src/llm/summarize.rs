use serde::{Deserialize, Serialize};
use crate::action::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummarizeAction;

impl Action for SummarizeAction {
    fn execute(&self) {}
}
