#[derive(Default)]
pub struct Spec{
    pub url: String,
    pub method: String,
    pub data: String,
    pub spec_type: SpecType,
}

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

