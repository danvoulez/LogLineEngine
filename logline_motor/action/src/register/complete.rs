use serde::{Deserialize, Serialize};
use crate::action::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteAction;

impl Action for CompleteAction {
    fn execute(&self) {}
}
