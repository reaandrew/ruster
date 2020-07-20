use super::models;
use super::core::{Result};
use super::errors::{RusterError, ErrorType};
use std::collections::HashMap;

#[cfg_attr(test, mockall::automock)]
pub trait SpecExecutor{
    fn execute(&self, spec: &models::Spec) -> 
        Result<models::SpecResult>;
    fn supported_spec_types(&self) -> Vec<models::SpecType>;
}

pub struct HttpSpecExecutor{

}

impl SpecExecutor for HttpSpecExecutor{
    fn execute(&self, spec: &models::Spec) -> Result<models::SpecResult>{
        match spec.method.to_uppercase().as_ref(){
            "GET" => {
                let _ = reqwest::blocking::get(&spec.url)?
                    .json::<HashMap<String, String>>()?;
            },
            "POST" => {
                let client = reqwest::blocking::Client::new();
                let _ = client.post(&spec.url)
                    .send();
            }
            _ => {
                println!("Returning the expected error");
                return Err(RusterError::Of(ErrorType::MethodNotSupported))
            } 
        }
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

mod tests{

    #[cfg(test)]
    use mockito;

    #[cfg(test)]
    use super::*;


    #[test]
    fn test_spec_executor_execute_returns_method_not_supported(){
        let executor = HttpSpecExecutor{};
        let mut spec: models::Spec = Default::default();
        spec.method = "SOMETHING".into();

        let expected = Err(RusterError::Of(ErrorType::MethodNotSupported));
        let actual = executor.execute(&spec).map_err(|e| e);
        assert_eq!(expected,actual);
    }

    #[test]
    fn test_spec_maps_url(){
        let mock = mockito::mock("GET", "/hello")
            .with_body(r#"{"hello": "world"}"#)
            .create();
        let executor = HttpSpecExecutor{};
        let mut spec: models::Spec = Default::default();
        spec.url = [mockito::server_url(), "/hello".to_string()].join("");
        let _ = executor.execute(&spec);
        mock.assert();
    }

    #[test]
    fn test_spec_maps_method(){
        let mock = mockito::mock("POST", "/hello")
            .with_body(r#"{"hello": "world"}"#)
            .create();
        let executor = HttpSpecExecutor{};
        let mut spec: models::Spec = Default::default();
        spec.url = [mockito::server_url(), "/hello".to_string()].join("");
        spec.method = "POST".into();
        let _ = executor.execute(&spec);
        mock.assert();
    }

    #[test]
    fn test_my_knowledge_of_mocking_traits() -> Result<()>{
        let mut mock_spec_executor = MockSpecExecutor::new();
        &mock_spec_executor.expect_execute()
            .returning(|_| Ok(models::SpecResult{
                success: true,
            }));

        let mut spec: models::Spec = Default::default();
        spec.url = String::from("http://somewhere");

        assert_eq!(mock_spec_executor.execute(&spec)?.success, true);
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

        let mut spec: models::Spec = Default::default();
        spec.url = String::from("http://somewhere");

        do_something(Box::new(mock_spec_executor), spec)?;
        Ok(())
    }
}
