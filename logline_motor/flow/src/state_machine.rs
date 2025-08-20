use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum State {
    Start,
    Running,
    Finished,
}
