use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Uuid,
    pub text: String,
}

impl Task {
    pub fn new(text: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            text,
        }
    }
}

impl Default for Task {
    fn default() -> Self {
        Self::new(String::new())
    }
}
