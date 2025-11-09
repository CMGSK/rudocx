use std::fmt;

/// Contains vertical alignment on line: top, center, baseline, bottom, auto
#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphTextAlign {
    pub val: ParagraphTextAlignValues,
}

impl ParagraphTextAlign {
    pub fn new(val: ParagraphTextAlignValues) -> Self {
        Self { val }
    }

    pub fn change_value(&mut self, val: ParagraphTextAlignValues) {
        self.val = val;
    }

    pub fn value(&self) -> String {
        self.val.to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphTextAlignValues {
    Top,
    Center,
    Baseline,
    Bottom,
    Auto,
}

impl fmt::Display for ParagraphTextAlignValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParagraphTextAlignValues::Top => write!(f, "top"),
            ParagraphTextAlignValues::Center => write!(f, "center"),
            ParagraphTextAlignValues::Baseline => write!(f, "baseline"),
            ParagraphTextAlignValues::Bottom => write!(f, "bottom"),
            ParagraphTextAlignValues::Auto => write!(f, "auto"),
        }
    }
}
