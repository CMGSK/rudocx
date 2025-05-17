use crate::elements::RunProperties;
#[derive(Debug, Clone, PartialEq)]
pub struct Run {
    pub properties: RunProperties,
    pub text: String,
    pub space_preserve: bool,
}

impl Default for Run {
    fn default() -> Self {
        Self {
            properties: RunProperties::default(),
            text: String::new(),
            space_preserve: false,
        }
    }
}

impl From<String> for Run {
    fn from(s: String) -> Self {
        Self {
            text: s,
            ..Default::default()
        }
    }
}

impl From<RunProperties> for Run {
    fn from(rp: RunProperties) -> Self {
        Self {
            properties: rp,
            ..Self::default()
        }
    }
}

impl Run {
    pub fn new(properties: RunProperties, text: String, space_preserve: bool) -> Self {
        Self {
            properties,
            text,
            space_preserve,
        }
    }
}
