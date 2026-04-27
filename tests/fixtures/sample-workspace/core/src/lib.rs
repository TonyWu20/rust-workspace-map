use serde::{Deserialize, Serialize};

/// A task that can be executed by a runner.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub name: String,
    pub payload: Vec<u8>,
}

impl Task {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            payload: Vec::new(),
        }
    }

    pub fn with_payload(mut self, payload: Vec<u8>) -> Self {
        self.payload = payload;
        self
    }
}

/// A trait for types that can execute tasks.
pub trait Runner {
    fn run(&self, task: &Task) -> bool;
}

/// A basic no-op runner.
pub struct NoopRunner;

impl Runner for NoopRunner {
    fn run(&self, _task: &Task) -> bool {
        true
    }
}

/// Re-export Task for convenience.
pub use Task as CoreTask;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_new() {
        let task = Task::new(1, "test".into());
        assert_eq!(task.id, 1);
        assert_eq!(task.name, "test");
    }
}
