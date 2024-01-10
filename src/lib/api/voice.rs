use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Voice {
    pub duration: u32,
    pub mime_type: String,
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: u32,
}
