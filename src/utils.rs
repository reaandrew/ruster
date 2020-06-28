#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::Write;
#[cfg(test)]
use tempfile::Builder;

#[cfg(test)]
use uuid::Uuid;

#[cfg(test)]
pub struct CreateSpecFileResult{
    pub directory: String,
    pub file_path: Vec<String>
}

#[cfg(test)]
pub fn create_spec_file<T>(count:i32, callback:T) 
    -> Result<(), std::io::Error> where 
    T: Fn(CreateSpecFileResult) -> Result<(),std::io::Error>{
    let dir = Builder::new().prefix("ruster").tempdir()?;
    let mut result = CreateSpecFileResult{
        directory: dir.path().display().to_string(),
        file_path: vec![],
    };
    for _ in 0..count {
        let my_uuid = Uuid::new_v4();
        let file_path = dir.path().join(format!("spec_{uuid}.yml",
                uuid=my_uuid));
        let mut tmp_file = File::create(&file_path)?;
        writeln!(tmp_file, "---
    url: \"http://localhost/path\"
")?;
        result.file_path.push(file_path.display().to_string())
    }
    callback(result)?;
    Ok(())
}
