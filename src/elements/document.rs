use crate::elements::Paragraph;
use crate::rels::RelationshipManager;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Document {
    pub paragraphs: Vec<Paragraph>,
    pub relationship_manager: RelationshipManager,
}
