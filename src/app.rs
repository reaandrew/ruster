use super::models;
use super::executors;
use super::factories;
use super::finders;
use super::core::{Result};

pub struct App<'a, 
    TFinder:finders::SpecFinder,
    TExecutorFactory:factories::ExecutorFactory>{
    pub spec_finder: &'a TFinder,
    pub executor_factory: &'a TExecutorFactory
}

impl<'a,
    TFinder:finders::SpecFinder,
    TExecutorFactory:factories::ExecutorFactory> App<'a,TFinder, TExecutorFactory>{
    pub fn execute(&self) -> Result<i32>{
        println!("app finding specs");

        let specs = self.spec_finder.find()?;
        specs.iter().for_each(|spec| {
            let executor = self.executor_factory.create(spec);
            match executor{
                Ok(executor) => {
                    executor.execute(spec);
                },
                _ => ()
            }
        });
        return Ok(specs.len() as i32);
    }
}

struct FakeExecutor{

}

impl executors::SpecExecutor for FakeExecutor{
    fn execute(&self, _: &models::Spec) -> Result<models::SpecResult>{
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



#[test]
fn test_app_returns_number_of_specs_found() -> Result<()> {
    let mut mock_spec_finder = finders::MockSpecFinder::new();
    &mock_spec_finder.expect_find()
        .times(1)
        .returning(| | Ok(vec![models::Spec{
            url: String::from(""),
            spec_type: models::SpecType::HTTP,
        }]));

    let mut mock_spec_executor = executors::MockSpecExecutor::new();
    &mock_spec_executor.expect_execute()
        .returning(|_| Ok(models::SpecResult{
            success: true,
        }));

    //TODO: Figure out how to create and pass a MockSpecExecutor instead of a FakeExecutor
    let mut mock_executor_factory = factories::MockExecutorFactory::new();
    &mock_executor_factory.expect_create()
        .returning(|_| Ok(Box::new(FakeExecutor{})));
    let app = App{
        spec_finder: &mock_spec_finder,
        executor_factory: &mock_executor_factory,
    };

    assert_eq!(1, app.execute()?);
    Ok(())
}

