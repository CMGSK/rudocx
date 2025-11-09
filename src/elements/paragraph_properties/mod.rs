use std::default::Default;

use crate::{elements::RunProperties, errors::RudocxParagraphStyleError};

mod alignment;
mod border;
mod direction;
mod indentation;
mod justification;
mod numbering;
mod shadow;
mod spacing;
mod tabs;
mod textbox_tight_wrap;

pub use alignment::*;
pub use border::*;
pub use direction::*;
pub use indentation::*;
pub use justification::*;
pub use numbering::*;
pub use shadow::*;
pub use spacing::*;
pub use tabs::*;
pub use textbox_tight_wrap::*;

/// Representation of the paragraph formatting properties applied to a `Paragraph` in a docx document.
///
/// All properties internal values are public, however, modifying or accessing them directly is discouraged if you're not sure
/// what you're doing. For naive uses of this library, we provide a set of getters and setters that will handle the correct
/// behaviour.
///
/// ### Fields
/// > - **keep_next:** `bool` - Keep paragraph with next paragraph [`w:keepNext`]
/// > - **keep_lines:** `bool` - Keep all lines of paragraph on one page [`w:keepLines`]
/// > - **page_break_before:** `bool` - Start paragraph on next page [`w:pageBreakBefore`]
/// > - **window_control:** `bool` - Window/orphan control [`w:widowControl`]
/// > - **supress_line_numbers:** `bool` - Suppress line numbers for paragraph [`w:suppressLineNumbers`]
/// > - **borders:** `Option<ParagraphBorder>` - Paragraph borders [`w:pBdr`]. `None` is unused.
/// > - **shading:** `Option<ParagraphShadow>` - Paragraph shading [`w:shd`]. `None` is unused.
/// > - **tabs:** `Option<ParagraphTab>` - Custom tab stops [`w:tabs`]. `None` is unused.
/// > - **numbering_properties:** `Option<ParagraphNumberingProperties>` - Numbering definition instance reference [`w:numPr`]. `None` is unused.
/// > - **suppress_auto_hyphens:** `bool` - Suppress hyphenation for paragraph [`w:suppressAutoHyphens`]
/// > - **word_wrap:** `bool` - Allow line breaking at character level [`w:wordWrap`]
/// > - **topline_punct:** `bool` - Compress punctuation at start of a line [`w:topLinePunct`]
/// > - **autospace_de:** `bool` - Automatically adjust spacing of Latin and East Asian text [`w:autoSpaceDE`]
/// > - **autospace_dn:** `bool` - Automatically adjust spacing of East Asian text and numbers [`w:autoSpaceDN`]
/// > - **bidi:** `bool` - Right to left paragraph layout [`w:bidi`]
/// > - **snap_to_grid:** `bool` - Use document grid settings for inter-line paragraph spacing [`w:snapToGrid`]
/// > - **spacing:** `Option<ParagraphSpacing>` - Spacing between lines and paragraphs [`w:spacing`]. `None` is unused.
/// > - **ind:** `Option<ParagraphIndentation>` - Paragraph indentation [`w:ind`]. `None` is unused.
/// > - **contextual_spacing:** `bool` - Ignore spacing above and below when using identical styles [`w:contextualSpacing`]
/// > - **mirror_indents:** `bool` - Use left/right indents as inside/outside indents [`w:mirrorIndents`]
/// > - **suppress_overlap:** `bool` - Prevent text frames from overlapping [`w:suppressOverlap`]
/// > - **jc:** `Option<ParagraphJustification>` - Paragraph alignment [`w:jc`]. `None` defaults to left alignment.
/// > - **text_direction:** `Option<ParagraphTextDir>` - Paragraph text flow direction [`w:textDirection`]. `None` is unused.
/// > - **text_alignment:** `Option<ParagraphTextAlign>` - Vertical character alignment on line [`w:textAlignment`]. `None` is unused.
/// > - **textbox_tight_wrap:** `Option<ParagraphTBoxTightWrap>` - Text box tight wrapping [`w:textboxTightWrap`]. `None` is unused.
/// > - **outline_level:** `Option<u8>` - Associated outline level (0-9) [`w:outlineLvl`]. `None` is unused.
/// > - **default_run_properties:** `Option<RunProperties>` - Default run properties for paragraph [`w:rPr`]. `None` is unused.
///
/// Note: We're currently missing support for: Style (`w:pStyle`), framePr (`w:framePr`), Kinsoku (`w:kinsoku`),
/// overflow punctuation (`w:overflowPunct`), divId (`w:divId`), and cnfStyle (`w:cnfStyle`). These may be added in future versions.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ParagraphProperties {
    pub keep_next: bool,
    pub keep_lines: bool,
    pub page_break_before: bool,
    pub window_control: bool,
    pub supress_line_numbers: bool,
    /// Contains top, bottom, left, right, between, and bar border definitions
    pub borders: Option<ParagraphBorder>,
    /// Contains fill color, pattern, and background color for paragraph shading
    pub shading: Option<ParagraphShading>,
    /// Contains a collection of custom tab stop positions and alignments
    pub tabs: Option<ParagraphTab>,
    /// Contains numbering level (ilvl) and numbering definition ID (numId)
    pub numbering_properties: Option<ParagraphNumberingProperties>,
    pub suppress_auto_hyphens: bool,
    pub word_wrap: bool,
    pub topline_punct: bool,
    pub autospace_de: bool,
    pub autospace_dn: bool,
    pub bidi: bool,
    pub snap_to_grid: bool,
    /// Contains before/after spacing, line spacing rule, and line spacing value
    pub spacing: Option<ParagraphSpacing>,
    /// Contains left, right, first line, and hanging indentation values
    pub ind: Option<ParagraphIndentation>,
    pub contextual_spacing: bool,
    pub mirror_indents: bool,
    pub suppress_overlap: bool,
    /// Contains alignment value: left, center, right, both (justified), distribute, etc.
    pub jc: Option<ParagraphJustification>,
    /// Contains text flow direction: lr-tb, tb-rl, bt-lr, lr-tb-v, tb-rl-v, tb-lr-v
    pub text_direction: Option<ParagraphTextDir>,
    /// Contains vertical alignment on line: top, center, baseline, bottom, auto
    pub text_alignment: Option<ParagraphTextAlign>,
    /// Contains tight wrap setting for text boxes: none, allLines, firstAndLastLine, firstLineOnly, lastLineOnly
    pub textbox_tight_wrap: Option<ParagraphTBoxTightWrap>,
    pub outline_level: Option<u8>,
    /// Contains default text formatting (bold, italic, font, etc.) for all runs in this paragraph
    pub default_run_properties: Option<RunProperties>,
}
