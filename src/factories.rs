use super::models;
use super::executors;

#[cfg_attr(test, mockall::automock)]
pub trait ExecutorFactory{
    fn create(&self, spec: &models::Spec) -> Result<Box<dyn executors::SpecExecutor>, std::io::Error>;
}

pub struct DefaultExecutorFactory{

}

impl ExecutorFactory for DefaultExecutorFactory {
    fn create(&self, _: &models::Spec) -> Result<Box<dyn executors::SpecExecutor>, std::io::Error>{
        Ok(Box::new(executors::HttpSpecExecutor{}))
    }
}

#[test]
fn test_executor_factory_return_http_executor() -> Result<(), std::io::Error>{
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
