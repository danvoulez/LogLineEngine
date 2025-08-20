use serde::{Deserialize, Serialize};
use crate::action::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifyAction;

impl Action for ClassifyAction {
    fn execute(&self) {}
}
