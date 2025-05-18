use crate::elements::Run;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Paragraph {
    pub runs: Vec<Run>,
}
