use super::models;
use super::executors;
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
        println!("creating an executor");
        Ok(Box::new(executors::HttpSpecExecutor{}))
    }
}

pub trait AppFactory<'a, 
    TFinder:finders::SpecFinder,
    TExecutorFactory:ExecutorFactory>{
    fn create(&self) -> 
        Result<app::App<'a, TFinder,TExecutorFactory>>;
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
    TExecutorFactory:ExecutorFactory> AppFactory<'a, TFinder,TExecutorFactory> for 
    DefaultAppFactory<'a, TFinder,TExecutorFactory>{
        
    fn create(&self) -> Result<app::App<'a, TFinder,TExecutorFactory>>{
        let app = app::App{
            spec_finder: self.spec_finder,
            executor_factory: self.executor_factory
        };
        return Ok(app);
    }

} 

#[test]
fn test_executor_factory_return_http_executor() -> Result<()>{
    let spec = &models::Spec{
        url: String::from("http://localhost:8000"),
        spec_type: models::SpecType::HTTP,
    };
    let factory = DefaultExecutorFactory{};
    let executor = factory.create(spec)?;

    let supported_types = executor.supported_spec_types();
    assert_eq!(supported_types.len(), 1);
    assert_eq!(supported_types[0], models::SpecType::HTTP);
    Ok(())
}

