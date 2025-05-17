use crate::elements::{Document, HexColor, Paragraph, Run, RunProperties};
use crate::errors::RudocxError;
use quick_xml::events::Event;
use quick_xml::Reader;

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
            //If a tag gets open
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
            //If the tag is empty, which means certain elements such as style markers
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
            //If tag contains plain text
            Ok(Event::Text(e)) => {
                if let Some(ref mut run) = current_run {
                    run.text.push_str(&e.unescape()?.to_string());
                }
            }
            //If a tag ends
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

        let result = parse_document_xml(xml_input);
        assert!(result.is_ok());
        let doc = result.unwrap();

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
