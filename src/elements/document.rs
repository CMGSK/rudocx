use crate::elements::paragraph::Paragraph;
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Document {
    pub paragraphs: Vec<Paragraph>,
}
