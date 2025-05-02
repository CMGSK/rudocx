mod error;
use error::RudocxError;
use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::{BytesText, Event};
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Write};
use std::path::Path;
use zip::ZipArchive;
use zip::write::{FileOptions, ZipWriter};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct RunProperties {
    pub is_bold: bool,
    pub is_italic: bool,
    pub font_color: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Run {
    pub properties: RunProperties,
    pub text: String,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Paragraph {
    pub runs: Vec<Run>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Document {
    pub paragraphs: Vec<Paragraph>,
}

impl RunProperties {
    pub fn has_formatting(&self) -> bool {
        self.is_bold || self.is_italic || self.font_color.is_some()
    }
}

const DOCUMENT_XML_PATH: &str = "word/document.xml";

// Boilerplate XML content
const RELS_XML_CONTENT: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
</Relationships>"#;

const CONTENT_TYPES_XML_CONTENT: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
    <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
    <Default Extension="xml" ContentType="application/xml"/>
    <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
</Types>"#;

// Minimal document rels - can be expanded later if images, hyperlinks etc. are added
const DOC_RELS_XML_CONTENT: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
</Relationships>"#;

pub fn load<P: AsRef<Path>>(path: P) -> Result<Document, RudocxError> {
    let file = File::open(path.as_ref()).map_err(RudocxError::IoError)?;
    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader).map_err(RudocxError::ZipError)?;

    let mut document_file = archive
        .by_name(DOCUMENT_XML_PATH)
        .map_err(|_| RudocxError::MissingPart(DOCUMENT_XML_PATH.to_string()))?;

    let mut xml_content = String::new();
    document_file
        .read_to_string(&mut xml_content)
        .map_err(RudocxError::IoError)?;

    parse_document_xml(&xml_content)
}

// Helper function to parse the actual XML content
fn parse_document_xml(xml_content: &str) -> Result<Document, RudocxError> {
    let mut reader = Reader::from_str(xml_content);
    let mut buf = Vec::new();
    let mut document = Document::default();
    let mut current_paragraph: Option<Paragraph> = None;
    let mut current_run: Option<Run> = None;
    let mut current_run_properties: Option<RunProperties> = None;
    let mut is_in_run_properties = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name().as_ref() {
                b"w:p" => {
                    if let Some(p) = current_paragraph.take() {
                        document.paragraphs.push(p);
                    }
                    current_paragraph = Some(Paragraph::default());
                }
                b"w:r" => {
                    if let Some(r) = current_run.take() {
                        if let Some(ref mut p) = current_paragraph {
                            p.runs.push(r);
                        }
                    }
                    current_run_properties = Some(RunProperties::default());
                    current_run = Some(Run {
                        properties: RunProperties::default(),
                        text: String::new(),
                    });
                }
                b"w:rPr" => {
                    is_in_run_properties = true;
                }
                b"w:t" => {}
                _ => (),
            },
            Ok(Event::Empty(ref e)) => match e.name().as_ref() {
                b"w:b" => {
                    if is_in_run_properties {
                        if let Some(ref mut props) = current_run_properties {
                            props.is_bold = true;
                        }
                    }
                }
                b"w:i" => {
                    if is_in_run_properties {
                        if let Some(ref mut props) = current_run_properties {
                            props.is_italic = true;
                        }
                    }
                }
                b"w:color" => {
                    if is_in_run_properties {
                        if let Some(ref mut props) = current_run_properties {
                            for attr_result in e.attributes() {
                                if let Ok(attr) = attr_result {
                                    if attr.key.as_ref() == b"w:val" {
                                        if let Ok(val) =
                                            attr.decode_and_unescape_value(reader.decoder())
                                        {
                                            props.font_color = Some(val.into_owned());
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => (),
            },
            Ok(Event::Text(e)) => {
                if let Some(ref mut run) = current_run {
                    run.text.push_str(&e.unescape()?.to_string());
                }
            }
            Ok(Event::End(ref e)) => match e.name().as_ref() {
                b"w:p" => {
                    if let Some(p) = current_paragraph.take() {
                        if let Some(r) = current_run.take() {
                            if let Some(mut current_p) = Some(p) {
                                current_p.runs.push(r);
                                document.paragraphs.push(current_p);
                            }
                        } else {
                            document.paragraphs.push(p);
                        }
                    }
                    current_paragraph = None;
                }
                b"w:r" => {
                    if let Some(mut run) = current_run.take() {
                        if let Some(props) = current_run_properties.take() {
                            run.properties = props;
                        }
                        if let Some(ref mut p) = current_paragraph {
                            p.runs.push(run);
                        }
                    }
                    current_run = None;
                    current_run_properties = None;
                }
                b"w:rPr" => {
                    is_in_run_properties = false;
                }
                _ => (),
            },
            Ok(Event::Eof) => {
                if let Some(p) = current_paragraph.take() {
                    if let Some(r) = current_run.take() {
                        if let Some(mut current_p) = Some(p) {
                            current_p.runs.push(r);
                            document.paragraphs.push(current_p);
                        }
                    } else {
                        document.paragraphs.push(p);
                    }
                }
                break;
            }
            Err(e) => return Err(RudocxError::XmlError(e)),
            _ => (),
        }
        buf.clear();
    }

    Ok(document)
}

// Helper function to generate the word/document.xml content
fn generate_document_xml(document: &Document) -> Result<String, RudocxError> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    writer
        .create_element("w:document")
        .with_attribute((
            "xmlns:w",
            "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
        ))
        .write_inner_content(|writer| {
            writer
                .create_element("w:body")
                .write_inner_content(|writer| {
                    for p in &document.paragraphs {
                        writer.create_element("w:p").write_inner_content(|writer| {
                            for r in &p.runs {
                                writer.create_element("w:r").write_inner_content(|writer| {
                                    if r.properties.has_formatting() {
                                        writer.create_element("w:rPr").write_inner_content(
                                            |writer| {
                                                if r.properties.is_bold {
                                                    writer.create_element("w:b").write_empty()?;
                                                }
                                                if r.properties.is_italic {
                                                    writer.create_element("w:i").write_empty()?;
                                                }
                                                if let Some(color) = &r.properties.font_color {
                                                    writer
                                                        .create_element("w:color")
                                                        .with_attribute(("w:val", color.as_str()))
                                                        .write_empty()?;
                                                }
                                                Ok(())
                                            },
                                        )?;
                                    }
                                    writer
                                        .create_element("w:t")
                                        .write_text_content(BytesText::new(&r.text))?;
                                    Ok(())
                                })?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            Ok(())
        })?;

    let xml_bytes = writer.into_inner().into_inner();
    String::from_utf8(xml_bytes).map_err(RudocxError::Utf8Error)
}

pub fn save<P: AsRef<Path>>(document: &Document, path: P) -> Result<(), RudocxError> {
    let file = File::create(path.as_ref()).map_err(RudocxError::IoError)?;
    let mut zip = ZipWriter::new(file);
    let options: FileOptions<'_, ()> = FileOptions::default();

    // Write boilerplate files
    zip.start_file("_rels/.rels", options)?;
    zip.write_all(RELS_XML_CONTENT.as_bytes())?;

    zip.start_file("[Content_Types].xml", options)?;
    zip.write_all(CONTENT_TYPES_XML_CONTENT.as_bytes())?;

    // Ensure word/_rels directory exists implicitly via path
    zip.start_file("word/_rels/document.xml.rels", options)?;
    zip.write_all(DOC_RELS_XML_CONTENT.as_bytes())?;

    // Generate and write word/document.xml
    let document_xml = generate_document_xml(document)?;
    zip.start_file(DOCUMENT_XML_PATH, options)?;
    zip.write_all(document_xml.as_bytes())?;

    zip.finish().map_err(RudocxError::ZipError)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_doc() {
        let xml_input = r#"
            <w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
                <w:body>
                    <w:p>
                        <w:r><w:t>This is plain text.</w:t></w:r>
                    </w:p>
                    <w:p>
                        <w:r><w:rPr><w:b/></w:rPr><w:t>This is bold.</w:t></w:r>
                        <w:r><w:t xml:space="preserve"> </w:t></w:r>
                        <w:r><w:rPr><w:i/></w:rPr><w:t>This is italic.</w:t></w:r>
                    </w:p>
                    <w:p>
                        <w:r><w:rPr><w:b/><w:i/></w:rPr><w:t>Bold and Italic.</w:t></w:r>
                    </w:p>
                </w:body>
            </w:document>
        "#;

        let result = parse_document_xml(xml_input);
        assert!(result.is_ok());
        let doc = result.unwrap();

        assert_eq!(doc.paragraphs.len(), 3);

        // Paragraph 1: Plain text
        assert_eq!(doc.paragraphs[0].runs.len(), 1);
        assert_eq!(doc.paragraphs[0].runs[0].text, "This is plain text.");
        assert!(!doc.paragraphs[0].runs[0].properties.is_bold);
        assert!(!doc.paragraphs[0].runs[0].properties.is_italic);

        // Paragraph 2: Bold, space, Italic
        assert_eq!(doc.paragraphs[1].runs.len(), 3);
        // Run 1: Bold
        assert_eq!(doc.paragraphs[1].runs[0].text, "This is bold.");
        assert!(doc.paragraphs[1].runs[0].properties.is_bold);
        assert!(!doc.paragraphs[1].runs[0].properties.is_italic);
        // Run 2: Space (should be preserved)
        assert_eq!(doc.paragraphs[1].runs[1].text, " ");
        assert!(!doc.paragraphs[1].runs[1].properties.is_bold);
        assert!(!doc.paragraphs[1].runs[1].properties.is_italic);
        // Run 3: Italic
        assert_eq!(doc.paragraphs[1].runs[2].text, "This is italic.");
        assert!(!doc.paragraphs[1].runs[2].properties.is_bold);
        assert!(doc.paragraphs[1].runs[2].properties.is_italic);

        // Paragraph 3: Bold and Italic
        assert_eq!(doc.paragraphs[2].runs.len(), 1);
        assert_eq!(doc.paragraphs[2].runs[0].text, "Bold and Italic.");
        assert!(doc.paragraphs[2].runs[0].properties.is_bold);
        assert!(doc.paragraphs[2].runs[0].properties.is_italic);
    }

    #[test]
    fn test_save_simple_doc() {
        let original_doc = Document {
            paragraphs: vec![
                Paragraph {
                    runs: vec![
                        Run {
                            properties: RunProperties::default(),
                            text: "Hello ".to_string(),
                        },
                        Run {
                            properties: RunProperties {
                                is_bold: true,
                                is_italic: false,
                                font_color: None,
                            },
                            text: "World".to_string(),
                        },
                        Run {
                            properties: RunProperties {
                                is_bold: false,
                                is_italic: false,
                                font_color: Some("FF0000".to_string()), // Red
                            },
                            text: " Red!".to_string(),
                        },
                    ],
                },
                Paragraph {
                    runs: vec![Run {
                        properties: RunProperties {
                            is_bold: false,
                            is_italic: true,
                            font_color: None,
                        },
                        text: "This is italic.".to_string(),
                    }],
                },
            ],
        };

        let temp_file_path = std::env::temp_dir().join("rudocx_test_save.docx");

        let save_result = save(&original_doc, &temp_file_path);
        assert!(
            save_result.is_ok(),
            "Failed to save document: {:?}",
            save_result.err()
        );

        let load_result = load(&temp_file_path);
        assert!(
            load_result.is_ok(),
            "Failed to load saved document: {:?}",
            load_result.err()
        );
        let loaded_doc = load_result.unwrap();

        assert_eq!(
            original_doc, loaded_doc,
            "Loaded document does not match original"
        );

        let _ = std::fs::remove_file(&temp_file_path);
    }
}
