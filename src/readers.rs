#[cfg(test)]
use mockall::{predicate::*};
use super::models;

#[automock]
pub trait SpecFinder{
    fn find(&self) -> Vec<models::Spec>;
}

pub struct FileSpecFinder{

}
