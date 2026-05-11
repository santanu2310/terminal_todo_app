use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoItem {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub priority: Priority,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}
