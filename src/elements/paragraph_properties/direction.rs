use std::fmt;

/// Contains text flow direction: lr-tb, tb-rl, bt-lr, lr-tb-v, tb-rl-v, tb-lr-v
#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphTextDir {
    pub val: ParagraphTextDirValues,
}

impl ParagraphTextDir {
    pub fn new(val: ParagraphTextDirValues) -> Self {
        Self { val }
    }

    pub fn change_value(&mut self, val: ParagraphTextDirValues) {
        self.val = val;
    }

    pub fn value(&self) -> String {
        self.val.to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphTextDirValues {
    LrTb,     // Left to right, top to bottom
    TbRlTbLr, // top to bottom, right to left, then top to bottom
    BtLr,     // bottom to top, left to right
    TbLrTbRl, // top to bottom, left to right, then top to bottom
    TbRl,     // top to bottom, right to left
    Lr,       // left to right
    LrTbBidi, // left to right, top to bottom, bidirectional
}

impl fmt::Display for ParagraphTextDirValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParagraphTextDirValues::LrTb => write!(f, "lrTb"),
            ParagraphTextDirValues::TbRlTbLr => write!(f, "tbRlTbLr"),
            ParagraphTextDirValues::BtLr => write!(f, "brLr"),
            ParagraphTextDirValues::TbLrTbRl => write!(f, "tbLrTbRl"),
            ParagraphTextDirValues::TbRl => write!(f, "tbRl"),
            ParagraphTextDirValues::Lr => write!(f, "lr"),
            ParagraphTextDirValues::LrTbBidi => write!(f, "lrTbBidi"),
        }
    }
}
