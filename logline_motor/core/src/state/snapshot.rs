use serde::{Deserialize, Serialize};

use super::state::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub state: State,
}
