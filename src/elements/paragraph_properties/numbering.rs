/// Contains numbering level (ilvl) and numbering definition ID (numId)
///
/// Technically, ilvl is unbounded but it's typically a 0-8 value. Also, num_id is a reference into
/// the document's `numbering.xml` file. Both fields are by themselves a tag with an only attribute
/// called `w:val`
#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphNumberingProperties {
    pub ilvl: u8,
    pub num_id: u32,
}

impl ParagraphNumberingProperties {
    pub fn new(ilvl: u8, num_id: u32) -> Self {
        Self { ilvl, num_id }
    }

    pub fn change_ilvl(&mut self, ilvl: u8) {
        self.ilvl = ilvl;
    }

    pub fn change_num_id(&mut self, num_id: u32) {
        self.num_id = num_id;
    }
}
