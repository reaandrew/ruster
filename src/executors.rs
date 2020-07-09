use super::models;
use super::core::{Result};
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
                let resp = reqwest::blocking::get(&spec.url)?
                    .json::<HashMap<String, String>>()?;
            },
            "POST" => {
                let client = reqwest::blocking::Client::new();
                let resp = client.post(&spec.url)
                    .send();
            }
            _ => ()
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
