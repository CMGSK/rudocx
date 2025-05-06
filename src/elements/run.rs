use crate::elements::RunProperties;
#[derive(Debug, Clone, PartialEq)]
pub struct Run {
    pub properties: RunProperties,
    pub text: String,
}
