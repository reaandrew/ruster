#[cfg(test)]
use super::models;

use super::finders;

pub struct App<'a, 
    FINDER:finders::SpecFinder>{
    pub spec_finder: &'a FINDER
}

impl<'a,T:finders::SpecFinder> App<'a,T>{
    pub fn execute(&self) -> Result<i32,std::io::Error>{
        let specs = self.spec_finder.find()?;
        return Ok(specs.len() as i32);
    }
}

#[test]
fn test_app_returns_number_of_specs_found() -> Result<(), std::io::Error> {
    let mut mock_spec_finder = finders::MockSpecFinder::new();
    &mock_spec_finder.expect_find()
        .times(1)
        .returning(| | Ok(vec![models::Spec{
            url: String::from(""),
            spec_type: models::SpecType::HTTP,
        }]));

    let app = App{
        spec_finder: &mock_spec_finder,
    };

    assert_eq!(1, app.execute()?);
    Ok(())
}

