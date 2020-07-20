use std::{fmt};

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
    pub data: String,
}
// Implement std::fmt::Display for AppError
impl fmt::Display for SpecResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Success: {}
Data: {}",self.success, self.data)
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum SpecType{
    HTTP,
}

impl Default for SpecType {
    fn default() -> Self { SpecType::HTTP }
}

