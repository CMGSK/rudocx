use std::fmt;

/// Contains alignment value: left, center, right, both (justified), distribute, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphJustification {
    pub val: ParagraphJustificationValues,
}

impl ParagraphJustification {
    pub fn new(val: ParagraphJustificationValues) -> Self {
        Self { val }
    }

    pub fn change_value(&mut self, val: ParagraphJustificationValues) {
        self.val = val;
    }

    pub fn value(&self) -> String {
        self.val.to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphJustificationValues {
    Left,
    Center,
    Right,
    Both,
    MediumKashida,
    DistributedKashida,
    NumTab,
    HighKashida,
    LowKashida,
    ThaiDistributed,
}

impl fmt::Display for ParagraphJustificationValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParagraphJustificationValues::Left => write!(f, "left"),
            ParagraphJustificationValues::Center => write!(f, "center"),
            ParagraphJustificationValues::Right => write!(f, "right"),
            ParagraphJustificationValues::Both => write!(f, "both"),
            ParagraphJustificationValues::MediumKashida => write!(f, "mediumKashida"),
            ParagraphJustificationValues::DistributedKashida => write!(f, "distributedKashida"),
            ParagraphJustificationValues::NumTab => write!(f, "numTab"),
            ParagraphJustificationValues::HighKashida => write!(f, "highKashida"),
            ParagraphJustificationValues::LowKashida => write!(f, "lowKashida"),
            ParagraphJustificationValues::ThaiDistributed => write!(f, "thaiDistributed"),
        }
    }
}
