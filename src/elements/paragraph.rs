use crate::elements::{Hyperlink, Run};

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphChild {
    Run(Run),
    Hyperlink(Hyperlink),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Paragraph {
    pub children: Vec<ParagraphChild>,
}
