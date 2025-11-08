use crate::elements::{HexColor, PercentFill, StripePattern};

#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphShadow {
    pub val: ParagraphShadowValues,
    pub fill: Option<HexColor>,
    pub color: Option<HexColor>, // ignored for Clear
    //TODO: To be implemented
    _theme_color: Option<String>,
    _theme_fill: Option<String>,
}

impl Default for ParagraphShadow {
    fn default() -> Self {
        Self {
            val: ParagraphShadowValues::Clear,
            fill: Some(HexColor::new("FFF700")), // Yellow
            color: None,
            _theme_color: None,
            _theme_fill: None,
        }
    }
}

impl ParagraphShadow {
    fn new(val: ParagraphShadowValues, fill: HexColor, color: HexColor) -> Self {
        Self {
            val,
            fill: Some(fill),
            color: Some(color),
            _theme_color: None,
            _theme_fill: None,
        }
    }

    pub fn change_value(&mut self, val: ParagraphShadowValues) {
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
pub enum ParagraphShadowValues {
    Clear, // Solid color
    Percentage(PercentFill),
    Pattern(StripePattern),
    Nil,
}
