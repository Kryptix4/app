use super::task::Task;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List(pub Vec<Task>);

impl List {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, item: Task) {
        self.0.push(item)
    }

    pub fn clear_done(&mut self) {
        self.0 = self.0.clone().into_iter().filter(|task| !(task.done)()).collect();
    }
}
