use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub struct VerticalAlign {
    pub value: AlignValues,
}

impl Default for VerticalAlign {
    fn default() -> Self {
        Self {
            value: AlignValues::Baseline,
        }
    }
}

impl VerticalAlign {
    pub fn new(value: AlignValues) -> Self {
        Self { value }
    }

    pub fn value(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlignValues {
    Baseline,
    Superscript,
    Subscript,
}
impl fmt::Display for AlignValues {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AlignValues::Baseline => "baseline",
                AlignValues::Subscript => "subscript",
                AlignValues::Superscript => "superscript",
            }
        )
    }
}
