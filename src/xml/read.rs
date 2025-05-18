use crate::elements::*;
use crate::errors::RudocxError;
use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::Reader;

/// Struct to contain the current status of
struct CurrentData {
    document: Document,
    paragraph: Option<Paragraph>,
    run: Option<Run>,
    run_properties: Option<RunProperties>,
    in_run_properties: bool,
}
impl CurrentData {
    fn new() -> Self {
        Self {
            document: Document::default(),
            paragraph: None,
            run: None,
            run_properties: None,
            in_run_properties: false,
        }
    }
}

///Generate a Document struct from parsing the contents of an OOXML
pub fn parse(contents: &str) -> Result<Document, RudocxError> {
    parse_ooxml(contents)
}

fn parse_ooxml(content: &str) -> Result<Document, RudocxError> {
    let mut reader = Reader::from_str(content);
    let mut buf = Vec::new();
    let mut current_data = CurrentData::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            //Tag opening. With or without attributes
            Event::Start(e) => handle_open_tag(
                e.name().as_ref(),
                &mut current_data,
                &mut e.attributes(),
                &reader,
            )?,
            //Self-closing tag. With or without attributes
            Event::Empty(e) => handle_empty_tag(
                e.name().as_ref(),
                &mut current_data,
                &mut e.attributes(),
                &reader,
            )?,
            //Plain text contained between two tags
            Event::Text(e) => handle_text(&mut current_data, e.unescape()?.to_string())?,
            //Tag closing. Without attributes
            Event::End(e) => handle_close_tag(e.name().as_ref(), &mut current_data)?,
            //End of file
            Event::Eof => {
                handle_eof(&mut current_data)?;
                break;
            }
            _ => (),
        }
        buf.clear();
    }
    Ok(current_data.document)
}

fn handle_text(data: &mut CurrentData, text: String) -> Result<(), RudocxError> {
    if let Some(ref mut r) = data.run {
        r.text.push_str(&text);
    }
    Ok(())
}

fn handle_open_tag(
    tag: &[u8],
    data: &mut CurrentData,
    _attr: &mut Attributes,
    _reader: &Reader<&[u8]>,
) -> Result<(), RudocxError> {
    match tag {
        //Plain text
        b"w:t" => Ok(()),
        //RunProperties
        b"w:rPr" => {
            data.in_run_properties = true;
            Ok(())
        }
        //Paragraph
        b"w:p" => {
            //If current contains a paragraph, take it from the option
            if let Some(p) = data.paragraph.take() {
                data.document.paragraphs.push(p);
            }
            //Put a default paragraph in the empty option
            data.paragraph = Some(Paragraph::default());
            Ok(())
        }

        //Run
        b"w:r" => {
            if let Some(r) = data.run.take() {
                if let Some(ref mut p) = data.paragraph {
                    p.runs.push(r);
                }
            }
            data.run_properties = Some(RunProperties::default());
            data.run = Some(Run::default());
            Ok(())
        }
        _ => Ok(()),
    }
}

fn handle_empty_tag(
    tag: &[u8],
    data: &mut CurrentData,
    attr: &mut Attributes,
    reader: &Reader<&[u8]>,
) -> Result<(), RudocxError> {
    match tag {
        //TODO: Error for !data.in_run_properties
        //Bold
        b"w:b" => {
            if data.in_run_properties {
                if let Some(ref mut p) = data.run_properties {
                    p.bold = true;
                }
            }
            Ok(())
        }
        //Italic
        b"w:i" => {
            if data.in_run_properties {
                if let Some(ref mut p) = data.run_properties {
                    p.italic = true;
                }
            }
            Ok(())
        }
        //Underline
        b"w:u" => {
            if data.in_run_properties {
                if let Some(ref mut p) = data.run_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            p.underline = Some(Underline::new(UnderlineStyle::from(v.as_ref())));
                        }
                    }
                }
            }
            Ok(())
        }
        //Font color
        b"w:color" => {
            if data.in_run_properties {
                if let Some(ref mut p) = data.run_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            p.color = Some(HexColor::new(v.as_ref()));
                        }
                    }
                }
            }
            Ok(())
        }
        //Font size
        b"w:sz" => {
            if data.in_run_properties {
                if let Some(ref mut p) = data.run_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            p.size = Some(v.parse::<u32>()?);
                        }
                    }
                }
            }
            Ok(())
        }
        //Fonts (get ready this is a big one)
        b"w:rFonts" => {
            if data.in_run_properties {
                if let Some(ref mut p) = data.run_properties {
                    for r in attr {
                        if let Ok(a) = r {
                            match a.key.as_ref() {
                                b"w:hint" => {
                                    if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                                        if p.font.is_some() {
                                            let mut fonts = p.clone().font.unwrap();
                                            fonts.hint = FontType::from(v.as_ref());
                                            p.font = Some(fonts);
                                        } else {
                                            let mut fonts = FontSet::default();
                                            fonts.hint = FontType::from(v.as_ref());
                                            p.font = Some(fonts);
                                        }
                                    }
                                }
                                b"w:ascii" => {
                                    if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                                        if p.font.is_some() {
                                            let mut fonts = p.clone().font.unwrap();
                                            fonts.ascii = Some(v.to_string());
                                            p.font = Some(fonts);
                                        } else {
                                            let mut fonts = FontSet::default();
                                            fonts.ascii = Some(v.to_string());
                                            p.font = Some(fonts);
                                        }
                                    }
                                }
                                b"w:hiAnsi" => {
                                    if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                                        if p.font.is_some() {
                                            let mut fonts = p.clone().font.unwrap();
                                            fonts.hi_ansi = Some(v.to_string());
                                            p.font = Some(fonts);
                                        } else {
                                            let mut fonts = FontSet::default();
                                            fonts.hi_ansi = Some(v.to_string());
                                            p.font = Some(fonts);
                                        }
                                    }
                                }
                                b"w:eastAsia" => {
                                    if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                                        if p.font.is_some() {
                                            let mut fonts = p.clone().font.unwrap();
                                            fonts.east_asia = Some(v.to_string());
                                            p.font = Some(fonts);
                                        } else {
                                            let mut fonts = FontSet::default();
                                            fonts.east_asia = Some(v.to_string());
                                            p.font = Some(fonts);
                                        }
                                    }
                                }
                                b"w:cs" => {
                                    if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                                        if p.font.is_some() {
                                            let mut fonts = p.clone().font.unwrap();
                                            fonts.cs = Some(v.to_string());
                                            p.font = Some(fonts);
                                        } else {
                                            let mut fonts = FontSet::default();
                                            fonts.cs = Some(v.to_string());
                                            p.font = Some(fonts);
                                        }
                                    }
                                }
                                b"w:asciiTheme" => {
                                    if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                                        if p.font.is_some() {
                                            let mut fonts = p.clone().font.unwrap();
                                            fonts.ascii_theme = Some(v.to_string());
                                            p.font = Some(fonts);
                                        } else {
                                            let mut fonts = FontSet::default();
                                            fonts.ascii_theme = Some(v.to_string());
                                            p.font = Some(fonts);
                                        }
                                    }
                                }
                                b"w:hiAnsiTheme" => {
                                    if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                                        if p.font.is_some() {
                                            let mut fonts = p.clone().font.unwrap();
                                            fonts.hi_ansi_theme = Some(v.to_string());
                                            p.font = Some(fonts);
                                        } else {
                                            let mut fonts = FontSet::default();
                                            fonts.hi_ansi_theme = Some(v.to_string());
                                            p.font = Some(fonts);
                                        }
                                    }
                                }
                                b"w:eastAsiaTheme" => {
                                    if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                                        if p.font.is_some() {
                                            let mut fonts = p.clone().font.unwrap();
                                            fonts.east_asia_theme = Some(v.to_string());
                                            p.font = Some(fonts);
                                        } else {
                                            let mut fonts = FontSet::default();
                                            fonts.east_asia_theme = Some(v.to_string());
                                            p.font = Some(fonts);
                                        }
                                    }
                                }
                                b"w:csTheme" => {
                                    if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                                        if p.font.is_some() {
                                            let mut fonts = p.clone().font.unwrap();
                                            fonts.cs_theme = Some(v.to_string());
                                            p.font = Some(fonts);
                                        } else {
                                            let mut fonts = FontSet::default();
                                            fonts.cs_theme = Some(v.to_string());
                                            p.font = Some(fonts);
                                        }
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
            Ok(())
        }
        //Highlighting
        b"w:highlight" => {
            if data.in_run_properties {
                if let Some(ref mut p) = data.run_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            p.highlight = Some(HLColor::new(HighlightPalette::from(v.as_ref())));
                        }
                    }
                }
            }
            Ok(())
        }
        //Striked text
        b"w:strike" => {
            if data.in_run_properties {
                if let Some(ref mut p) = data.run_properties {
                    p.strike = true;
                }
            }
            Ok(())
        }
        //Double striked text
        b"w:dstrike" => {
            if data.in_run_properties {
                if let Some(ref mut p) = data.run_properties {
                    p.dstrike = true;
                }
            }
            Ok(())
        }
        //Vertical alignment
        b"w:valign" => {
            if data.in_run_properties {
                if let Some(ref mut p) = data.run_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            p.valign = Some(VerticalAlign::new(AlignValues::from(v.as_ref())));
                        }
                    }
                }
            }
            Ok(())
        }
        //Spacing
        b"w:spacing" => {
            if data.in_run_properties {
                if let Some(ref mut p) = data.run_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            p.spacing = Some(v.parse::<u32>()?);
                        }
                    }
                }
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn handle_close_tag(tag: &[u8], data: &mut CurrentData) -> Result<(), RudocxError> {
    match tag {
        //Text
        b"w:t" => Ok(()),
        //Run Properties
        b"w:rPr" => {
            data.in_run_properties = false;
            Ok(())
        }
        //Paragraph
        b"w:p" => {
            if let Some(p) = data.paragraph.take() {
                if let Some(r) = data.run.take() {
                    if let Some(mut p) = Some(p) {
                        p.runs.push(r);
                        data.document.paragraphs.push(p);
                    }
                } else {
                    data.document.paragraphs.push(p);
                }
            }
            data.paragraph = None;
            Ok(())
        }
        //Run
        b"w:r" => {
            if let Some(mut r) = data.run.take() {
                if let Some(p) = data.run_properties.take() {
                    r.properties = p;
                }
                if let Some(ref mut p) = data.paragraph {
                    p.runs.push(r);
                }
            }
            data.run = None;
            Ok(())
        }
        _ => Ok(()),
    }
}

fn handle_eof(data: &mut CurrentData) -> Result<(), RudocxError> {
    if let Some(p) = data.paragraph.take() {
        if let Some(r) = data.run.take() {
            if let Some(mut p) = Some(p) {
                p.runs.push(r);
                data.document.paragraphs.push(p);
            }
        } else {
            data.document.paragraphs.push(p);
        }
    }
    Ok(())
}

///This function server as a boilerplate parser and thus it is not completed.
///It will not work with the majority of the elements that intervene in OOXML.
#[deprecated]
pub fn parse_document_xml(xml_content: &str) -> Result<Document, RudocxError> {
    let mut reader = Reader::from_str(xml_content);
    let mut buf = Vec::new();
    let mut document = Document::default();
    let mut current_paragraph: Option<Paragraph> = None;
    let mut current_run: Option<Run> = None;
    let mut current_run_properties: Option<RunProperties> = None;
    let mut is_in_run_properties = false;

    loop {
        //Loop through all the events from an XML string
        match reader.read_event_into(&mut buf) {
            //If it's a tag opening. With or without attributes.
            Ok(Event::Start(ref e)) => match e.name().as_ref() {
                //Paragraphs
                b"w:p" => {
                    if let Some(p) = current_paragraph.take() {
                        document.paragraphs.push(p);
                    }
                    current_paragraph = Some(Paragraph::default());
                }
                //Runs
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
                        space_preserve: false,
                    });
                }
                //RunProperties
                b"w:rPr" => {
                    is_in_run_properties = true;
                }
                //Text
                b"w:t" => {}
                //Skip
                _ => (),
            },
            //If it's a self closed tag. With or without attributes
            Ok(Event::Empty(ref e)) => match e.name().as_ref() {
                //Bold
                b"w:b" => {
                    if is_in_run_properties {
                        if let Some(ref mut props) = current_run_properties {
                            props.bold = true;
                        }
                    }
                }
                //Italics
                b"w:i" => {
                    if is_in_run_properties {
                        if let Some(ref mut props) = current_run_properties {
                            props.italic = true;
                        }
                    }
                }
                //Color
                b"w:color" => {
                    if is_in_run_properties {
                        if let Some(ref mut props) = current_run_properties {
                            for attr_result in e.attributes() {
                                if let Ok(attr) = attr_result {
                                    if attr.key.as_ref() == b"w:val" {
                                        if let Ok(val) =
                                            attr.decode_and_unescape_value(reader.decoder())
                                        {
                                            props.color = Some(HexColor::new(val.as_ref()));
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                //Skip
                _ => (),
            },
            //Plain text contained between two tags
            Ok(Event::Text(e)) => {
                if let Some(ref mut run) = current_run {
                    run.text.push_str(&e.unescape()?.to_string());
                }
            }
            //End of a tag. Without attributes
            Ok(Event::End(ref e)) => match e.name().as_ref() {
                //Paragraph
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
                //Run
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
                //RunProperties
                b"w:rPr" => {
                    is_in_run_properties = false;
                }
                //Skip
                _ => (),
            },
            //Detect End of File, push and set remaining dangling data and break the loop
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

        let result = parse(xml_input);
        assert!(result.is_ok());
        let doc = result.unwrap();
        println!("{:?}", doc);

        assert_eq!(doc.paragraphs.len(), 3);

        // Paragraph 1: Plain text
        assert_eq!(doc.paragraphs[0].runs.len(), 1);
        assert_eq!(doc.paragraphs[0].runs[0].text, "This is plain text.");
        assert!(!doc.paragraphs[0].runs[0].properties.bold);
        assert!(!doc.paragraphs[0].runs[0].properties.italic);

        // Paragraph 2: Bold, space, Italic
        assert_eq!(doc.paragraphs[1].runs.len(), 3);
        // Run 1: Bold
        assert_eq!(doc.paragraphs[1].runs[0].text, "This is bold.");
        assert!(doc.paragraphs[1].runs[0].properties.bold);
        assert!(!doc.paragraphs[1].runs[0].properties.italic);
        // Run 2: Space (should be preserved)
        assert_eq!(doc.paragraphs[1].runs[1].text, " ");
        assert!(!doc.paragraphs[1].runs[1].properties.bold);
        assert!(!doc.paragraphs[1].runs[1].properties.italic);
        // Run 3: Italic
        assert_eq!(doc.paragraphs[1].runs[2].text, "This is italic.");
        assert!(!doc.paragraphs[1].runs[2].properties.bold);
        assert!(doc.paragraphs[1].runs[2].properties.italic);

        // Paragraph 3: Bold and Italic
        assert_eq!(doc.paragraphs[2].runs.len(), 1);
        assert_eq!(doc.paragraphs[2].runs[0].text, "Bold and Italic.");
        assert!(doc.paragraphs[2].runs[0].properties.bold);
        assert!(doc.paragraphs[2].runs[0].properties.italic);
    }
}
