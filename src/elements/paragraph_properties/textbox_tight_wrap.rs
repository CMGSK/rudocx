use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphTBoxTightWrap {
    pub val: ParagraphTBoxTightWrapValues,
}

impl ParagraphTBoxTightWrap {
    pub fn new(val: ParagraphTBoxTightWrapValues) -> Self {
        Self { val }
    }

    pub fn change_value(&mut self, val: ParagraphTBoxTightWrapValues) {
        self.val = val;
    }

    pub fn value(&self) -> String {
        self.val.to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphTBoxTightWrapValues {
    AllLines,
    FirstAndLastLine,
    FirstLineOnly,
    LastLineOnly,
    None,
}

impl fmt::Display for ParagraphTBoxTightWrapValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParagraphTBoxTightWrapValues::AllLines => write!(f, "allLines"),
            ParagraphTBoxTightWrapValues::FirstAndLastLine => write!(f, "firstAndLastLine"),
            ParagraphTBoxTightWrapValues::FirstLineOnly => write!(f, "firstLineOnly"),
            ParagraphTBoxTightWrapValues::LastLineOnly => write!(f, "lastLineOnly"),
            ParagraphTBoxTightWrapValues::None => write!(f, "none"),
        }
    }
}
