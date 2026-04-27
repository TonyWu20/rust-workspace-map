/// A stage within the pipeline.
#[derive(Debug, Clone)]
pub struct PipelineStage {
    pub name: String,
    pub order: u32,
}

impl PipelineStage {
    pub fn new(name: String, order: u32) -> Self {
        Self { name, order }
    }
}
