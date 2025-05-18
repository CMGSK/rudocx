use crate::elements::{Document, FontType, Paragraph, Run, RunProperties};
use crate::errors::RudocxError;

use quick_xml::Writer;
use quick_xml::events::BytesText;
use std::io::Cursor;

type XmlWriter = Writer<Cursor<Vec<u8>>>;
type XmlResult = std::io::Result<()>;

enum XmlNs {
    W,
}

impl XmlNs {
    fn as_str(&self) -> &'static str {
        match self {
            XmlNs::W => "xmlns:w",
        }
    }

    fn url(&self) -> &'static str {
        match self {
            XmlNs::W => "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
        }
    }
}

enum XmlElement {
    Document,
    Body,
    Paragraph,
    Run,
    RunProps,
    Text,
    Bold,
    Italic,
    Strike,
    DStrike,
    Underline,
    Color,
    Size,
    Fonts,
    Highlight,
    VertAlign,
    Spacing,
}

impl XmlElement {
    fn as_str(&self) -> &'static str {
        match self {
            XmlElement::Document => "w:document",
            XmlElement::Body => "w:body",
            XmlElement::Paragraph => "w:p",
            XmlElement::Run => "w:r",
            XmlElement::RunProps => "w:rPr",
            XmlElement::Text => "w:t",
            XmlElement::Bold => "w:b",
            XmlElement::Italic => "w:i",
            XmlElement::Strike => "w:strike",
            XmlElement::DStrike => "w:dstrike",
            XmlElement::Underline => "w:u",
            XmlElement::Color => "w:color",
            XmlElement::Size => "w:sz",
            XmlElement::Fonts => "w:rFonts",
            XmlElement::Highlight => "w:highlight",
            XmlElement::VertAlign => "w:vertAlign",
            XmlElement::Spacing => "w:spacing",
        }
    }
}

enum XmlAttr {
    Val,
    Space,
}

impl XmlAttr {
    fn as_str(&self) -> &'static str {
        match self {
            XmlAttr::Val => "w:val",
            XmlAttr::Space => "xml:space",
        }
    }
}

enum XmlAttrValue<'a> {
    Preserve,
    Custom(&'a str),
}

impl<'a> XmlAttrValue<'a> {
    fn as_str(&self) -> &str {
        match self {
            XmlAttrValue::Preserve => "preserve",
            XmlAttrValue::Custom(s) => s,
        }
    }
}

pub fn generate(document: &Document) -> Result<String, RudocxError> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    let element = writer.create_element(XmlElement::Document.as_str());
    element
        .with_attribute((XmlNs::W.as_str(), XmlNs::W.url()))
        .write_inner_content(|writer| write_body(writer, document))
        .map_err(|e| RudocxError::XmlError(e.into()))?;

    let xml_bytes = writer.into_inner().into_inner();
    String::from_utf8(xml_bytes).map_err(RudocxError::Utf8Error)
}

fn write_body(writer: &mut XmlWriter, document: &Document) -> XmlResult {
    let element = writer.create_element(XmlElement::Body.as_str());
    element.write_inner_content(|writer| {
        for paragraph in &document.paragraphs {
            write_paragraph(writer, paragraph)?;
        }
        Ok(())
    })?;
    Ok(())
}

fn write_paragraph(writer: &mut XmlWriter, paragraph: &Paragraph) -> XmlResult {
    let element = writer.create_element(XmlElement::Paragraph.as_str());
    element.write_inner_content(|writer| {
        for run in &paragraph.runs {
            write_run(writer, run)?;
        }
        Ok(())
    })?;
    Ok(())
}

fn write_run(writer: &mut XmlWriter, run: &Run) -> XmlResult {
    let element = writer.create_element(XmlElement::Run.as_str());
    element.write_inner_content(|writer| {
        if run.properties.has_formatting() {
            write_run_properties(writer, &run.properties)?;
        }

        if run.space_preserve {
            let element = writer.create_element(XmlElement::Text.as_str());
            element
                .with_attribute((XmlAttr::Space.as_str(), XmlAttrValue::Preserve.as_str()))
                .write_text_content(BytesText::new(&run.text))?;
        } else {
            let element = writer.create_element(XmlElement::Text.as_str());
            element.write_text_content(BytesText::new(&run.text))?;
        }

        Ok(())
    })?;
    Ok(())
}

fn write_run_properties(writer: &mut XmlWriter, properties: &RunProperties) -> XmlResult {
    let element = writer.create_element(XmlElement::RunProps.as_str());
    element.write_inner_content(|writer| {
        for (condition, element) in [
            (properties.bold, XmlElement::Bold),
            (properties.italic, XmlElement::Italic),
            (properties.strike, XmlElement::Strike),
            (properties.dstrike, XmlElement::DStrike),
        ] {
            if condition {
                writer.create_element(element.as_str()).write_empty()?;
            }
        }

        if let Some(underline) = &properties.underline {
            write_attribute_element(
                writer,
                &XmlElement::Underline,
                &XmlAttr::Val,
                &XmlAttrValue::Custom(&underline.value()),
            )?;
        }

        if let Some(color) = &properties.color {
            write_attribute_element(
                writer,
                &XmlElement::Color,
                &XmlAttr::Val,
                &XmlAttrValue::Custom(&color.value()),
            )?;
        }

        if let Some(size) = &properties.size {
            let size_str = size.to_string();
            write_attribute_element(
                writer,
                &XmlElement::Size,
                &XmlAttr::Val,
                &XmlAttrValue::Custom(&size_str),
            )?;
        }

        if let Some(font_set) = &properties.font {
            match font_set.get_hint_value() {
                FontType::Default => (),
                _ => {
                    if let Ok(val) = font_set.get_hint() {
                        let attr = format!("w:{}", font_set.get_hint_value());
                        let mut font_element = writer.create_element(XmlElement::Fonts.as_str());
                        font_element = font_element.with_attribute((attr.as_str(), val.as_str()));
                        font_element.write_empty()?;
                    }
                }
            }
        }

        if let Some(highlight) = &properties.highlight {
            write_attribute_element(
                writer,
                &XmlElement::Highlight,
                &XmlAttr::Val,
                &XmlAttrValue::Custom(&highlight.value()),
            )?;
        }

        if let Some(valign) = &properties.valign {
            write_attribute_element(
                writer,
                &XmlElement::VertAlign,
                &XmlAttr::Val,
                &XmlAttrValue::Custom(&valign.value()),
            )?;
        }

        if let Some(spacing) = &properties.spacing {
            let spacing_str = spacing.to_string();
            write_attribute_element(
                writer,
                &XmlElement::Spacing,
                &XmlAttr::Val,
                &XmlAttrValue::Custom(&spacing_str),
            )?;
        }

        Ok(())
    })?;
    Ok(())
}

fn write_attribute_element(
    writer: &mut XmlWriter,
    element: &XmlElement,
    attr_name: &XmlAttr,
    attr_value: &XmlAttrValue,
) -> XmlResult {
    let element = writer.create_element(element.as_str());
    element
        .with_attribute((attr_name.as_str(), attr_value.as_str()))
        .write_empty()?;
    Ok(())
}
