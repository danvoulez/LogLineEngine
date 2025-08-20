use serde::{Deserialize, Serialize};
use crate::action::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AskAction;

impl Action for AskAction {
    fn execute(&self) {}
}
