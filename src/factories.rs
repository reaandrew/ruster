use super::models;
use super::executors;
use super::decorators::{ScriptExecutor};
use super::core::{Result};
use super::app;
use super::finders;

#[cfg_attr(test, mockall::automock)]
pub trait ExecutorFactory{
    fn create(&self, spec: &models::Spec) -> Result<Box<dyn executors::SpecExecutor>>;
}

pub struct DefaultExecutorFactory{

}

impl ExecutorFactory for DefaultExecutorFactory {
    fn create(&self, _: &models::Spec) -> Result<Box<dyn executors::SpecExecutor>>{

        Ok(Box::new(ScriptExecutor{
            executor: &executors::HttpSpecExecutor{}
        }))
    }
}

pub struct DefaultAppFactory<'a, 
    TFinder:finders::SpecFinder,
    TExecutorFactory:ExecutorFactory>
{
    
    pub spec_finder: &'a TFinder,
    pub executor_factory: &'a TExecutorFactory
}

impl <'a, 
    TFinder:finders::SpecFinder,
    TExecutorFactory:ExecutorFactory> DefaultAppFactory<'a, TFinder,TExecutorFactory>{
        
    pub fn create(&self) -> Result<impl app::App + 'a>{
        let app = app::DefaultApp{
            spec_finder: self.spec_finder,
            executor_factory: self.executor_factory
        };
        return Ok(app);
    }

} 

mod tests{

    #[cfg(test)]
    use super::*;

    #[test]
    fn test_executor_factory_return_http_executor() -> Result<()>{
        let mut spec: models::Spec = Default::default();
        spec.url = String::from("http://localhost:8000");
        spec.data = String::from("bla");
        spec.method = String::from("GET");

        let factory = DefaultExecutorFactory{};
        let executor = factory.create(&spec)?;

        let supported_types = executor.supported_spec_types();
        assert_eq!(supported_types.len(), 1);
        assert_eq!(supported_types[0], models::SpecType::HTTP);
        Ok(())
    }
}
