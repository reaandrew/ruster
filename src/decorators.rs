use super::core::{Result};
use super::models;
use super::executors::{SpecExecutor};

pub struct ScriptExecutor{
    pub executor: Box<dyn SpecExecutor>
}

impl SpecExecutor for ScriptExecutor{
    fn execute(&self, spec: &models::Spec) -> Result<models::SpecResult>{
        self.executor.execute(spec)
    }
    fn supported_spec_types(&self) -> Vec<models::SpecType>{
        return self.executor.supported_spec_types() 
    }
}


