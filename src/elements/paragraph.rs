use crate::elements::{Hyperlink, ParagraphProperties, Run};

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphChild {
    Run(Run),
    Hyperlink(Hyperlink),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Paragraph {
    pub properties: ParagraphProperties,
    pub children: Vec<ParagraphChild>,
}
