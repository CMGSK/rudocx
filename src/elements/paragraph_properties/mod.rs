use std::default::Default;

use crate::{elements::RunProperties, errors::RudocxParagraphStyleError};

mod border;
mod indentation;
mod numbering;
mod shadow;
mod spacing;
mod tabs;

pub use border::*;
pub use indentation::*;
pub use numbering::*;
pub use shadow::*;
pub use spacing::*;
pub use tabs::*;

type Result<T> = std::result::Result<T, RudocxParagraphStyleError>;

//TODO: Documentation
///We're currently missing: Style, framePr, Kinsoku, overflow punctuation, divId, cnfStyle
#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphProperties {
    pub keep_next: bool,
    pub keep_lines: bool,
    pub page_break_before: bool,
    pub window_control: bool,
    pub supress_line_numbers: bool,
    pub borders: Option<ParagraphBorder>,
    pub shadow: Option<ParagraphShadow>,
    pub tabs: Option<ParagraphTab>,
    pub numbering_properties: Option<ParagraphNumberingProperties>,
    pub suppress_auto_hyphens: bool,
    pub word_wrap: bool,
    pub topline_punct: bool,
    pub autospace_de: bool,
    pub autospace_dn: bool,
    pub bidi: bool,
    pub snap_to_grid: bool,
    pub spacing: Option<ParagraphSpacing>,
    pub ind: Option<ParagraphIndentation>,
    pub contextual_spacing: bool,
    pub mirror_indents: bool,
    pub suppress_overlap: bool,
    pub jc: Option<ParagraphJustification>,
    pub text_direction: Option<ParagraphTextDir>,
    pub text_alignment: Option<ParagraphTextAlign>,
    pub textbox_tight_wrap: Option<ParagraphTBoxTightWrap>,
    pub outline_level: Option<u8>,
    pub default_run_properties: Option<RunProperties>,
}

impl Default for ParagraphProperties {
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}
