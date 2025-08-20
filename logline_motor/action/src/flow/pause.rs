use serde::{Deserialize, Serialize};
use crate::action::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PauseAction;

impl Action for PauseAction {
    fn execute(&self) {}
}
