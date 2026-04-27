mod pipeline;

use core::Task;
use serde::Serialize;

/// The main pipeline struct that orchestrates task execution.
#[derive(Debug, Clone, Serialize)]
pub struct Pipeline {
    pub name: String,
    pub tasks: Vec<Task>,
}

impl Pipeline {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
}

pub use pipeline::PipelineStage;

/// Re-export Pipeline for convenience.
pub use Pipeline as EnginePipeline;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_new() {
        let p = Pipeline::new("test".into());
        assert_eq!(p.name, "test");
        assert!(p.tasks.is_empty());
    }
}
