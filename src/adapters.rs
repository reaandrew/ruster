#[cfg(test)]
use super::utils;
use super::core::{Result};
use super::models;

use std::fs::{read_to_string};
use yaml_rust::{YamlLoader};

pub struct SpecFileAdapter{

}

impl SpecFileAdapter{
    pub fn adapt(&self, path:String) -> Result<models::Spec>{
        println!("adapting file");
        let contents = read_to_string(path).expect("Unable to read file");
        let docs = YamlLoader::load_from_str(&contents).expect("Could not load file");
        //TODO: Test for no docs
        let doc = &docs[0];
        println!("returning spec");
        return Ok(models::Spec{
            url: String::from(doc["url"].as_str().unwrap()),
            method: String::from(match doc["method"].as_str(){
                Some(method) => String::from(method),
                None       => String::from("GET")
            }),
            data: String::from(match doc["data"].as_str(){
                Some(data) => String::from(data),
                None       => String::from("")
            }),
            spec_type: models::SpecType::HTTP,
        });
    }
}

#[test]
fn test_creating_a_spec_from_file()-> Result<()>{
    utils::create_spec_file(10, |mut result| -> Result<()> {
        let adapter = SpecFileAdapter{};
        let spec = adapter.adapt(result.file_path.remove(1))?;
        assert_eq!(spec.url,"http://localhost/path");
        Ok(())
    })?;
    Ok(())
}

