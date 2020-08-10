use super::core::{Result};
use super::models;
use super::executors::{SpecExecutor};

pub struct ScriptExecutor<'a, TExecutor:SpecExecutor>{
    pub executor: &'a TExecutor,
}

impl <'a, TExecutor:SpecExecutor> ScriptExecutor<'a, TExecutor>{

    fn evalulate_before(&self, value: String) {

    }

}

impl <'a, TExecutor:SpecExecutor> SpecExecutor for ScriptExecutor<'a, TExecutor>{

    fn execute(&self, spec: &models::Spec) -> Result<models::SpecResult>{
        self.executor.execute(spec)
    }
    fn supported_spec_types(&self) -> Vec<models::SpecType>{
        return self.executor.supported_spec_types() 
    }
}


