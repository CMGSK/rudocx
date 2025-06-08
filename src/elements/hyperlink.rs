use crate::elements::Run;
use crate::rels::RelationshipManager;

/// Represents a hyperlink in a DOCX document.
/// 
/// A hyperlink consists of:
/// - An ID that references a relationship in the document's relationship manager
/// - One or more runs containing the display text (which can be formatted)
/// 
/// # Examples
/// 
/// ```rust
/// use rudocx::elements::{Hyperlink, Document};
/// 
/// let mut document = Document::default();
/// 
/// // Simple hyperlink with URL as display text
/// let link1 = Hyperlink::new("https://example.com", &mut document.relationship_manager);
/// 
/// // Hyperlink with custom display text
/// let link2 = Hyperlink::new_with_text(
///     "https://rust-lang.org", 
///     "The Rust Programming Language", 
///     &mut document.relationship_manager
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Hyperlink {
    /// The relationship ID that links to the target URL
    pub id: String,
    /// The runs containing the display text (can be formatted)
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
    /// Create a new hyperlink with a relationship manager.
    /// The display text will be the same as the target URL.
    /// 
    /// # Arguments
    /// 
    /// * `target` - The URL or target of the hyperlink
    /// * `relationship_manager` - Mutable reference to the document's relationship manager
    /// 
    /// # Returns
    /// 
    /// A new `Hyperlink` instance with an auto-generated relationship ID
    pub fn new(target: &str, relationship_manager: &mut RelationshipManager) -> Self {
        let id = relationship_manager.generate_rid(target);

        Self {
            id,
            runs: vec![Run::from(target.to_string())],
        }
    }

    /// Create a new hyperlink with custom display text.
    /// 
    /// # Arguments
    /// 
    /// * `target` - The URL or target of the hyperlink
    /// * `display_text` - The text to display for the hyperlink
    /// * `relationship_manager` - Mutable reference to the document's relationship manager
    pub fn new_with_text(target: &str, display_text: &str, relationship_manager: &mut RelationshipManager) -> Self {
        let id = relationship_manager.generate_rid(target);

        Self {
            id,
            runs: vec![Run::from(display_text.to_string())],
        }
    }

    /// Create a new hyperlink with custom runs (for formatted text).
    /// 
    /// # Arguments
    /// 
    /// * `target` - The URL or target of the hyperlink
    /// * `runs` - Vector of runs that will make up the hyperlink display text
    /// * `relationship_manager` - Mutable reference to the document's relationship manager
    pub fn new_with_runs(target: &str, runs: Vec<Run>, relationship_manager: &mut RelationshipManager) -> Self {
        let id = relationship_manager.generate_rid(target);

        Self { id, runs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rels::RelationshipManager;

    #[test]
    fn test_hyperlink_new() {
        let mut rel_manager = RelationshipManager::new();
        let hyperlink = Hyperlink::new("https://example.com", &mut rel_manager);
        
        assert_eq!(hyperlink.id, "rId1");
        assert_eq!(hyperlink.runs.len(), 1);
        assert_eq!(hyperlink.runs[0].text, "https://example.com");
        assert_eq!(rel_manager.get_links().len(), 1);
        assert_eq!(rel_manager.get_links().get("rId1"), Some(&"https://example.com".to_string()));
    }

    #[test]
    fn test_hyperlink_new_with_text() {
        let mut rel_manager = RelationshipManager::new();
        let hyperlink = Hyperlink::new_with_text(
            "https://rust-lang.org", 
            "Rust Programming Language", 
            &mut rel_manager
        );
        
        assert_eq!(hyperlink.id, "rId1");
        assert_eq!(hyperlink.runs.len(), 1);
        assert_eq!(hyperlink.runs[0].text, "Rust Programming Language");
        assert_eq!(rel_manager.get_links().get("rId1"), Some(&"https://rust-lang.org".to_string()));
    }

    #[test]
    fn test_hyperlink_new_with_runs() {
        let mut rel_manager = RelationshipManager::new();
        let runs = vec![
            Run::from("First part ".to_string()),
            Run::from("Second part".to_string()),
        ];
        let hyperlink = Hyperlink::new_with_runs("https://example.com", runs, &mut rel_manager);
        
        assert_eq!(hyperlink.id, "rId1");
        assert_eq!(hyperlink.runs.len(), 2);
        assert_eq!(hyperlink.runs[0].text, "First part ");
        assert_eq!(hyperlink.runs[1].text, "Second part");
    }

    #[test]
    fn test_multiple_hyperlinks_sequential_ids() {
        let mut rel_manager = RelationshipManager::new();
        
        let hyperlink1 = Hyperlink::new("https://example1.com", &mut rel_manager);
        let hyperlink2 = Hyperlink::new("https://example2.com", &mut rel_manager);
        let hyperlink3 = Hyperlink::new("https://example3.com", &mut rel_manager);
        
        assert_eq!(hyperlink1.id, "rId1");
        assert_eq!(hyperlink2.id, "rId2");
        assert_eq!(hyperlink3.id, "rId3");
        assert_eq!(rel_manager.get_links().len(), 3);
    }

    #[test]
    fn test_relationship_manager_isolation() {
        // Test that different relationship managers don't interfere with each other
        let mut rel_manager1 = RelationshipManager::new();
        let mut rel_manager2 = RelationshipManager::new();
        
        let hyperlink1 = Hyperlink::new("https://example1.com", &mut rel_manager1);
        let hyperlink2 = Hyperlink::new("https://example2.com", &mut rel_manager2);
        
        // Both should get rId1 since they use different managers
        assert_eq!(hyperlink1.id, "rId1");
        assert_eq!(hyperlink2.id, "rId1");
        
        // Each manager should only have one relationship
        assert_eq!(rel_manager1.get_links().len(), 1);
        assert_eq!(rel_manager2.get_links().len(), 1);
    }

    #[test]
    fn test_default_hyperlink() {
        let hyperlink = Hyperlink::default();
        assert_eq!(hyperlink.id, "");
        assert_eq!(hyperlink.runs.len(), 0);
    }
}
