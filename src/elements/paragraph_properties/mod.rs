use std::default::Default;

use crate::elements::RunProperties;

mod alignment;
mod border;
mod direction;
mod indentation;
mod justification;
mod numbering;
mod shading;
mod spacing;
mod tabs;
mod textbox_tight_wrap;

pub use alignment::*;
pub use border::*;
pub use direction::*;
pub use indentation::*;
pub use justification::*;
pub use numbering::*;
pub use shading::*;
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
    pub suppress_line_numbers: bool,
    pub paragraph_borders: Option<ParagraphBorder>,
    pub shading: Option<ParagraphShading>,
    pub tabs: Option<Vec<ParagraphTab>>,
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

impl ParagraphProperties {
    pub fn builder() -> ParagraphPropertiesBuilder {
        ParagraphPropertiesBuilder::new()
    }

    pub fn has_formatting(&self) -> bool {
        self != &Self::default()
    }
}

pub struct ParagraphPropertiesBuilder {
    inner: ParagraphProperties,
}

impl ParagraphPropertiesBuilder {
    pub fn new() -> Self {
        Self {
            inner: ParagraphProperties::default(),
        }
    }

    pub fn build(self) -> ParagraphProperties {
        self.inner
    }

    // Builder methods for all properties.
    pub fn keep_next(mut self, v: bool) -> Self {
        self.inner.keep_next = v;
        self
    }

    pub fn keep_lines(mut self, v: bool) -> Self {
        self.inner.keep_lines = v;
        self
    }

    pub fn page_break_before(mut self, v: bool) -> Self {
        self.inner.page_break_before = v;
        self
    }

    pub fn window_control(mut self, v: bool) -> Self {
        self.inner.window_control = v;
        self
    }

    pub fn supress_line_numbers(mut self, v: bool) -> Self {
        self.inner.suppress_line_numbers = v;
        self
    }

    pub fn borders(mut self, v: Option<ParagraphBorder>) -> Self {
        self.inner.paragraph_borders = v;
        self
    }

    pub fn shading(mut self, v: Option<ParagraphShading>) -> Self {
        self.inner.shading = v;
        self
    }

    pub fn tabs(mut self, v: Option<Vec<ParagraphTab>>) -> Self {
        self.inner.tabs = v;
        self
    }

    pub fn numbering_properties(mut self, v: Option<ParagraphNumberingProperties>) -> Self {
        self.inner.numbering_properties = v;
        self
    }

    pub fn suppress_auto_hyphens(mut self, v: bool) -> Self {
        self.inner.suppress_auto_hyphens = v;
        self
    }

    pub fn word_wrap(mut self, v: bool) -> Self {
        self.inner.word_wrap = v;
        self
    }

    pub fn topline_punct(mut self, v: bool) -> Self {
        self.inner.topline_punct = v;
        self
    }

    pub fn autospace_de(mut self, v: bool) -> Self {
        self.inner.autospace_de = v;
        self
    }

    pub fn autospace_dn(mut self, v: bool) -> Self {
        self.inner.autospace_dn = v;
        self
    }

    pub fn bidi(mut self, v: bool) -> Self {
        self.inner.bidi = v;
        self
    }

    pub fn snap_to_grid(mut self, v: bool) -> Self {
        self.inner.snap_to_grid = v;
        self
    }

    pub fn spacing(mut self, v: Option<ParagraphSpacing>) -> Self {
        self.inner.spacing = v;
        self
    }

    pub fn ind(mut self, v: Option<ParagraphIndentation>) -> Self {
        self.inner.ind = v;
        self
    }

    pub fn contextual_spacing(mut self, v: bool) -> Self {
        self.inner.contextual_spacing = v;
        self
    }

    pub fn mirror_indents(mut self, v: bool) -> Self {
        self.inner.mirror_indents = v;
        self
    }

    pub fn suppress_overlap(mut self, v: bool) -> Self {
        self.inner.suppress_overlap = v;
        self
    }

    pub fn jc(mut self, v: Option<ParagraphJustification>) -> Self {
        self.inner.jc = v;
        self
    }

    pub fn text_direction(mut self, v: Option<ParagraphTextDir>) -> Self {
        self.inner.text_direction = v;
        self
    }

    pub fn text_alignment(mut self, v: Option<ParagraphTextAlign>) -> Self {
        self.inner.text_alignment = v;
        self
    }

    pub fn textbox_tight_wrap(mut self, v: Option<ParagraphTBoxTightWrap>) -> Self {
        self.inner.textbox_tight_wrap = v;
        self
    }

    pub fn outline_level(mut self, v: Option<u8>) -> Self {
        self.inner.outline_level = v;
        self
    }

    pub fn default_run_properties(mut self, v: Option<RunProperties>) -> Self {
        self.inner.default_run_properties = v;
        self
    }
}
