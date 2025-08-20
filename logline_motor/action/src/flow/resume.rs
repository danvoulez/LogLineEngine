use serde::{Deserialize, Serialize};
use crate::action::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeAction;

impl Action for ResumeAction {
    fn execute(&self) {}
}
