use crate::elements::{Document, FontType, Paragraph, Run, RunProperties};
use crate::errors::RudocxError;

use quick_xml::Writer;
use quick_xml::events::BytesText;
use std::io::Cursor;

type XmlWriter = Writer<Cursor<Vec<u8>>>;
type XmlResult = std::io::Result<()>;

pub fn generate(document: &Document) -> Result<String, RudocxError> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    let element = writer.create_element("w:document");
    element
        .with_attribute((
            "xmlns:w",
            "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
        ))
        .write_inner_content(|writer| write_body(writer, document))
        .map_err(|e| RudocxError::XmlError(e.into()))?;

    let xml_bytes = writer.into_inner().into_inner();
    String::from_utf8(xml_bytes).map_err(RudocxError::Utf8Error)
}

fn write_body(writer: &mut XmlWriter, document: &Document) -> XmlResult {
    let element = writer.create_element("w:body");
    element.write_inner_content(|writer| {
        for paragraph in &document.paragraphs {
            write_paragraph(writer, paragraph)?;
        }
        Ok(())
    })?;
    Ok(())
}

fn write_paragraph(writer: &mut XmlWriter, paragraph: &Paragraph) -> XmlResult {
    let element = writer.create_element("w:p");
    element.write_inner_content(|writer| {
        for run in &paragraph.runs {
            write_run(writer, run)?;
        }
        Ok(())
    })?;
    Ok(())
}

fn write_run(writer: &mut XmlWriter, run: &Run) -> XmlResult {
    let element = writer.create_element("w:r");
    element.write_inner_content(|writer| {
        if run.properties.has_formatting() {
            write_run_properties(writer, &run.properties)?;
        }

        if run.space_preserve {
            let element = writer.create_element("w:t");
            element
                .with_attribute(("xml:space", "preserve"))
                .write_text_content(BytesText::new(&run.text))?;
        } else {
            let element = writer.create_element("w:t");
            element.write_text_content(BytesText::new(&run.text))?;
        }

        Ok(())
    })?;
    Ok(())
}

fn write_run_properties(writer: &mut XmlWriter, properties: &RunProperties) -> XmlResult {
    let element = writer.create_element("w:rPr");
    element.write_inner_content(|writer| {
        for (condition, element_name) in [
            (properties.bold, "w:b"),
            (properties.italic, "w:i"),
            (properties.strike, "w:strike"),
            (properties.dstrike, "w:dstrike"),
        ] {
            if condition {
                writer.create_element(element_name).write_empty()?;
            }
        }

        if let Some(underline) = &properties.underline {
            write_attribute_element(writer, "w:u", "w:val", &underline.value())?;
        }

        if let Some(color) = &properties.color {
            write_attribute_element(writer, "w:color", "w:val", &color.value())?;
        }

        if let Some(size) = &properties.size {
            write_attribute_element(writer, "w:sz", "w:val", &size.to_string())?;
        }

        if let Some(font_set) = &properties.font {
            match font_set.get_hint_value() {
                FontType::Default => (),
                _ => {
                    if let Ok(val) = font_set.get_hint() {
                        let attr = format!("w:{}", font_set.get_hint_value());
                        let mut font_element = writer.create_element("w:rFonts");
                        font_element = font_element.with_attribute((attr.as_str(), val.as_str()));
                        font_element.write_empty()?;
                    }
                }
            }
        }

        if let Some(highlight) = &properties.highlight {
            write_attribute_element(writer, "w:highlight", "w:val", &highlight.value())?;
        }

        if let Some(valign) = &properties.valign {
            write_attribute_element(writer, "w:vertAlign", "w:val", &valign.value())?;
        }

        if let Some(spacing) = &properties.spacing {
            write_attribute_element(writer, "w:spacing", "w:val", &spacing.to_string())?;
        }

        Ok(())
    })?;
    Ok(())
}

fn write_attribute_element(
    writer: &mut XmlWriter,
    element: &str,
    attr_name: &str,
    attr_value: &str,
) -> XmlResult {
    let element = writer.create_element(element);
    element
        .with_attribute((attr_name, attr_value))
        .write_empty()?;
    Ok(())
}
