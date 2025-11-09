use std::fmt;

use crate::elements::HexColor;

/// Contains top, bottom, left, right, between, and bar border definitions
#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphBorder {
    pub top: Option<ParagraphBorderSide>,
    pub bottom: Option<ParagraphBorderSide>,
    pub left: Option<ParagraphBorderSide>,
    pub right: Option<ParagraphBorderSide>,
    pub between: Option<ParagraphBorderSide>,
}

impl Default for ParagraphBorder {
    fn default() -> Self {
        Self {
            top: Some(ParagraphBorderSide::default()),
            bottom: Some(ParagraphBorderSide::default()),
            left: Some(ParagraphBorderSide::default()),
            right: Some(ParagraphBorderSide::default()),
            between: None,
        }
    }
}

impl ParagraphBorder {
    pub fn new(
        top: Option<ParagraphBorderSide>,
        bottom: Option<ParagraphBorderSide>,
        left: Option<ParagraphBorderSide>,
        right: Option<ParagraphBorderSide>,
        between: Option<ParagraphBorderSide>,
    ) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
            between,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphBorderSide {
    pub val: Option<ParagraphBorderStyle>,
    pub sz: Option<u8>,
    pub space: Option<u8>,
    pub color: Option<HexColor>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphBorderStyle {
    Single,
    Double,
    Dashed,
    Nil,
    // Note: None is defined by the None value of the Option containing this enum
}

impl fmt::Display for ParagraphBorderStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParagraphBorderStyle::Single => write!(f, "single"),
            ParagraphBorderStyle::Double => write!(f, "double"),
            ParagraphBorderStyle::Dashed => write!(f, "dashed"),
            ParagraphBorderStyle::Nil => write!(f, "nil"),
        }
    }
}

impl Default for ParagraphBorderSide {
    fn default() -> Self {
        Self {
            val: Some(ParagraphBorderStyle::Single),
            sz: Some(4),
            space: None,
            color: Some(HexColor::new("FFFFFF")),
        }
    }
}

impl ParagraphBorderSide {
    pub fn new(
        val: Option<ParagraphBorderStyle>,
        sz: Option<u8>,
        space: Option<u8>,
        color: Option<HexColor>,
    ) -> Self {
        Self {
            val,
            sz,
            space,
            color,
        }
    }
}
