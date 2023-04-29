use serde::Serialize;

use crate::models::data::Summary;

#[derive(Debug, Serialize)]
pub struct ReplayHeader {
    pub summary: Summary,
    pub author_nickname: String,
    pub arena_unique_id: u64,
    pub arena_type_id: u32,
}
