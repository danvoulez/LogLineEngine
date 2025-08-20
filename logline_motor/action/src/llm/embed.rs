use serde::{Deserialize, Serialize};
use crate::action::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedAction;

impl Action for EmbedAction {
    fn execute(&self) {}
}
