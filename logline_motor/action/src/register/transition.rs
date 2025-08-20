use serde::{Deserialize, Serialize};
use crate::action::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionAction;

impl Action for TransitionAction {
    fn execute(&self) {}
}
