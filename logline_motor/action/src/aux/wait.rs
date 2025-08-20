use serde::{Deserialize, Serialize};
use crate::action::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitAction;

impl Action for WaitAction {
    fn execute(&self) {}
}
