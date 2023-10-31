use serde::{Deserialize, Serialize};
use uuid::Uuid;
use leptos::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub text: String,
    pub done: RwSignal<bool>,
}

impl Task {
    pub fn new(text: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            text,
            done: create_rw_signal(false),
        }
    }
}

impl Default for Task {
    fn default() -> Self {
        Self::new(String::new())
    }
}
