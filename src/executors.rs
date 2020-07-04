use super::models;

use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[cfg_attr(test, mockall::automock)]
pub trait SpecExecutor{
    fn execute(&self, spec: &models::Spec) -> Result<models::SpecResult>;
    fn supported_spec_types(&self) -> Vec<models::SpecType>;
}

pub struct HttpSpecExecutor{

}

impl SpecExecutor for HttpSpecExecutor{
    fn execute(&self, _: &models::Spec) -> Result<models::SpecResult>{
        println!("executing spec");
         let resp = reqwest::blocking::get("https://httpbin.org/ip")?
            .json::<HashMap<String, String>>()?;
        println!("Something {:#?}", resp);
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
fn test_my_knowledge_of_mocking_traits() -> Result<()>{
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
fn test_my_knowledge_of_mocking_traits_part_2() -> Result<()>{
    fn do_something(executor: Box<dyn SpecExecutor>,
        spec: models::Spec) -> Result<()>{
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
