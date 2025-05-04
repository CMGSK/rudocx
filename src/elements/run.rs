use crate::elements::run_properties::RunProperties;
#[derive(Debug, Clone, PartialEq)]
pub struct Run {
    pub properties: RunProperties,
    pub text: String,
}
