use serde::{Deserialize, Serialize};
use crate::action::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedAction;

impl Action for SeedAction {
    fn execute(&self) {}
}
