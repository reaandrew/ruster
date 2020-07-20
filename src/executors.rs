use super::models;
use super::core::{Result};
use super::errors::{RusterError, ErrorType};

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
        let mut result = models::SpecResult{
            success: true,
            data: String::from(""),
        };
        match spec.method.to_uppercase().as_ref(){
            "GET" => {
                let response = reqwest::blocking::get(&spec.url)?;
                result.data = response.text()?;
            },
            "POST" => {
                let client = reqwest::blocking::Client::new();
                let response = client.post(&spec.url)
                    .send()?;
                result.data = response.text()?;
            }
            _ => {
                println!("Returning the expected error");
                return Err(RusterError::Of(ErrorType::MethodNotSupported))
            } 
        }
        Ok(result)
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
    fn test_spec_executor_execute_returns_spec_result_body_for_get(){
        let expected_body = "boo";
        let mock = mockito::mock("GET", "/hello")
            .with_body(expected_body)
            .create();
        let executor = HttpSpecExecutor{};
        let mut spec: models::Spec = Default::default();
        spec.method = "GET".into();
        spec.url = [mockito::server_url(), "/hello".to_string()].join("");
        let spec_result = executor.execute(&spec).unwrap();
        mock.assert();
        assert_eq!(spec_result.data, expected_body);
    }

    #[test]
    fn test_spec_executor_execute_returns_spec_result_body_for_post(){
        let expected_body = "boo";
        let mock = mockito::mock("POST", "/hello")
            .with_body(expected_body)
            .create();
        let executor = HttpSpecExecutor{};
        let mut spec: models::Spec = Default::default();
        spec.method = "POST".into();
        spec.url = [mockito::server_url(), "/hello".to_string()].join("");
        let spec_result = executor.execute(&spec).unwrap();
        mock.assert();
        assert_eq!(spec_result.data, expected_body);
    }

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
                data: String::from(""),
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
                data: String::from(""),
            }));

        let mut spec: models::Spec = Default::default();
        spec.url = String::from("http://somewhere");

        do_something(Box::new(mock_spec_executor), spec)?;
        Ok(())
    }
}
