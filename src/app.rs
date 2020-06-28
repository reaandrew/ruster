#[cfg(test)]
use super::models;

use super::finders;

pub struct App<'a, T:finders::SpecFinder>{
    pub spec_finder: &'a T
}

impl<'a,T:finders::SpecFinder> App<'a,T>{
    pub fn execute(&self) -> i32{
        match self.spec_finder.find(){
            Ok(specs) => {
                return specs.len() as i32;
            },
            Err(_) => {
                1
            }
        }
    }
}

#[test]
fn test_app_returns_number_of_specs_found() {
    let mut mock_spec_finder = finders::MockSpecFinder::new();
    &mock_spec_finder.expect_find()
        .times(1)
        .returning(| | Ok(vec![models::Spec{
            url: String::from(""),
        }]));

    let app = App{
        spec_finder: &mock_spec_finder,
    };

    assert_eq!(1, app.execute())
}

