pub struct Spec{
   pub url: String,
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

