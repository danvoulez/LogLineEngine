use serde::{Deserialize, Serialize};
use crate::action::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAction;

impl Action for CreateAction {
    fn execute(&self) {}
}
