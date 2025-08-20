use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SpanId {
    pub uuid: Uuid,
    pub checksum: String,
}

impl SpanId {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();
        let mut hasher = Sha256::new();
        hasher.update(uuid.as_bytes());
        let checksum = format!("{:x}", hasher.finalize());
        Self { uuid, checksum }
    }
}
