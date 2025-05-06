use crate::errors::RudocxStyleError;

pub use color::*;
pub use font::*;
pub use underline::*;
pub use vertical_align::*;

mod color;
mod font;
mod underline;
mod vertical_align;

type Result<T> = std::result::Result<T, RudocxStyleError>;

/// Representation of the format applied to a text `Run` in a docx document.
///
/// All properties internal values are public, however, modifying or accessing them directly is discouraged if you're not sure
/// what you're doing. For naive uses of this library, we provide a set of getters and setters that will handle the correct
/// behaviour.
///
/// ### Fields
/// > - **bold:** `bool` - Indicates if a text is bold [`w:b`]
/// > - **italic:** `bool` - Indicates if a text is italic [`w:i`]
/// > - **underline:** `Option<Underline>` - Indicates the `Underline` of a text [`w:b`]. `None` is unused.
/// > - **color:** `Option<HexColor>` - Indicates the `HexColor` of a text font. `None` defaults to `FFFFFF`. _Note:_ XML tag value does **not** prepend the `#` to the HEX code. [`w:color w:val="<HEX_VAL>"`]()
/// > - **size:** `Option<u32>` - Indicates the font size of a text in half points (e.g. `21` == `10.5 pt.`). `None` defaults to 22 (11pt). [`w:sz w:val="<NUM>"`]()
/// > - **font:** `Option<FontSet>` - Indicates the `FontSet` of a text. For `None` and other details, please refere to: [FontSet](crate::properties::FontSet) [`w:rFonts[...]`]()
/// > - **highlight:** `Option<HLColor>` - Indicates the highlighting `HLColor` of a text. `None` is unused. Only predefined colors are accepted. For custom coloring, `Shading` is used instead. [`w:highlight w:val="<COLOR>"`]()
///
/// Note: It's not in the scope right now to add direct support for `Cs` `TypeFont` properties such as szCs, bCs, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct RunProperties {
    pub bold: bool,
    pub italic: bool,
    pub underline: Option<Underline>,
    pub color: Option<HexColor>,
    pub size: Option<u32>,
    pub font: Option<FontSet>,
    pub highlight: Option<HLColor>,
    pub strike: bool,
    pub dstrike: bool,
    pub valign: Option<VerticalAlign>,
    pub spacing: Option<u32>,
}

impl Default for RunProperties {
    fn default() -> Self {
        Self {
            bold: false,
            italic: false,
            underline: None,
            color: None,
            size: None,
            font: None,
            highlight: None,
            strike: false,
            dstrike: false,
            valign: None,
            spacing: None,
        }
    }
}

impl RunProperties {
    pub fn new(
        bold: bool,
        italic: bool,
        underline: Option<Underline>,
        color: Option<HexColor>,
        size: Option<u32>,
        font: Option<FontSet>,
        highlight: Option<HLColor>,
        strike: bool,
        dstrike: bool,
        valign: Option<VerticalAlign>,
        spacing: Option<u32>,
    ) -> Self {
        Self {
            bold,
            italic,
            underline,
            color,
            size,
            font,
            highlight,
            strike,
            dstrike,
            valign,
            spacing,
        }
    }

    pub fn has_formatting(&self) -> bool {
        self != &Self::default()
    }
}
