use crate::elements::*;
use crate::errors::RudocxError;
use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::events::attributes::Attributes;

/// Struct to contain the current status of
struct CurrentData {
    document: Document,
    paragraph: Option<Paragraph>,
    paragraph_properties: Option<ParagraphProperties>,
    in_paragraph_properties: bool,
    hyperlink: Option<Hyperlink>,
    run: Option<Run>,
    run_properties: Option<RunProperties>,
    in_run_properties: bool,
}
impl CurrentData {
    fn new() -> Self {
        Self {
            document: Document::default(),
            paragraph: None,
            paragraph_properties: None,
            in_paragraph_properties: false,
            hyperlink: None,
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

/// Writes plain text not contained within explicit tags (i.e. plain text contained within a pair of
/// open/close tags)
fn handle_text(data: &mut CurrentData, text: String) -> Result<(), RudocxError> {
    if let Some(ref mut r) = data.run {
        r.text.push_str(&text);
    }
    Ok(())
}

/// Handles opening tags, and generates the children structures they may hold within in the
/// temporary structure `CurrentData` to parse and create the necessary structures for the
/// children the opened tag may contain. It will not perform a closure of the tag.
fn handle_open_tag(
    tag: &[u8],
    data: &mut CurrentData,
    attr: &mut Attributes,
    reader: &Reader<&[u8]>,
) -> Result<(), RudocxError> {
    match tag {
        //Plain text
        b"w:t" => Ok(()),
        //RunProperties
        b"w:rPr" => {
            if data.in_paragraph_properties {
                if let Some(ref mut pp) = data.paragraph_properties {
                    pp.default_run_properties = Some(RunProperties::default());
                }
                return Ok(());
            }
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
            data.paragraph_properties = Some(ParagraphProperties::default());
            data.paragraph = Some(Paragraph::default());
            Ok(())
        }
        //ParagraphProperties
        b"w:pPr" => {
            data.in_paragraph_properties = true;
            Ok(())
        }
        //Hyperlink
        b"w:hyperlink" => {
            //Since hyperlinks are at the same level in the hierarchy as runs, if we
            //encounter a run, we push it and take it out of current to start a hyperlink
            //Hyperlinks cannot be inside hyperlinks.
            if let Some(r) = data.run.take() {
                if let Some(ref mut p) = data.paragraph {
                    p.children.push(ParagraphChild::Run(r));
                }
            }
            let mut link = Hyperlink::default();
            if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"r:id") {
                if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                    link.id = String::from(v.as_ref())
                }
            }
            data.hyperlink = Some(link);
            Ok(())
        }
        //Run
        b"w:r" => {
            //Check if we have to push to hyperlink or to paragraph
            if let Some(ref mut h) = data.hyperlink {
                if let Some(r) = data.run.take() {
                    h.runs.push(r);
                }
            } else {
                if let Some(r) = data.run.take() {
                    if let Some(ref mut p) = data.paragraph {
                        p.children.push(ParagraphChild::Run(r));
                    }
                }
            }
            data.run_properties = Some(RunProperties::default());
            data.run = Some(Run::default());
            Ok(())
        }
        // pBdr (pPr)
        b"w:pBdr" => {
            if let Some(ref mut pp) = data.paragraph_properties {
                pp.paragraph_borders = Some(ParagraphBorder::default());
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

/// Handle all the tags which information is self-contained (i.e. the tag is a `<_:val _:attr="" />`)
/// and have no children within it.
fn handle_empty_tag(
    tag: &[u8],
    data: &mut CurrentData,
    attr: &mut Attributes,
    reader: &Reader<&[u8]>,
) -> Result<(), RudocxError> {
    match tag {
        //TODO: Error when !data.in_run_properties and similars

        /* START OF RUN PROPERTIES */
        //Bold
        b"w:b" => {
            if data.in_run_properties {
                if let Some(ref mut p) = data.run_properties {
                    p.bold = true;
                }
            }
            //TODO: Check logic for setting default run properties inside pPr
            if data.in_paragraph_properties {
                if let Some(ref mut pp) = data.paragraph_properties {
                    if let Some(ref mut p) = pp.default_run_properties {
                        p.bold = true;
                    }
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
        /* END OF RUN PROPERTIES */
        /* START OF PARAGRAPH PROPERTIES */
        //Keep next
        b"w:keepNext" => {
            if data.in_paragraph_properties {
                if let Some(ref mut p) = data.paragraph_properties {
                    //TODO: !! Add this check to all boolean value based tags
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            match v.as_ref() {
                                "off" | "0" | "false" => (),
                                _ => p.keep_next = true,
                            }
                        }
                    } else {
                        p.keep_next = true;
                    }
                }
            }
            Ok(())
        }
        //Keep lines
        b"w:keepLines" => {
            if data.in_paragraph_properties {
                if let Some(ref mut p) = data.paragraph_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            match v.as_ref() {
                                "off" | "0" | "false" => (),
                                _ => p.keep_lines = true,
                            }
                        }
                    } else {
                        p.keep_lines = true;
                    }
                }
            }
            Ok(())
        }
        //Page break before
        b"w:pageBreakBefore" => {
            if data.in_paragraph_properties {
                if let Some(ref mut p) = data.paragraph_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            match v.as_ref() {
                                "off" | "0" | "false" => (),
                                _ => p.page_break_before = true,
                            }
                        }
                    } else {
                        p.page_break_before = true;
                    }
                }
            }
            Ok(())
        }
        //Window control
        b"w:windowControl" => {
            if data.in_paragraph_properties {
                if let Some(ref mut p) = data.paragraph_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            match v.as_ref() {
                                "off" | "0" | "false" => (),
                                _ => p.window_control = true,
                            }
                        }
                    } else {
                        p.window_control = true;
                    }
                }
            }
            Ok(())
        }
        //Suppress line numbers
        b"w:suppressLineNumbers" => {
            if data.in_paragraph_properties {
                if let Some(ref mut p) = data.paragraph_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            match v.as_ref() {
                                "off" | "0" | "false" => (),
                                _ => p.suppress_line_numbers = true,
                            }
                        }
                    } else {
                        p.suppress_line_numbers = true;
                    }
                }
            }
            Ok(())
        }
        //Suppress auto hyphen
        b"w:suppressAutoHyphens" => {
            if data.in_paragraph_properties {
                if let Some(ref mut p) = data.paragraph_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            match v.as_ref() {
                                "off" | "0" | "false" => (),
                                _ => p.suppress_auto_hyphens = true,
                            }
                        }
                    } else {
                        p.suppress_auto_hyphens = true;
                    }
                }
            }
            Ok(())
        }
        //Word wrap
        b"w:wordWrap" => {
            if data.in_paragraph_properties {
                if let Some(ref mut p) = data.paragraph_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            match v.as_ref() {
                                "off" | "0" | "false" => (),
                                _ => p.word_wrap = true,
                            }
                        }
                    } else {
                        p.word_wrap = true;
                    }
                }
            }
            Ok(())
        }
        //Topline Punct
        b"w:toplinePunct" => {
            if data.in_paragraph_properties {
                if let Some(ref mut p) = data.paragraph_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            match v.as_ref() {
                                "off" | "0" | "false" => (),
                                _ => p.topline_punct = true,
                            }
                        }
                    } else {
                        p.topline_punct = true;
                    }
                }
            }
            Ok(())
        }
        // AutoSpaceDN
        b"w:autoSpaceDE" => {
            if data.in_paragraph_properties {
                if let Some(ref mut p) = data.paragraph_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            match v.as_ref() {
                                "off" | "0" | "false" => (),
                                _ => p.autospace_de = true,
                            }
                        }
                    } else {
                        p.autospace_de = true;
                    }
                }
            }
            Ok(())
        }
        // AutoSpaceDN
        b"w:autoSpaceDN" => {
            if data.in_paragraph_properties {
                if let Some(ref mut p) = data.paragraph_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            match v.as_ref() {
                                "off" | "0" | "false" => (),
                                _ => p.autospace_dn = true,
                            }
                        }
                    } else {
                        p.autospace_dn = true;
                    }
                }
            }
            Ok(())
        }
        // Bidi
        b"w:bidi" => {
            if data.in_paragraph_properties {
                if let Some(ref mut p) = data.paragraph_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            match v.as_ref() {
                                "off" | "0" | "false" => (),
                                _ => p.bidi = true,
                            }
                        }
                    } else {
                        p.bidi = true;
                    }
                }
            }
            Ok(())
        }
        // Snap to grid
        b"w:snapToGrid" => {
            if data.in_paragraph_properties {
                if let Some(ref mut p) = data.paragraph_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            match v.as_ref() {
                                "off" | "0" | "false" => (),
                                _ => p.snap_to_grid = true,
                            }
                        }
                    } else {
                        p.snap_to_grid = true;
                    }
                }
            }
            Ok(())
        }
        //Contextual Spacing
        b"w:contextualSpacing" => {
            if data.in_paragraph_properties {
                if let Some(ref mut p) = data.paragraph_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            match v.as_ref() {
                                "off" | "0" | "false" => (),
                                _ => p.contextual_spacing = true,
                            }
                        }
                    } else {
                        p.contextual_spacing = true;
                    }
                }
            }
            Ok(())
        }
        //Mirror indents
        b"w:mirrorIndents" => {
            if data.in_paragraph_properties {
                if let Some(ref mut p) = data.paragraph_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            match v.as_ref() {
                                "off" | "0" | "false" => (),
                                _ => p.mirror_indents = true,
                            }
                        }
                    } else {
                        p.mirror_indents = true;
                    }
                }
            }
            Ok(())
        }
        // Suppress overlap
        b"w:suppressOverlap" => {
            if data.in_paragraph_properties {
                if let Some(ref mut p) = data.paragraph_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            match v.as_ref() {
                                "off" | "0" | "false" => (),
                                _ => p.suppress_overlap = true,
                            }
                        }
                    } else {
                        p.suppress_overlap = true;
                    }
                }
            }
            Ok(())
        }
        // Justification
        b"w:jc" => {
            if data.in_paragraph_properties {
                if let Some(ref mut p) = data.paragraph_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            p.jc = Some(ParagraphJustification::new(
                                ParagraphJustificationValues::from(v.as_ref()),
                            ));
                        }
                    }
                }
            }
            Ok(())
        }
        // Text direction (apparently can be textFlow as well, so we include it here)
        b"w:textDirection" | b"w:textFlow" => {
            if data.in_paragraph_properties {
                if let Some(ref mut p) = data.paragraph_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            p.text_direction = Some(ParagraphTextDir::new(
                                ParagraphTextDirValues::from(v.as_ref()),
                            ));
                        }
                    }
                }
            }
            Ok(())
        }
        // Text Alignment
        b"w:textAlignment" => {
            if data.in_paragraph_properties {
                if let Some(ref mut p) = data.paragraph_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            p.text_alignment = Some(ParagraphTextAlign::new(
                                ParagraphTextAlignValues::from(v.as_ref()),
                            ));
                        }
                    }
                }
            }
            Ok(())
        }
        // TBox tight wrap
        b"w:textboxTightWrap" => {
            if data.in_paragraph_properties {
                if let Some(ref mut p) = data.paragraph_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            p.textbox_tight_wrap = Some(ParagraphTBoxTightWrap::new(
                                ParagraphTBoxTightWrapValues::from(v.as_ref()),
                            ));
                        }
                    }
                }
            }
            Ok(())
        }
        // Outline level
        b"w:outlineLvl" => {
            if data.in_paragraph_properties {
                if let Some(ref mut p) = data.paragraph_properties {
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            p.outline_level = Some(v.as_ref().parse::<u8>().unwrap_or(0));
                        }
                    }
                }
            }
            Ok(())
        }
        // Ind
        b"w:ind" => {
            if data.in_paragraph_properties {
                if let Some(ref mut p) = data.paragraph_properties {
                    let mut pi = ParagraphIndentation::default();
                    if let Some(Ok(a)) = attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:left")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            pi.left = Some(v.as_ref().parse::<i32>().unwrap_or(0));
                        }
                    }
                    if let Some(Ok(a)) =
                        attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:leftChars")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            pi.left_chars = Some(v.as_ref().parse::<i32>().unwrap_or(0));
                        }
                    }
                    if let Some(Ok(a)) =
                        attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:right")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            pi.right = Some(v.as_ref().parse::<i32>().unwrap_or(0));
                        }
                    }
                    if let Some(Ok(a)) =
                        attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:rightChars")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            pi.left = Some(v.as_ref().parse::<i32>().unwrap_or(0));
                        }
                    }
                    if let Some(Ok(a)) =
                        attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:hanging")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            pi.hanging = Some(v.as_ref().parse::<i32>().unwrap_or(0));
                        }
                    }
                    if let Some(Ok(a)) =
                        attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:hangingChars")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            pi.hanging_chars = Some(v.as_ref().parse::<i32>().unwrap_or(0));
                        }
                    }
                    if let Some(Ok(a)) =
                        attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:firstLine")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            pi.first_line = Some(v.as_ref().parse::<i32>().unwrap_or(0));
                        }
                    }
                    if let Some(Ok(a)) =
                        attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:firstLineChars")
                    {
                        if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                            pi.first_line_chars = Some(v.as_ref().parse::<i32>().unwrap_or(0));
                        }
                    }
                    p.ind = Some(pi);
                }
            }
            Ok(())
        }
        // Top | Left | Bottom | Right | Between (pBdr)
        b"w:top" | b"w:left" | b"w:bottom" | b"w:right" | b"w:between" => {
            if data.in_paragraph_properties {
                if let Some(ref mut pp) = data.paragraph_properties {
                    if let Some(ref mut p) = pp.paragraph_borders {
                        let mut pbs = ParagraphBorderSide::default();
                        if let Some(Ok(a)) =
                            attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:val")
                        {
                            if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                                pbs.val = ParagraphBorderStyle::from(v.as_ref());
                            }
                        }
                        if let Some(Ok(a)) =
                            attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:sz")
                        {
                            if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                                pbs.sz = Some(v.as_ref().parse::<u8>().unwrap_or(0));
                            }
                        }
                        if let Some(Ok(a)) =
                            attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:space")
                        {
                            if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                                pbs.space = Some(v.as_ref().parse::<u8>().unwrap_or(0));
                            }
                        }
                        if let Some(Ok(a)) =
                            attr.find(|x| x.clone().unwrap().key.as_ref() == b"w:color")
                        {
                            if let Ok(v) = a.decode_and_unescape_value(reader.decoder()) {
                                pbs.color = Some(HexColor::new(v.as_ref()));
                            }
                        }
                        p.top = Some(pbs);
                    }
                }
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

/// Handles closing a previously opened tag and performs the necessary operations to let the
/// temporary structure `CurrentData` keep with its work and not generate conflicts or overwrites.
fn handle_close_tag(tag: &[u8], data: &mut CurrentData) -> Result<(), RudocxError> {
    match tag {
        //Text
        b"w:t" => Ok(()),
        //Run Properties
        b"w:rPr" => {
            if data.in_paragraph_properties {
                return Ok(());
            }
            data.in_run_properties = false;
            Ok(())
        }
        //Paragraph
        b"w:p" => {
            if let Some(mut p) = data.paragraph.take() {
                if let Some(mut h) = data.hyperlink.take() {
                    if let Some(r) = data.run.take() {
                        h.runs.push(r);
                    }
                    p.children.push(ParagraphChild::Hyperlink(h));
                    data.document.paragraphs.push(p);
                } else {
                    if let Some(r) = data.run.take() {
                        p.children.push(ParagraphChild::Run(r));
                        data.document.paragraphs.push(p);
                    } else {
                        data.document.paragraphs.push(p);
                    }
                }
            }
            data.paragraph = None;
            Ok(())
        }
        //Hyperlink
        b"w:hyperlink" => {
            if let Some(mut h) = data.hyperlink.take() {
                if let Some(mut r) = data.run.take() {
                    if let Some(rp) = data.run_properties.take() {
                        r.properties = rp;
                    }
                    h.runs.push(r);
                }
                if let Some(ref mut p) = data.paragraph {
                    p.children.push(ParagraphChild::Hyperlink(h));
                }
            }
            data.hyperlink = None;
            Ok(())
        }
        //Run
        b"w:r" => {
            if let Some(mut r) = data.run.take() {
                if let Some(rp) = data.run_properties.take() {
                    r.properties = rp;
                }
                if let Some(ref mut h) = data.hyperlink {
                    h.runs.push(r);
                } else {
                    if let Some(ref mut p) = data.paragraph {
                        p.children.push(ParagraphChild::Run(r));
                    }
                }
            }
            data.run = None;
            Ok(())
        }
        // Some tags doesn't need any closing logic: pBdr
        _ => Ok(()),
    }
}

/// Handle reaching the end of the file. This will correctly close all the structures and
/// perform the last operations to generate a valid document.
fn handle_eof(data: &mut CurrentData) -> Result<(), RudocxError> {
    if let Some(p) = data.paragraph.take() {
        if let Some(mut h) = data.hyperlink.take() {
            if let Some(mut p) = data.paragraph.take() {
                if let Some(r) = data.run.take() {
                    h.runs.push(r);
                    p.children.push(ParagraphChild::Hyperlink(h));
                }
            }
        }
        if let Some(r) = data.run.take() {
            if let Some(mut p) = Some(p) {
                p.children.push(ParagraphChild::Run(r));
                data.document.paragraphs.push(p);
            }
        } else {
            data.document.paragraphs.push(p);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    //TODO: Extend example XML to include current defined properties and structs
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
                    <w:p>
                        <w:hyperlink r:id="rId1">
                            <w:r><w:rPr><w:i/></w:rPr><w:t>www.github.com/cmgsk/rudocx</w:t></w:r>
                        </w:hyperlink>
                        <w:r><w:t> That was hyperlink.</w:t></w:r>
                    </w:p>
                </w:body>
            </w:document>
        "#;

        let result = parse(xml_input);
        assert!(result.is_ok());
        let doc = result.unwrap();

        assert_eq!(doc.paragraphs.len(), 4);

        // Paragraph 1: Plain text
        assert_eq!(doc.paragraphs[0].children.len(), 1);
        if let Some(p) = doc.paragraphs.iter().nth(0) {
            if let Some(ParagraphChild::Run(r)) = p.children.iter().nth(0) {
                assert_eq!(r.text, "This is plain text.");
                assert!(!r.properties.bold);
                assert!(!r.properties.italic);
            } else {
                assert!(false);
            }
        }

        // Paragraph 2: Bold, space, Italic
        assert_eq!(doc.paragraphs[1].children.len(), 3);
        if let Some(p) = doc.paragraphs.iter().nth(1) {
            // Run 1: Bold
            if let Some(ParagraphChild::Run(r)) = p.children.iter().nth(0) {
                assert_eq!(r.text, "This is bold.");
                assert!(r.properties.bold);
                assert!(!r.properties.italic);
            } else {
                assert!(false);
            }
            // Run 2: Space (should be preserved)
            if let Some(ParagraphChild::Run(r)) = p.children.iter().nth(1) {
                assert_eq!(r.text, " ");
                assert!(!r.properties.bold);
                assert!(!r.properties.italic);
            } else {
                assert!(false);
            }
            // Run 3: Italic
            if let Some(ParagraphChild::Run(r)) = p.children.iter().nth(2) {
                assert_eq!(r.text, "This is italic.");
                assert!(!r.properties.bold);
                assert!(r.properties.italic);
            } else {
                assert!(false);
            }
        }

        // Paragraph 3: Bold and Italic
        assert_eq!(doc.paragraphs[2].children.len(), 1);
        if let Some(p) = doc.paragraphs.iter().nth(2) {
            if let Some(ParagraphChild::Run(r)) = p.children.iter().nth(0) {
                assert_eq!(r.text, "Bold and Italic.");
                assert!(r.properties.bold);
                assert!(r.properties.italic);
            } else {
                assert!(false);
            }
        }

        // Paragraph 3: Hyperlink and Plain
        assert_eq!(doc.paragraphs[3].children.len(), 2);
        if let Some(p) = doc.paragraphs.iter().nth(3) {
            // Child 1 (hyperlink)
            if let Some(ParagraphChild::Hyperlink(h)) = p.children.iter().nth(0) {
                assert_eq!(h.id, "rId1");
                assert_eq!(h.runs.len(), 1);
                assert_eq!(h.runs[0].text, "www.github.com/cmgsk/rudocx");
                assert!(!h.runs[0].properties.bold);
                assert!(h.runs[0].properties.italic);
            } else {
                assert!(false);
            }
            // Child 2 (run)
            if let Some(ParagraphChild::Run(r)) = p.children.iter().nth(1) {
                assert_eq!(r.text, " That was hyperlink.");
                assert!(!r.properties.bold);
                assert!(!r.properties.italic);
            } else {
                assert!(false);
            }
        }
    }
}
