use crate::elements::{
    Document, FontType, Hyperlink, Paragraph, ParagraphChild, ParagraphProperties, Run,
    RunProperties,
};
use crate::errors::RudocxError;
use crate::xml::XmlWElement;

use quick_xml::events::BytesText;
use quick_xml::{ElementWriter, Writer};
use std::io::Cursor;

type XmlWriter = Writer<Cursor<Vec<u8>>>;
type XmlResult = std::io::Result<()>;

enum XmlNs {
    W,
    R,
}

impl XmlNs {
    fn as_str(&self) -> &'static str {
        match self {
            XmlNs::W => "xmlns:w",
            XmlNs::R => "xmlns:r",
        }
    }

    fn url(&self) -> &'static str {
        match self {
            XmlNs::W => "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
            XmlNs::R => "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
        }
    }
}

/// All attributes are preceded by their semicolon identifyer but those that use ``:w`
enum XmlAttr {
    After,
    AfterAutospacing,
    Before,
    BeforeAutospacing,
    Line,
    LineRule,
    Color,
    Fill,
    Space,
    Sz,
    Pos,
    Val,
    Leader,
    Rid,
    XmlSpace,
    Left,
    Right,
    Hanging,
    FirstLine,
    LeftChars,
    RightChars,
    HangingChars,
    FirstLineChars,
}

impl XmlAttr {
    fn as_str(&self) -> &'static str {
        match self {
            XmlAttr::After => "w:after",
            XmlAttr::AfterAutospacing => "w:afterAutospacing",
            XmlAttr::Before => "w:before",
            XmlAttr::BeforeAutospacing => "w:beforeAutospacing",
            XmlAttr::Line => "w:line",
            XmlAttr::LineRule => "w:lineRule",
            XmlAttr::Val => "w:val",
            XmlAttr::Rid => "r:id",
            XmlAttr::XmlSpace => "xml:space",
            XmlAttr::Color => "w:color",
            XmlAttr::Fill => "w:fill",
            XmlAttr::Leader => "w:leader",
            XmlAttr::Pos => "w:pos",
            XmlAttr::Sz => "w:sz",
            XmlAttr::Space => "w:space",
            XmlAttr::Left => "w:left",
            XmlAttr::Right => "w:right",
            XmlAttr::Hanging => "w:hanging",
            XmlAttr::FirstLine => "w:firstLine",
            XmlAttr::LeftChars => "w:leftChars",
            XmlAttr::RightChars => "w:rightChars",
            XmlAttr::HangingChars => "w:hangingChars",
            XmlAttr::FirstLineChars => "w:firstLineChars",
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

    let element = writer.create_element(XmlWElement::Document.as_str());
    element
        .with_attribute((XmlNs::W.as_str(), XmlNs::W.url()))
        .with_attribute((XmlNs::R.as_str(), XmlNs::R.url()))
        .write_inner_content(|writer| write_body(writer, document))
        .map_err(|e| RudocxError::XmlError(e.into()))?;

    let xml_bytes = writer.into_inner().into_inner();
    String::from_utf8(xml_bytes).map_err(RudocxError::Utf8Error)
}

fn write_body(writer: &mut XmlWriter, document: &Document) -> XmlResult {
    let element = writer.create_element(XmlWElement::Body.as_str());
    element.write_inner_content(|writer| {
        for paragraph in &document.paragraphs {
            write_paragraph(writer, paragraph)?;
        }
        Ok(())
    })?;
    Ok(())
}

fn write_paragraph(writer: &mut XmlWriter, paragraph: &Paragraph) -> XmlResult {
    let element = writer.create_element(XmlWElement::Paragraph.as_str());
    element.write_inner_content(|writer| {
        if paragraph.properties.has_formatting() {
            write_paragraph_properties(writer, &paragraph.properties)?;
        }
        for child in &paragraph.children {
            match child {
                ParagraphChild::Run(run) => write_run(writer, run)?,
                ParagraphChild::Hyperlink(hyperlink) => write_hyperlink(writer, hyperlink)?,
            }
        }
        Ok(())
    })?;
    Ok(())
}

fn write_paragraph_properties(
    writer: &mut XmlWriter,
    properties: &ParagraphProperties,
) -> XmlResult {
    let element = writer.create_element(XmlWElement::ParagraphProps.as_str());
    element.write_inner_content(|writer| {
        for (condition, element) in [
            (properties.keep_next, XmlWElement::KeepNext),
            (properties.keep_lines, XmlWElement::KeepLines),
            (properties.page_break_before, XmlWElement::PageBreakBefore),
            (properties.window_control, XmlWElement::WindowControl),
            (
                properties.suppress_line_numbers,
                XmlWElement::SuppressLineNumbers,
            ),
            (
                properties.suppress_auto_hyphens,
                XmlWElement::SuppressAutoHyphens,
            ),
            (properties.word_wrap, XmlWElement::WordWrap),
            (properties.topline_punct, XmlWElement::ToplinePunct),
            (properties.autospace_de, XmlWElement::AutospaceDe),
            (properties.autospace_dn, XmlWElement::AutospaceDn),
            (properties.bidi, XmlWElement::Bidi),
            (properties.snap_to_grid, XmlWElement::SnapToGrid),
            (
                properties.contextual_spacing,
                XmlWElement::ContextualSpacing,
            ),
            (properties.mirror_indents, XmlWElement::MirrorIndents),
            (properties.suppress_overlap, XmlWElement::SuppressOverlap),
        ] {
            if condition {
                writer.create_element(element.as_str()).write_empty()?;
            }
        }
        //TODO: Write non boolean properties
        // Borders
        if let Some(parahraph_border) = &properties.paragraph_borders {
            if let Some(side) = &parahraph_border.top {
                let mut multi = MultiattributeElementFactory::new(writer, &XmlWElement::BorderTop);
                multi = multi
                    .add_attribute(&XmlAttr::Val, &XmlAttrValue::Custom(&side.val.to_string()));
                if let Some(opt) = &side.color {
                    multi =
                        multi.add_attribute(&XmlAttr::Color, &XmlAttrValue::Custom(&opt.value()));
                }
                if let Some(opt) = &side.sz {
                    multi =
                        multi.add_attribute(&XmlAttr::Sz, &XmlAttrValue::Custom(&opt.to_string()));
                }
                if let Some(opt) = &side.space {
                    multi = multi
                        .add_attribute(&XmlAttr::Space, &XmlAttrValue::Custom(&opt.to_string()));
                }
                multi.build()?;
            }
            if let Some(side) = &parahraph_border.bottom {
                let mut multi =
                    MultiattributeElementFactory::new(writer, &XmlWElement::BorderBottom);
                multi = multi
                    .add_attribute(&XmlAttr::Val, &XmlAttrValue::Custom(&side.val.to_string()));
                if let Some(opt) = &side.color {
                    multi =
                        multi.add_attribute(&XmlAttr::Color, &XmlAttrValue::Custom(&opt.value()));
                }
                if let Some(opt) = &side.sz {
                    multi =
                        multi.add_attribute(&XmlAttr::Sz, &XmlAttrValue::Custom(&opt.to_string()));
                }
                if let Some(opt) = &side.space {
                    multi = multi
                        .add_attribute(&XmlAttr::Space, &XmlAttrValue::Custom(&opt.to_string()));
                }
                multi.build()?;
            }
            if let Some(side) = &parahraph_border.left {
                let mut multi = MultiattributeElementFactory::new(writer, &XmlWElement::BorderLeft);
                multi = multi
                    .add_attribute(&XmlAttr::Val, &XmlAttrValue::Custom(&side.val.to_string()));
                if let Some(opt) = &side.color {
                    multi =
                        multi.add_attribute(&XmlAttr::Color, &XmlAttrValue::Custom(&opt.value()));
                }
                if let Some(opt) = &side.sz {
                    multi =
                        multi.add_attribute(&XmlAttr::Sz, &XmlAttrValue::Custom(&opt.to_string()));
                }
                if let Some(opt) = &side.space {
                    multi = multi
                        .add_attribute(&XmlAttr::Space, &XmlAttrValue::Custom(&opt.to_string()));
                }
                multi.build()?;
            }
            if let Some(side) = &parahraph_border.right {
                let mut multi =
                    MultiattributeElementFactory::new(writer, &XmlWElement::BorderRight);
                multi = multi
                    .add_attribute(&XmlAttr::Val, &XmlAttrValue::Custom(&side.val.to_string()));
                if let Some(opt) = &side.color {
                    multi =
                        multi.add_attribute(&XmlAttr::Color, &XmlAttrValue::Custom(&opt.value()));
                }
                if let Some(opt) = &side.sz {
                    multi =
                        multi.add_attribute(&XmlAttr::Sz, &XmlAttrValue::Custom(&opt.to_string()));
                }
                if let Some(opt) = &side.space {
                    multi = multi
                        .add_attribute(&XmlAttr::Space, &XmlAttrValue::Custom(&opt.to_string()));
                }
                multi.build()?;
            }
            if let Some(side) = &parahraph_border.between {
                let mut multi =
                    MultiattributeElementFactory::new(writer, &XmlWElement::BorderBetween);
                multi = multi
                    .add_attribute(&XmlAttr::Val, &XmlAttrValue::Custom(&side.val.to_string()));
                if let Some(opt) = &side.color {
                    multi =
                        multi.add_attribute(&XmlAttr::Color, &XmlAttrValue::Custom(&opt.value()));
                }
                if let Some(opt) = &side.sz {
                    multi =
                        multi.add_attribute(&XmlAttr::Sz, &XmlAttrValue::Custom(&opt.to_string()));
                }
                if let Some(opt) = &side.space {
                    multi = multi
                        .add_attribute(&XmlAttr::Space, &XmlAttrValue::Custom(&opt.to_string()));
                }
                multi.build()?;
            }
        }

        // Shading
        if let Some(shading) = &properties.shading {
            let mut multi = MultiattributeElementFactory::new(writer, &XmlWElement::Shading);
            multi = multi.add_attribute(&XmlAttr::Val, &&XmlAttrValue::Custom(&shading.value()));
            if let Some(opt) = &shading.color {
                multi = multi.add_attribute(&XmlAttr::Color, &&XmlAttrValue::Custom(&opt.value()));
            }
            if let Some(opt) = &shading.fill {
                multi = multi.add_attribute(&XmlAttr::Fill, &&XmlAttrValue::Custom(&opt.value()));
            }
            multi.build()?;
        }

        // Tabs
        if let Some(tabs) = &properties.tabs {
            let parent = writer.create_element(XmlWElement::Tabs.as_str());
            parent.write_inner_content(|writer| {
                for tab in tabs {
                    let mut multi = MultiattributeElementFactory::new(writer, &XmlWElement::Tab)
                        .add_attribute(&XmlAttr::Val, &XmlAttrValue::Custom(&tab.value()))
                        .add_attribute(&XmlAttr::Pos, &XmlAttrValue::Custom(&tab.pos.to_string()));
                    if let Some(leader) = &tab.leader {
                        multi = multi.add_attribute(
                            &XmlAttr::Leader,
                            &XmlAttrValue::Custom(&leader.to_string()),
                        );
                    }
                    multi.build()?;
                }
                Ok(())
            })?;
        }

        // Numbering
        if let Some(numbering) = &properties.numbering_properties {
            let parent = writer.create_element(XmlWElement::NumberingProperties.as_str());
            parent.write_inner_content(|writer| {
                write_attribute_element(
                    writer,
                    &XmlWElement::Ilvl,
                    &XmlAttr::Val,
                    &XmlAttrValue::Custom(&numbering.ilvl.to_string()),
                )?;
                write_attribute_element(
                    writer,
                    &XmlWElement::NumId,
                    &XmlAttr::Val,
                    &XmlAttrValue::Custom(&numbering.num_id.to_string()),
                )?;
                Ok(())
            })?;
        }

        // Spacing
        if let Some(spacing) = &properties.spacing {
            let mut multi = MultiattributeElementFactory::new(writer, &XmlWElement::Spacing);
            if let Some(opt) = &spacing.before {
                multi =
                    multi.add_attribute(&XmlAttr::Before, &&XmlAttrValue::Custom(&opt.to_string()));
            }
            if let Some(opt) = &spacing.after {
                multi =
                    multi.add_attribute(&XmlAttr::After, &&XmlAttrValue::Custom(&opt.to_string()));
            }
            if let Some(opt) = &spacing.before_autospacing {
                multi = multi.add_attribute(
                    &XmlAttr::BeforeAutospacing,
                    &&XmlAttrValue::Custom(&opt.to_string()),
                );
            }
            if let Some(opt) = &spacing.after_autospacing {
                multi = multi.add_attribute(
                    &XmlAttr::AfterAutospacing,
                    &&XmlAttrValue::Custom(&opt.to_string()),
                );
            }
            if let Some(opt) = &spacing.line {
                multi =
                    multi.add_attribute(&XmlAttr::Line, &&XmlAttrValue::Custom(&opt.to_string()));
            }
            if let Some(opt) = &spacing.line_rule {
                multi = multi
                    .add_attribute(&XmlAttr::LineRule, &&XmlAttrValue::Custom(&opt.to_string()));
            }
            multi.build()?;
        }

        // Indentation
        if let Some(indent) = &properties.ind {
            let mut multi = MultiattributeElementFactory::new(writer, &XmlWElement::Ind);
            if let Some(opt) = &indent.first_line {
                multi = multi.add_attribute(
                    &XmlAttr::FirstLine,
                    &&XmlAttrValue::Custom(&opt.to_string()),
                );
            }
            if let Some(opt) = &indent.first_line_chars {
                multi = multi.add_attribute(
                    &XmlAttr::FirstLineChars,
                    &&XmlAttrValue::Custom(&opt.to_string()),
                );
            }
            if let Some(opt) = &indent.right {
                multi =
                    multi.add_attribute(&XmlAttr::Right, &&XmlAttrValue::Custom(&opt.to_string()));
            }
            if let Some(opt) = &indent.right_chars {
                multi = multi.add_attribute(
                    &XmlAttr::RightChars,
                    &&XmlAttrValue::Custom(&opt.to_string()),
                );
            }
            if let Some(opt) = &indent.left {
                multi =
                    multi.add_attribute(&XmlAttr::Left, &&XmlAttrValue::Custom(&opt.to_string()));
            }
            if let Some(opt) = &indent.left_chars {
                multi = multi.add_attribute(
                    &XmlAttr::LeftChars,
                    &&XmlAttrValue::Custom(&opt.to_string()),
                );
            }
            if let Some(opt) = &indent.hanging {
                multi = multi
                    .add_attribute(&XmlAttr::Hanging, &&XmlAttrValue::Custom(&opt.to_string()));
            }
            if let Some(opt) = &indent.hanging_chars {
                multi = multi.add_attribute(
                    &XmlAttr::HangingChars,
                    &&XmlAttrValue::Custom(&opt.to_string()),
                );
            }
            multi.build()?;
        }

        // Justification
        if let Some(justification) = &properties.jc {
            write_attribute_element(
                writer,
                &XmlWElement::Jc,
                &XmlAttr::Val,
                &XmlAttrValue::Custom(&justification.value()),
            )?;
        }

        // Direction
        if let Some(text_dir) = &properties.text_direction {
            write_attribute_element(
                writer,
                &XmlWElement::TextDirection,
                &XmlAttr::Val,
                &XmlAttrValue::Custom(&text_dir.value()),
            )?;
        }

        // Alignment
        if let Some(text_align) = &properties.text_alignment {
            write_attribute_element(
                writer,
                &XmlWElement::TextAlignment,
                &XmlAttr::Val,
                &XmlAttrValue::Custom(&text_align.value()),
            )?;
        }

        // TBox tight wrap
        if let Some(tbtw) = &properties.textbox_tight_wrap {
            write_attribute_element(
                writer,
                &XmlWElement::TextboxTightWrap,
                &XmlAttr::Val,
                &XmlAttrValue::Custom(&tbtw.value()),
            )?;
        }

        // Outline Level
        if let Some(outline_lvl) = &properties.outline_level {
            write_attribute_element(
                writer,
                &XmlWElement::TextboxTightWrap,
                &XmlAttr::Val,
                &XmlAttrValue::Custom(&outline_lvl.to_string()),
            )?;
        }

        // Default Run Properties
        if let Some(run_properties) = &properties.default_run_properties {
            write_run_properties(writer, run_properties)?;
        }

        Ok(())
    })?;
    Ok(())
}

fn write_hyperlink(writer: &mut XmlWriter, hyperlink: &Hyperlink) -> XmlResult {
    let _element = writer
        .create_element(XmlWElement::Hyperlink.as_str())
        .with_attribute((
            XmlAttr::Rid.as_str(),
            XmlAttrValue::Custom(&hyperlink.id).as_str(),
        ))
        .write_inner_content(|writer| {
            for run in &hyperlink.runs {
                write_run(writer, run)?;
            }
            Ok(())
        })?;
    Ok(())
}

fn write_run(writer: &mut XmlWriter, run: &Run) -> XmlResult {
    let element = writer.create_element(XmlWElement::Run.as_str());
    element.write_inner_content(|writer| {
        if run.properties.has_formatting() {
            write_run_properties(writer, &run.properties)?;
        }

        if run.space_preserve {
            let element = writer.create_element(XmlWElement::Text.as_str());
            element
                .with_attribute((XmlAttr::XmlSpace.as_str(), XmlAttrValue::Preserve.as_str()))
                .write_text_content(BytesText::new(&run.text))?;
        } else {
            let element = writer.create_element(XmlWElement::Text.as_str());
            element.write_text_content(BytesText::new(&run.text))?;
        }

        Ok(())
    })?;
    Ok(())
}

fn write_run_properties(writer: &mut XmlWriter, properties: &RunProperties) -> XmlResult {
    let element = writer.create_element(XmlWElement::RunProps.as_str());
    element.write_inner_content(|writer| {
        for (condition, element) in [
            (properties.bold, XmlWElement::Bold),
            (properties.italic, XmlWElement::Italic),
            (properties.strike, XmlWElement::Strike),
            (properties.dstrike, XmlWElement::DStrike),
        ] {
            if condition {
                writer.create_element(element.as_str()).write_empty()?;
            }
        }

        if let Some(underline) = &properties.underline {
            write_attribute_element(
                writer,
                &XmlWElement::Underline,
                &XmlAttr::Val,
                &XmlAttrValue::Custom(&underline.value()),
            )?;
        }

        if let Some(color) = &properties.color {
            write_attribute_element(
                writer,
                &XmlWElement::Color,
                &XmlAttr::Val,
                &XmlAttrValue::Custom(&color.value()),
            )?;
        }

        if let Some(size) = &properties.size {
            let size_str = size.to_string();
            write_attribute_element(
                writer,
                &XmlWElement::Size,
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
                        let mut font_element = writer.create_element(XmlWElement::Fonts.as_str());
                        font_element = font_element.with_attribute((attr.as_str(), val.as_str()));
                        font_element.write_empty()?;
                    }
                }
            }
        }

        if let Some(highlight) = &properties.highlight {
            write_attribute_element(
                writer,
                &XmlWElement::Highlight,
                &XmlAttr::Val,
                &XmlAttrValue::Custom(&highlight.value()),
            )?;
        }

        if let Some(valign) = &properties.valign {
            write_attribute_element(
                writer,
                &XmlWElement::VertAlign,
                &XmlAttr::Val,
                &XmlAttrValue::Custom(&valign.value()),
            )?;
        }

        if let Some(spacing) = &properties.spacing {
            let spacing_str = spacing.to_string();
            write_attribute_element(
                writer,
                &XmlWElement::Spacing,
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
    element: &XmlWElement,
    attr_name: &XmlAttr,
    attr_value: &XmlAttrValue,
) -> XmlResult {
    let element = writer.create_element(element.as_str());
    element
        .with_attribute((attr_name.as_str(), attr_value.as_str()))
        .write_empty()?;
    Ok(())
}

struct MultiattributeElementFactory<'a> {
    inner: ElementWriter<'a, Cursor<Vec<u8>>>,
}

impl<'a> MultiattributeElementFactory<'a> {
    pub fn new(writer: &'a mut XmlWriter, element: &XmlWElement) -> Self {
        Self {
            inner: writer.create_element(element.as_str()),
        }
    }

    pub fn add_attribute(mut self, attr_name: &XmlAttr, attr_value: &XmlAttrValue) -> Self {
        self.inner = self
            .inner
            .with_attribute((attr_name.as_str(), attr_value.as_str()));
        self
    }

    pub fn build(self) -> XmlResult {
        self.inner.write_empty()?;
        Ok(())
    }
}
