use super::models;
use super::executors;

#[cfg_attr(test, mockall::automock)]
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

#[test]
fn test_my_knowledge_of_mocking_traits() -> Result<(), std::io::Error>{
    let mut mock_spec_executor = MockSpecExecutor::new();
    &mock_spec_executor.expect_execute()
        .returning(|_| Ok(models::SpecResult{
            success: true,
        }));

    assert_eq!(mock_spec_executor.execute(&models::Spec{
        spec_type:models::SpecType::HTTP,
        url: String::from("http::somewhere"),
    })?.success, true);
    Ok(())
}

#[test]
fn test_my_knowledge_of_mocking_traits_part_2() -> Result<(), std::io::Error>{
    fn do_something(executor: Box<dyn executors::SpecExecutor>,
        spec: models::Spec) -> Result<(), std::io::Error>{
        executor.execute(&spec)?;
        Ok(())
    }
    let mut mock_spec_executor = MockSpecExecutor::new();
    &mock_spec_executor.expect_execute()
        .returning(|_| Ok(models::SpecResult{
            success: true,
        }));

    do_something(Box::new(mock_spec_executor), models::Spec{
        url: String::from("http://localhost"),
        spec_type: models::SpecType::HTTP,
    })?;
    Ok(())
}
