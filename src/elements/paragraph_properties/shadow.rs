use std::fmt;

use crate::{
    elements::{HexColor, PercentFill, StripePattern},
    errors::RudocxParagraphStyleError,
};

/// Contains fill color, pattern, and background color for paragraph shading
#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphShading {
    pub val: ParagraphShadingValues,
    pub fill: Option<HexColor>,
    pub color: Option<HexColor>, // ignored for Clear
    //TODO: To be implemented
    _theme_color: Option<String>,
    _theme_fill: Option<String>,
}

impl Default for ParagraphShading {
    fn default() -> Self {
        Self {
            val: ParagraphShadingValues::Clear,
            fill: Some(HexColor::new("FFF700")), // Yellow
            color: None,
            _theme_color: None,
            _theme_fill: None,
        }
    }
}

impl ParagraphShading {
    pub fn new(val: ParagraphShadingValues, fill: HexColor, color: HexColor) -> Self {
        Self {
            val,
            fill: Some(fill),
            color: Some(color),
            _theme_color: None,
            _theme_fill: None,
        }
    }

    pub fn change_value(&mut self, val: ParagraphShadingValues) {
        self.val = val;
    }

    pub fn change_fill(&mut self, fill: HexColor) {
        self.fill = Some(fill);
    }

    pub fn change_color(&mut self, color: HexColor) {
        self.color = Some(color);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphShadingValues {
    Clear, // Solid color
    Percentage(PercentFill),
    Pattern(StripePattern),
    Nil,
}

impl fmt::Display for ParagraphShadingValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParagraphShadingValues::Clear => write!(f, "clear"),
            ParagraphShadingValues::Percentage(v) => write!(f, "{}", v),
            ParagraphShadingValues::Pattern(v) => write!(f, "{}", v),
            ParagraphShadingValues::Nil => write!(f, "nil"),
        }
    }
}

impl TryFrom<String> for ParagraphShadingValues {
    type Error = RudocxParagraphStyleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "clear" => Ok(ParagraphShadingValues::Clear),
            "nil" => Ok(ParagraphShadingValues::Nil),
            s if s.starts_with("pct") => {
                if let Ok(pct) = s[3..].parse::<u8>() {
                    Ok(ParagraphShadingValues::Percentage(PercentFill::new(pct)))
                } else {
                    Err(RudocxParagraphStyleError::InvalidShading(s.to_string()))
                }
            }
            s => Ok(ParagraphShadingValues::Pattern(StripePattern::new(
                s.into(),
            ))),
        }
    }
}
