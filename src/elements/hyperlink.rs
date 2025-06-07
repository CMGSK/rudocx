use crate::elements::Run;
use crate::rels::generate_rid;

#[derive(Debug, Clone, PartialEq)]
pub struct Hyperlink {
    pub id: String,
    pub runs: Vec<Run>,
}

impl Default for Hyperlink {
    fn default() -> Self {
        Self {
            id: String::new(),
            runs: Vec::new(),
        }
    }
}

impl Hyperlink {
    pub fn new(target: &str) -> Self {
        let id = generate_rid(target);

        Self {
            id,
            runs: vec![Run::from(target.to_string())],
        }
    }
}
