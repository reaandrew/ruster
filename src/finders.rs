use std::fs::{read_dir};

use super::models;
use super::adapters;

#[cfg(test)]
use super::utils;

#[cfg_attr(test, mockall::automock)]
pub trait SpecFinder{
    fn find(&self) -> Result<Vec<models::Spec>, std::io::Error>;
}

#[test]
fn test_file_spec_finder_finds_spec_files()-> Result<(), std::io::Error>{
    utils::create_spec_file(10, |result| -> Result<(),std::io::Error> {
        let finder = FileSpecFinder{path:result.directory};
        assert_eq!(10, finder.find()?.len());
        Ok(())
    })?;
    Ok(())
}

pub struct FileSpecFinder{
    pub path: String,
}

impl SpecFinder for FileSpecFinder{
    fn find(&self) -> Result<Vec<models::Spec>, std::io::Error>{
        let adapter = adapters::SpecFileAdapter{};
        let mut result : Vec<models::Spec> = vec![];
        for path in read_dir(&self.path)?{
            let dir = path?;
            result.push(adapter.adapt(dir.path().display().to_string())?)
        }
        return Ok(result);
    }
}
