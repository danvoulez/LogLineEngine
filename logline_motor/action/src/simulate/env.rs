use serde::{Deserialize, Serialize};
use crate::action::Action;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvAction;

impl Action for EnvAction {
    fn execute(&self) {}
}
