use super::models;

use mockall::*;
use mockall::predicate::*;


#[automock()]
pub trait SpecFinder{
    fn find(&self) -> Vec<models::Spec>;
}

pub struct FileSpecFinder{

}
