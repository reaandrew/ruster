use super::models;
use super::executors;
use super::factories;
use super::finders;
use super::errors::{RusterError};
use super::core::{Result};

pub trait App{
    fn execute(&self) -> Result<i32>;
}

pub struct DefaultApp<'a, 
    TFinder:finders::SpecFinder,
    TExecutorFactory:factories::ExecutorFactory>{
    pub spec_finder: &'a TFinder,
    pub executor_factory: &'a TExecutorFactory
}

impl<'a,
    TFinder:finders::SpecFinder,
    TExecutorFactory:factories::ExecutorFactory> App for DefaultApp<'a,TFinder, TExecutorFactory>{
    fn execute(&self) -> Result<i32>{
        println!("app finding specs");

        let specs = self.spec_finder.find()?;
        let mut error: Option<RusterError> = None;
        specs.iter().for_each(|spec| {
            if error.is_none(){
                let executor = self.executor_factory.create(spec);
                match executor{
                    Ok(executor) => {
                        match executor.execute(spec){
                            Ok(_) => (),
                            Err(e) => {
                                error = Some(e)
                            }
                        }
                    },
                    _ => ()
                }
            }
        });
        match error{
            None => Ok(1),
            Some(e) => Err(e)
        }
    }
}

struct FakeExecutor{

}

impl executors::SpecExecutor for FakeExecutor{
    fn execute(&self, _: &models::Spec) -> Result<models::SpecResult>{
        Ok(models::SpecResult{
            success: true,
            data: String::from(""),
        })
    }
    fn supported_spec_types(&self) -> Vec<models::SpecType>{
        return vec![
            models::SpecType::HTTP,
        ] 
    }
}


mod tests{

    #[cfg(test)]
    use super::*;

    #[test]
    fn test_app_returns_number_of_specs_found() -> Result<()> {
        let mut mock_spec_finder = finders::MockSpecFinder::new();
        &mock_spec_finder.expect_find()
            .times(1)
            .returning(| | Ok(vec![models::Spec{
                url: String::from(""),
                data: String::from(""),
                method: String::from(""),
                spec_type: models::SpecType::HTTP,
            }]));

        let mut mock_spec_executor = executors::MockSpecExecutor::new();
        &mock_spec_executor.expect_execute()
            .returning(|_| Ok(models::SpecResult{
                success: true,
                data: String::from(""),
            }));

        //TODO: Figure out how to create and pass a MockSpecExecutor instead of a FakeExecutor
        let mut mock_executor_factory = factories::MockExecutorFactory::new();
        &mock_executor_factory.expect_create()
            .returning(|_| Ok(Box::new(FakeExecutor{})));
        let app = DefaultApp{
            spec_finder: &mock_spec_finder,
            executor_factory: &mock_executor_factory,
        };

        assert_eq!(1, app.execute()?);
        Ok(())
    }

}
