use serde::{Deserialize, Serialize};
use crate::action::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAction;

impl Action for LogAction {
    fn execute(&self) {}
}
