use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BacklogData {
    pub entries: Vec<Entry>,
    pub history: Vec<HistoryEntry>,
    pub raw_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub id: String,
    pub title: String,
    pub status: String,
    pub priority: String,
    pub due: Option<String>,
    pub progress: Option<u8>,
    pub body: Option<String>,
    pub children: Vec<Entry>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub timestamp: String,
    pub item_id: String,
    pub action: String,
    pub details: String,
}
