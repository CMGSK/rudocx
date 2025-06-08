use std::collections::HashMap;
use std::fmt::Write;

/// Manages relationships for a single document, ensuring thread-safety and preventing
/// relationship ID collisions between different documents.
/// 
/// This replaces the previous global state approach which could cause issues in
/// multi-threaded environments or when processing multiple documents.
/// 
/// # Examples
/// 
/// ```rust
/// use rudocx::rels::RelationshipManager;
/// 
/// let mut manager = RelationshipManager::new();
/// let rid = manager.generate_rid("https://example.com");
/// assert_eq!(rid, "rId1");
/// 
/// let links = manager.get_links();
/// assert_eq!(links.get("rId1"), Some(&"https://example.com".to_string()));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct RelationshipManager {
    counter: u32,
    links: HashMap<String, String>,
}

impl Default for RelationshipManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RelationshipManager {
    pub fn new() -> Self {
        Self {
            counter: 0,
            links: HashMap::new(),
        }
    }

    /// Generate a new relationship ID and store the target
    pub fn generate_rid(&mut self, target: &str) -> String {
        self.counter += 1;
        let rid = format!("rId{}", self.counter);
        self.links.insert(rid.clone(), target.to_string());
        rid
    }

    /// Get all stored relationships
    pub fn get_links(&self) -> &HashMap<String, String> {
        &self.links
    }

    /// Clear all relationships (useful for testing or document reset)
    pub fn clear(&mut self) {
        self.counter = 0;
        self.links.clear();
    }

    /// Add a relationship with a specific ID (used when loading documents)
    pub fn add_relationship(&mut self, id: String, target: String) {
        // Extract counter from ID if it follows the rId pattern
        if let Some(num_str) = id.strip_prefix("rId") {
            if let Ok(num) = num_str.parse::<u32>() {
                self.counter = self.counter.max(num);
            }
        }
        self.links.insert(id, target);
    }
}

pub fn generate_doc_rels<'a>(xml: &'a mut String, relationship_manager: &RelationshipManager) -> &'a str {
    xml.clear();
    xml.push_str(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">"#,
    );

    for (id, target) in relationship_manager.get_links() {
        if let Err(_) = write!(
            xml,
            r#"<Relationship Id="{id}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink" Target="{target}" TargetMode="External"/>"#
        ) {
            // Handle write error - for now we'll continue, but this could be improved
            eprintln!("Warning: Failed to write relationship for {}", id);
        }
    }

    xml.push_str("</Relationships>");
    xml.as_str()
}

pub mod bp {
    pub const DOCUMENT_XML_PATH: &str = "word/document.xml";

    // Boilerplate XML content
    pub const RELS_XML_CONTENT: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
</Relationships>"#;

    pub const CONTENT_TYPES_XML_CONTENT: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
    <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
    <Default Extension="xml" ContentType="application/xml"/>
    <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
</Types>"#;

    // Minimal document rels - can be expanded later if images, hyperlinks etc. are added
    pub const DOC_RELS_XML_CONTENT: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
</Relationships>"#;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_manager_new() {
        let manager = RelationshipManager::new();
        assert_eq!(manager.counter, 0);
        assert_eq!(manager.links.len(), 0);
    }

    #[test]
    fn test_generate_rid() {
        let mut manager = RelationshipManager::new();
        
        let rid1 = manager.generate_rid("https://example1.com");
        let rid2 = manager.generate_rid("https://example2.com");
        
        assert_eq!(rid1, "rId1");
        assert_eq!(rid2, "rId2");
        assert_eq!(manager.get_links().len(), 2);
        assert_eq!(manager.get_links().get("rId1"), Some(&"https://example1.com".to_string()));
        assert_eq!(manager.get_links().get("rId2"), Some(&"https://example2.com".to_string()));
    }

    #[test]
    fn test_clear() {
        let mut manager = RelationshipManager::new();
        
        manager.generate_rid("https://example1.com");
        manager.generate_rid("https://example2.com");
        assert_eq!(manager.get_links().len(), 2);
        
        manager.clear();
        assert_eq!(manager.counter, 0);
        assert_eq!(manager.get_links().len(), 0);
        
        // After clearing, should start from rId1 again
        let rid = manager.generate_rid("https://example3.com");
        assert_eq!(rid, "rId1");
    }

    #[test]
    fn test_add_relationship() {
        let mut manager = RelationshipManager::new();
        
        manager.add_relationship("rId5".to_string(), "https://example.com".to_string());
        assert_eq!(manager.counter, 5);
        assert_eq!(manager.get_links().get("rId5"), Some(&"https://example.com".to_string()));
        
        // Next generated ID should be rId6
        let next_rid = manager.generate_rid("https://example2.com");
        assert_eq!(next_rid, "rId6");
    }

    #[test]
    fn test_add_relationship_non_standard_id() {
        let mut manager = RelationshipManager::new();
        
        // Non-standard ID shouldn't affect counter
        manager.add_relationship("customId".to_string(), "https://example.com".to_string());
        assert_eq!(manager.counter, 0);
        
        let next_rid = manager.generate_rid("https://example2.com");
        assert_eq!(next_rid, "rId1");
    }

    #[test]
    fn test_generate_doc_rels() {
        let mut manager = RelationshipManager::new();
        manager.generate_rid("https://example1.com");
        manager.generate_rid("https://example2.com");
        
        let mut xml = String::new();
        let result = generate_doc_rels(&mut xml, &manager);
        
        assert!(!result.is_empty());
        assert!(result.contains("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>"));
        assert!(result.contains("xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\""));
        assert!(result.contains("rId1"));
        assert!(result.contains("rId2"));
        assert!(result.contains("https://example1.com"));
        assert!(result.contains("https://example2.com"));
        assert!(result.contains("TargetMode=\"External\""));
    }

    #[test]
    fn test_generate_doc_rels_empty() {
        let manager = RelationshipManager::new();
        let mut xml = String::new();
        let result = generate_doc_rels(&mut xml, &manager);
        
        assert!(!result.is_empty());
        assert!(result.contains("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>"));
        assert!(result.contains("</Relationships>"));
        // Should not contain any relationship entries
        assert!(!result.contains("rId"));
    }
}
