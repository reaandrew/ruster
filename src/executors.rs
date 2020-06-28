use super::models;

pub trait SpecExecutor{
    fn execute(&self, spec: &models::Spec) -> Result<models::SpecResult, std::io::Error>;
    fn supported_spec_types(&self) -> Vec<models::SpecType>;
}

pub struct HttpSpecExecutor{
    
}

impl SpecExecutor for HttpSpecExecutor{
    fn execute(&self, _: &models::Spec) -> Result<models::SpecResult, std::io::Error>{
        Ok(models::SpecResult{
            success: true,
        })
    }
    fn supported_spec_types(&self) -> Vec<models::SpecType>{
        return vec![
            models::SpecType::HTTP,
        ] 
    }
}
