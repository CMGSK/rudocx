use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

static RIDS: OnceLock<Mutex<u32>> = OnceLock::new();

fn rids() -> &'static Mutex<u32> {
    RIDS.get_or_init(|| Mutex::new(1))
}

static LINKS: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

fn links() -> &'static Mutex<HashMap<String, String>> {
    LINKS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn generate_rid(t: &str) -> String {
    let mut current = rids().lock().unwrap();
    *current += 1;

    let rid = format!("rId{}", current);

    let mut links = links().lock().unwrap();
    links.insert(rid.clone(), t.to_string());

    rid
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
