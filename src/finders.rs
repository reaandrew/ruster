use std::fs;

use super::models;
use super::adapters;
use super::core;

#[cfg(test)]
use super::utils;

#[cfg_attr(test, mockall::automock)]
pub trait SpecFinder{
    fn find(&self) -> core::Result<Vec<models::Spec>>;
}

#[test]
fn test_file_spec_finder_finds_spec_files()-> core::Result<()>{
    utils::create_spec_file(10, |result| -> core::Result<()> {
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
    fn find(&self) -> core::Result<Vec<models::Spec>>{
        println!("Finding specs");
        let adapter = adapters::SpecFileAdapter{};
        let mut result : Vec<models::Spec> = vec![];
        println!("Self Path is {:?}", &self.path);

        fs::read_dir(&self.path)?
            .filter(|f| match f{
                Ok(entry) => match entry.path().extension(){
                    Some(extension) => extension == "yml",
                    None            => false
                }
                Err(_) => false,
            })
        .map(|res| res.map(|e| e.path()))
            .filter_map(Result::ok)
            .for_each(|path| {
                match adapter.adapt(match path.to_str(){
                    Some(path) => String::from(path),
                    _=>String::from("")
                }){
                    Ok(spec) => result.push(spec),
                    _   => ()
                }
            });

        return Ok(result);
    }
}
