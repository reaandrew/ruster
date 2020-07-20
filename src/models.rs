#[derive(Debug)]
#[derive(PartialEq)]
pub struct Spec{
    pub url: String,
    pub method: String,
    pub data: String,
    pub spec_type: SpecType,
}

impl Default for Spec {
    fn default() -> Self { 
        return Spec{
            url: "".into(),
            method: "GET".into(),
            data: "".into(),
            spec_type: SpecType::HTTP,
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct SpecResult{
    pub success: bool,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum SpecType{
    HTTP,
}

impl Default for SpecType {
    fn default() -> Self { SpecType::HTTP }
}

