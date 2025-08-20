use serde::{Deserialize, Serialize};
use crate::action::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunAction;

impl Action for RunAction {
    fn execute(&self) {}
}
