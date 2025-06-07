use crate::elements::*;
use crate::errors::RudocxError;
use crate::rels::{bp, generate_doc_rels};
use crate::xml::*;

use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use zip::write::FileOptions;
use zip::{ZipArchive, ZipWriter};

pub fn load<P: AsRef<Path>>(path: P) -> Result<Document, RudocxError> {
    let file = File::open(path.as_ref()).map_err(RudocxError::IoError)?;
    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader).map_err(RudocxError::ZipError)?;

    let mut document_file = archive
        .by_name(bp::DOCUMENT_XML_PATH)
        .map_err(|_| RudocxError::MissingPart(bp::DOCUMENT_XML_PATH.to_string()))?;

    let mut xml_content = String::new();
    document_file
        .read_to_string(&mut xml_content)
        .map_err(RudocxError::IoError)?;

    parse(&xml_content)
}

// Helper function to parse the actual XML content

pub fn save<P: AsRef<Path>>(document: &Document, path: P) -> Result<(), RudocxError> {
    let file = File::create(path.as_ref()).map_err(RudocxError::IoError)?;
    let mut zip = ZipWriter::new(file);
    let options: FileOptions<'_, ()> = FileOptions::default();

    // Write boilerplate files
    zip.start_file("_rels/.rels", options)?;
    zip.write_all(bp::RELS_XML_CONTENT.as_bytes())?;

    zip.start_file("[Content_Types].xml", options)?;
    zip.write_all(bp::CONTENT_TYPES_XML_CONTENT.as_bytes())?;

    // Ensure word/_rels directory exists implicitly via path
    zip.start_file("word/_rels/document.xml.rels", options)?;
    zip.write_all(generate_doc_rels(&mut String::with_capacity(4096)).as_bytes())?;

    // Generate and write word/document.xml
    let document_xml = generate(document)?;
    zip.start_file(bp::DOCUMENT_XML_PATH, options)?;
    zip.write_all(document_xml.as_bytes())?;

    zip.finish().map_err(RudocxError::ZipError)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_simple_doc() {
        let original_doc = Document {
            paragraphs: vec![
                Paragraph {
                    children: vec![
                        ParagraphChild::Run(Run {
                            properties: RunProperties::default(),
                            text: "Hello ".to_string(),
                            space_preserve: false,
                        }),
                        ParagraphChild::Run(Run {
                            properties: RunProperties {
                                bold: true,
                                italic: false,
                                underline: None,
                                color: None,
                                size: None,
                                font: None,
                                highlight: None,
                                strike: false,
                                dstrike: false,
                                valign: None,
                                spacing: None,
                            },
                            text: "World".to_string(),
                            space_preserve: false,
                        }),
                        ParagraphChild::Run(Run {
                            properties: RunProperties {
                                bold: false,
                                italic: false,
                                underline: None,
                                color: Some(HexColor::new("FF0000")), // Red
                                size: None,
                                font: None,
                                highlight: None,
                                strike: false,
                                dstrike: false,
                                valign: None,
                                spacing: None,
                            },
                            text: " Red!".to_string(),
                            space_preserve: false,
                        }),
                    ],
                },
                Paragraph {
                    children: vec![ParagraphChild::Run(Run {
                        properties: RunProperties {
                            bold: false,
                            italic: true,
                            underline: None,
                            color: None,
                            size: None,
                            font: None,
                            highlight: None,
                            strike: false,
                            dstrike: false,
                            valign: None,
                            spacing: None,
                        },
                        text: "This is italic.".to_string(),
                        space_preserve: false,
                    })],
                },
                Paragraph {
                    children: vec![
                        ParagraphChild::Hyperlink(Hyperlink::new(
                            "https://github.com/cmgsk/rudocx",
                        )),
                        ParagraphChild::Run(Run {
                            properties: RunProperties::default(),
                            text: " That was hyperlink.".to_string(),
                            space_preserve: false,
                        }),
                    ],
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
