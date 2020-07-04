#[cfg(test)]
use super::utils;
use std::fs::{read_to_string};
use super::models;
use yaml_rust::{YamlLoader};

pub struct SpecFileAdapter{

}

impl SpecFileAdapter{
    pub fn adapt(&self, path:String) -> Result<models::Spec, std::io::Error>{
        println!("adapting file");
        let contents = read_to_string(path).expect("Unable to read file");
        let docs = YamlLoader::load_from_str(&contents).expect("Could not load file");
        //TODO: Test for no docs
        let doc = &docs[0];
        println!("returning spec");
        return Ok(models::Spec{
            url: String::from(doc["url"].as_str().unwrap()),
            spec_type: models::SpecType::HTTP,
        });
    }
}

#[test]
fn test_creating_a_spec_from_file()-> Result<(), std::io::Error>{
    utils::create_spec_file(10, |mut result| -> Result<(),std::io::Error> {
        let adapter = SpecFileAdapter{};
        let spec = adapter.adapt(result.file_path.remove(1))?;
        assert_eq!(spec.url,"http://localhost/path");
        Ok(())
    })?;
    Ok(())
}

