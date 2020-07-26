#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::Write;
#[cfg(test)]
use tempfile::Builder;

#[cfg(test)]
use uuid::Uuid;

#[cfg(test)]
use super::core;

#[cfg(test)]
pub struct CreateSpecFileResult{
    pub directory: String,
    pub file_path: Vec<String>
}

#[cfg(test)]
pub struct CreateSpecFileSpec{
    pub url: String,
    pub method: String,
    pub data: String
}

#[cfg(test)]
impl Default for CreateSpecFileSpec {
    fn default() -> Self { 
        return CreateSpecFileSpec{
            url: "".into(),
            method: "GET".into(),
            data: "".into(),
        }
    }
}

#[cfg(test)]
pub fn create_spec_file<T>(count:i32, spec:&CreateSpecFileSpec, callback:T)
    where T: Fn(CreateSpecFileResult) -> core::Result<()>{
    let dir = Builder::new().prefix("ruster").tempdir().unwrap();
    let mut something = CreateSpecFileResult{
        directory: dir.path().display().to_string(),
        file_path: vec![],
    };
    for _ in 0..count {
        let my_uuid = Uuid::new_v4();
        let file_path = dir.path().join(format!("spec_{uuid}.yml",
                uuid=my_uuid));
        let mut tmp_file = File::create(&file_path).unwrap();
        writeln!(tmp_file, "---
    url: {}
    method: {}
    data: |
        {}
", spec.url, spec.method, spec.data).unwrap();
        something.file_path.push(file_path.display().to_string())
    }
    callback(something).expect("Something went wrong invoking callback");
}
