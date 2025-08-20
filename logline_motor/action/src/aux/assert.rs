use serde::{Deserialize, Serialize};
use crate::action::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssertAction;

impl Action for AssertAction {
    fn execute(&self) {}
}
