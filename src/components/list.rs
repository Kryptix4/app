use super::task::Task;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List(pub Vec<Task>);

impl List {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, text: impl Into<String> + AsRef<str>) {
        if self.0.iter().any(|task| task.text == text.as_ref()) {
            return;
        }

        self.0.push(Task::new(text.into()))
    }

    pub fn remove(&mut self, text: impl AsRef<str>) {
        if let Some(index) = self.0.iter().position(|task| task.text == text.as_ref()) {
            self.0.remove(index);
        };
    }

    pub fn clear_completed(&mut self) {
        self.0 = self.0.clone().into_iter().filter(|task| !(task.done)()).collect();
    }
}
