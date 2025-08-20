use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateDiff {
    pub changes: Vec<(String, String)>,
}
