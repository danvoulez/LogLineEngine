use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockType {
    Text,
    Table,
    Timeline,
    Map,
    Choice,
}
