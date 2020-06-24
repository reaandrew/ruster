#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::Write;
#[cfg(test)]
use tempfile::Builder;

use std::fs::{read_dir, read_to_string};
use yaml_rust::{YamlLoader};


use super::models;

#[cfg_attr(test, mockall::automock)]
pub trait SpecFinder{
    fn find(&self) -> Result<Vec<models::Spec>, std::io::Error>;
}

#[test]
fn test_file_spec_finder_finds_spec_files()-> Result<(), std::io::Error>{
    let dir = Builder::new().prefix("example").tempdir()?;
    for x in 0..10 {
        let file_path = dir.path().join(format!("spec_{number}.yml",number=x));
        let mut tmp_file = File::create(file_path)?;
        writeln!(tmp_file, "
---
url: http://localhost/{number}
        ",number=x)?;
    }
    let finder = FileSpecFinder{path:dir.path().display().to_string()};
    assert_eq!(10, finder.find()?.len());
    Ok(())
}

pub struct FileSpecFinder{
    path: String,
}

impl SpecFinder for FileSpecFinder{
    fn find(&self) -> Result<Vec<models::Spec>, std::io::Error>{
        let mut result : Vec<models::Spec> = vec![];
        for path in read_dir(&self.path)?{
            let dir = path?;
            let contents = read_to_string(dir.path()).expect("Unable to read file");
            let docs = YamlLoader::load_from_str(&contents).expect("Could not load file");
            let doc = &docs[0];
            result.push(models::Spec{
                url: String::from(doc["url"].as_str().unwrap()),
            })
        }
        return Ok(result);
    }
}
