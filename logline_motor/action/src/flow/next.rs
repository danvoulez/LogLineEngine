use serde::{Deserialize, Serialize};
use crate::action::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextAction;

impl Action for NextAction {
    fn execute(&self) {}
}
