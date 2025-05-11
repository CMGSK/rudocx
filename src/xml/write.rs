use crate::elements::{Document, FontType, Paragraph, Run, RunProperties};
use crate::errors::RudocxError;

use quick_xml::events::BytesText;
use quick_xml::Writer;
use std::io::Cursor;

// Helper function to generate the word/document.xml content
#[deprecated]
pub fn generate_document_xml(document: &Document) -> Result<String, RudocxError> {
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
                                                if r.properties.bold {
                                                    writer.create_element("w:b").write_empty()?;
                                                }
                                                if r.properties.italic {
                                                    writer.create_element("w:i").write_empty()?;
                                                }
                                                if let Some(color) = &r.properties.color {
                                                    writer
                                                        .create_element("w:color")
                                                        .with_attribute((
                                                            "w:val",
                                                            color.value().as_str(),
                                                        ))
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

pub fn generate(document: &Document) -> Result<String, RudocxError> {
    write_document_xml(document)
}

fn write_document_xml(document: &Document) -> Result<String, RudocxError> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    writer
        .create_element("w:document")
        .with_attribute((
            "xmlns:w",
            "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
        ))
        .write_inner_content(|writer| {
            write_body_xml(writer, document)?;
            Ok(())
        })?;

    let xml_bytes = writer.into_inner().into_inner();
    String::from_utf8(xml_bytes).map_err(RudocxError::Utf8Error)
}

fn write_body_xml(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    document: &Document,
) -> Result<(), RudocxError> {
    writer
        .create_element("w:body")
        .write_inner_content(|writer| {
            for p in &document.paragraphs {
                write_paragraph_xml(writer, p)?;
            }
            Ok(())
        })?;
    Ok(())
}

fn write_paragraph_xml(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    paragraph: &Paragraph,
) -> Result<(), RudocxError> {
    writer.create_element("w:p").write_inner_content(|writer| {
        for r in &paragraph.runs {
            write_run_xml(writer, r)?;
        }
        Ok(())
    })?;
    Ok(())
}

fn write_run_xml(writer: &mut Writer<Cursor<Vec<u8>>>, run: &Run) -> Result<(), RudocxError> {
    writer.create_element("w:p").write_inner_content(|writer| {
        write_run_properties_xml(writer, &run.properties)?;
        writer
            .create_element("w:t")
            .write_text_content(BytesText::new(&run.text))?;
        Ok(())
    })?;
    Ok(())
}

fn write_run_properties_xml(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    properties: &RunProperties,
) -> Result<(), RudocxError> {
    if properties.has_formatting() {
        writer
            .create_element("w:rPr")
            .write_inner_content(|writer| {
                // Start run properties

                if properties.bold {
                    writer.create_element("w:b").write_empty()?;
                }

                if properties.italic {
                    writer.create_element("w:i").write_empty()?;
                }

                if let Some(underline) = &properties.underline {
                    writer
                        .create_element("w:u")
                        .with_attribute(("w:val", underline.value().as_str()))
                        .write_empty()?;
                }

                if let Some(color) = &properties.color {
                    writer
                        .create_element("w:color")
                        .with_attribute(("w:val", color.value().as_str()))
                        .write_empty()?;
                }

                if let Some(size) = &properties.size {
                    writer
                        .create_element("w:sz")
                        .with_attribute(("w:val", size.to_string().as_str()))
                        .write_empty()?;
                }

                //TODO: Handle multiple types. Right now only the hint is written.
                if let Some(font_set) = &properties.font {
                    match font_set.get_hint_value() {
                        FontType::Default => (),
                        _ => {
                            let attr = &format!("w:{}", font_set.get_hint_value().to_string());
                            if let Ok(val) = font_set.get_hint() {
                                writer
                                    .create_element("w:rFonts")
                                    .with_attribute((attr.as_str(), val.as_str()))
                                    .write_empty()?;
                            }
                        }
                    }
                }

                if let Some(highlight) = &properties.highlight {
                    writer
                        .create_element("w:highlight")
                        .with_attribute(("w:val", highlight.value().as_str()))
                        .write_empty()?;
                }

                if properties.strike {
                    writer.create_element("w:strike").write_empty()?;
                }

                if properties.dstrike {
                    writer.create_element("w:dstrike").write_empty()?;
                }

                if let Some(valign) = &properties.valign {
                    writer
                        .create_element("w:vertAlign")
                        .with_attribute(("w:val", valign.value().as_str()))
                        .write_empty()?;
                }

                if let Some(spacing) = &properties.spacing {
                    writer
                        .create_element("w:spacing")
                        .with_attribute(("w:val", spacing.to_string().as_str()))
                        .write_empty()?;
                }

                // End of run properties
                Ok(())
            })?;
    }
    Ok(())
}
