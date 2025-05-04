use crate::errors::RudocxStyleError;
use std::fmt;
use std::fmt::Formatter;
use std::path::Path;

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
    ) -> Self {
        Self {
            bold,
            italic,
            underline,
            color,
            size,
            font,
            highlight,
        }
    }

    pub fn has_formatting(&self) -> bool {
        self == &Self::default()
    }
}

// --- Colors ---

///Represents a HEX color code, without the `#` character.
#[derive(Debug, Clone, PartialEq)]
pub struct HexColor {
    pub value: String,
}

impl Default for HexColor {
    fn default() -> Self {
        Self {
            value: String::from("FFFFFF"),
        }
    }
}

impl HexColor {
    /// Receives a HEX color code. `#` is **NOT** required. Alpha is not supported. Wrong input defaults to Black.
    pub fn new(color: &str) -> Self {
        match check_hex(color) {
            Ok(_) => Self {
                value: String::from(color),
            },
            Err(_) => Self {
                value: String::from("FFFFFF"),
            },
        }
    }

    /// Get the value of the struct as `String`.
    pub fn value(&self) -> String {
        self.value.clone()
    }

    /// Change the value of the struct. Same rules as [new](crate::properties::HexColor::new) apply, but wrong input value results in an `Err()`
    pub fn change_value(&mut self, value: &str) -> Result<()> {
        match check_hex(value) {
            Ok(_) => Ok(self.value = value.to_string()),
            Err(e) => Err(e),
        }
    }
}

fn check_hex(value: &str) -> Result<()> {
    if !value.len() == 6 {
        return Err(RudocxStyleError::InvalidHex(value.to_string()));
    }
    if !value.chars().all(|x| x.is_ascii_hexdigit()) {
        return Err(RudocxStyleError::InvalidHex(value.to_string()));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub struct HLColor {
    pub value: Option<HighlightPalette>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HighlightPalette {
    Yellow,
    DarkYellow,
    Green,
    DarkGreen,
    Cyan,
    DarkCyan,
    Magenta,
    DarkMagenta,
    Blue,
    DarkBlue,
    Red,
    DarkRed,
    Black,
    White,
    // Note: "None" is represented by Option::None in the HLColor struct value.
}

impl fmt::Display for HighlightPalette {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Yellow => "yellow",
                Self::DarkYellow => "darkYellow",
                Self::Green => "green",
                Self::DarkGreen => "darkGreen",
                Self::Cyan => "cyan",
                Self::DarkCyan => "darkCyan",
                Self::Magenta => "magenta",
                Self::DarkMagenta => "darkMagenta",
                Self::Blue => "blue",
                Self::DarkBlue => "darkBlue",
                Self::Red => "red",
                Self::DarkRed => "darkRed",
                Self::Black => "black",
                Self::White => "white",
            }
        )
    }
}

// --- Underlines ---

///Represents an underline style.
#[derive(Debug, Clone, PartialEq)]
pub struct Underline {
    pub value: Option<UnderlineStyle>,
}

impl Default for Underline {
    fn default() -> Self {
        Self { value: None }
    }
}

impl Underline {
    pub fn new(style: UnderlineStyle) -> Self {
        Self { value: Some(style) }
    }

    pub fn value(&self) -> String {
        match self.value.clone() {
            Some(v) => v.to_string(),
            None => "None".to_string(),
        }
    }

    pub fn change_value(&mut self, value: Option<UnderlineStyle>) -> Result<()> {
        self.value = value;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnderlineStyle {
    Single,
    Words,
    Double,
    Thick,
    Dotted,
    DottedHeavy,
    Dash,
    DashedHeavy,
    DashLong,
    DashLongHeavy,
    DotDash,
    DashDotHeavy,
    DotDotDash,
    DashDotDotHeavy,
    Wave,
    WavyHeavy,
    WavyDouble,
    // Note: "None" is represented by Option::None in the Underline struct value.
}

impl fmt::Display for UnderlineStyle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UnderlineStyle::Single => "single",
                UnderlineStyle::Words => "words",
                UnderlineStyle::Double => "double",
                UnderlineStyle::Thick => "thick",
                UnderlineStyle::Dotted => "dotted",
                UnderlineStyle::DottedHeavy => "dottedHeavy",
                UnderlineStyle::Dash => "dash",
                UnderlineStyle::DashedHeavy => "dashedHeavy",
                UnderlineStyle::DashLong => "dashLong",
                UnderlineStyle::DashLongHeavy => "dashLongHeavy",
                UnderlineStyle::DotDash => "dotDash",
                UnderlineStyle::DashDotHeavy => "dashDotHeavy",
                UnderlineStyle::DotDotDash => "dotDotDash",
                UnderlineStyle::DashDotDotHeavy => "dashDotDotHeavy",
                UnderlineStyle::Wave => "wave",
                UnderlineStyle::WavyHeavy => "wavyHeavy",
                UnderlineStyle::WavyDouble => "wavyDouble",
            }
        )
    }
}

// --- FontSet ---

/// Represents font settings for a run in a DOCX document.
///
/// Controls which fonts are used for different character types in the `w:rFonts` XML tag (e.g. [`<w:rFonts ascii="Arial" hAnsi="Calibri" cs="Times New Roman"/>`]())
///
/// Docx specifications define that an rFonts tag can appear empty, and this will result in a fallback to the default theme/style/font defined in the software.
/// This, however, does not apply to this specific struct, where although all of the values within it can be `None`. Constructor always fallback to a default.
/// If getter is invoked with all attributes set to `None`, it will result in an `Err()`.
///
/// ## Fields
/// > - `ascii`: Font name for ASCII characters (U+0000–U+007F).
/// > - `hAnsi`: Font for high ANSI characters (U+0080+), e.g., accented letters.
/// > - `eastAsia`: Font for East Asian scripts (e.g., Chinese, Japanese).
/// > - `cs`: Font for complex scripts (e.g., Arabic, Hindi).
/// > - `ascii_theme`: Theme font for ASCII (e.g., "minorAscii").
/// > - `hAnsi_theme`: Theme font for high ANSI.
/// > - `eastAsia_theme`: Theme font for East Asian.
/// > - `cs_theme`: Theme font for complex scripts.
/// > - `hint`: Font rendering hint (e.g. `default`, `eastAsia`, `cs`...). Defaults to `ascii`.
///
/// **Note:** Checking whether the selected font has the correct `FontType` is not in the scope of this library. Using a `cs` font as `ascii` or vice-versa can
/// result in unexpected rendering or behaviours depending on your software.
///
#[derive(Debug, Clone, PartialEq)]
pub struct FontSet {
    pub ascii: Option<String>,
    pub hi_ansi: Option<String>,
    pub east_asia: Option<String>,
    pub cs: Option<String>,
    pub ascii_theme: Option<String>,
    pub hi_ansi_theme: Option<String>,
    pub east_asia_theme: Option<String>,
    pub cs_theme: Option<String>,
    pub hint: FontType,
}

impl Default for FontSet {
    fn default() -> Self {
        Self {
            ascii: None,
            hi_ansi: None,
            east_asia: None,
            cs: None,
            ascii_theme: None,
            hi_ansi_theme: None,
            east_asia_theme: None,
            cs_theme: None,
            hint: FontType::Default,
        }
    }
}

/// Trying to set or access a `default` hint value will result in an `Err`. Default value is a fallback to fetch the value from software or system configurations.
#[derive(Debug, Clone, PartialEq)]
pub enum FontType {
    Ascii,
    HiAnsi,
    EastAsia,
    Cs,
    AsciiTheme,
    HiAnsiTheme,
    EastAsiaTheme,
    CsTheme,
    Default,
}

impl fmt::Display for FontType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FontType::Ascii => write!(f, "ascii"),
            FontType::HiAnsi => write!(f, "hAnsi"),
            FontType::EastAsia => write!(f, "eastAsia"),
            FontType::Cs => write!(f, "cs"),
            FontType::AsciiTheme => write!(f, "asciiTheme"),
            FontType::HiAnsiTheme => write!(f, "hiAnsiTheme"),
            FontType::EastAsiaTheme => write!(f, "eastAsiaTheme"),
            FontType::CsTheme => write!(f, "csTheme"),
            FontType::Default => write!(f, "default"),
        }
    }
}

impl FontSet {
    /// Creates a new font set with a single `FontType`. Once created, you can also set other `FontType` through [get_value](crate::properties::FontSet::change_value)
    pub fn new(value: String, r#type: FontType) -> Result<Self> {
        let mut new_font = Self::default();
        Self::check_font(&value.clone())?;

        match r#type {
            FontType::Ascii => new_font.ascii = Some(value),
            FontType::HiAnsi => new_font.hi_ansi = Some(value),
            FontType::EastAsia => new_font.east_asia = Some(value),
            FontType::Cs => new_font.cs = Some(value),
            FontType::AsciiTheme => new_font.ascii_theme = Some(value),
            FontType::HiAnsiTheme => new_font.hi_ansi_theme = Some(value),
            FontType::EastAsiaTheme => new_font.east_asia_theme = Some(value),
            FontType::CsTheme => new_font.cs_theme = Some(value),
            FontType::Default => {
                return Err(RudocxStyleError::PropertyNotSet(String::from(
                    "FontSet. Default is fallback.",
                )));
            }
        };

        Ok(new_font)
    }

    /// Get the value of the FontType defined at the Hint property. If the
    pub fn value(&self) -> Result<String> {
        if &Self::default() == self {
            return Err(RudocxStyleError::EmptyFontSet);
        }
        self.get_hint()
    }

    /// Get the value of the FontType defined at the Hint property
    pub fn get_hint(&self) -> Result<String> {
        let hint = match self.hint {
            FontType::Ascii => self.ascii.clone(),
            FontType::HiAnsi => self.hi_ansi.clone(),
            FontType::EastAsia => self.east_asia.clone(),
            FontType::Cs => self.cs.clone(),
            FontType::AsciiTheme => self.ascii_theme.clone(),
            FontType::HiAnsiTheme => self.hi_ansi_theme.clone(),
            FontType::EastAsiaTheme => self.east_asia_theme.clone(),
            FontType::CsTheme => self.cs_theme.clone(),
            FontType::Default => {
                return Err(RudocxStyleError::Undefined(String::from(
                    "Default hint is fallback. Value must come from Software or System configuration.",
                )));
            }
        };

        match hint.clone() {
            Some(s) => Ok(s),
            None => Err(RudocxStyleError::HintPointsNone(self.hint.clone())),
        }
    }

    /// Get the value of the Hint property
    pub fn get_hint_value(&self) -> FontType {
        self.hint.clone()
    }

    /// Set the value of the `hint` property.
    ///
    /// **Note:** If set to `None`, take into consideration the `docx` standard behaviour.
    pub fn set_hint_value(&mut self, value: FontType) -> Result<()> {
        self.hint = value;
        Ok(())
    }

    /// Change or set the value of a specific `FontType` within your `FontSet`. Returns `Err` if all `FontSet` internal values result as `None`
    /// as a consequence of your change.
    pub fn change_value(&mut self, value: Option<String>, r#type: FontType) -> Result<()> {
        match r#type {
            FontType::Ascii => {
                if self != &Self::default() {
                    if value.is_some() {
                        Self::check_font(&value.clone().unwrap())?;
                    }
                    self.ascii = value;
                } else {
                    return Err(RudocxStyleError::PropertyNotSet(String::from(
                        "FontSet contains no values",
                    )));
                }
            }
            FontType::HiAnsi => {
                if self != &Self::default() {
                    if value.is_some() {
                        Self::check_font(&value.clone().unwrap())?;
                    }
                    self.hi_ansi = value;
                } else {
                    return Err(RudocxStyleError::PropertyNotSet(String::from(
                        "FontSet contains no values",
                    )));
                }
            }
            FontType::EastAsia => {
                if self != &Self::default() {
                    if value.is_some() {
                        Self::check_font(&value.clone().unwrap())?;
                    }
                    self.east_asia = value;
                } else {
                    return Err(RudocxStyleError::PropertyNotSet(String::from(
                        "FontSet contains no values",
                    )));
                }
            }
            FontType::Cs => {
                if self != &Self::default() {
                    if value.is_some() {
                        Self::check_font(&value.clone().unwrap())?;
                    }
                    self.cs = value;
                } else {
                    return Err(RudocxStyleError::PropertyNotSet(String::from(
                        "FontSet contains no values",
                    )));
                }
            }
            FontType::AsciiTheme => {
                if self != &Self::default() {
                    if value.is_some() {
                        Self::check_font(&value.clone().unwrap())?;
                    }
                    self.ascii_theme = value;
                } else {
                    return Err(RudocxStyleError::PropertyNotSet(String::from(
                        "FontSet contains no values",
                    )));
                }
            }
            FontType::HiAnsiTheme => {
                if self != &Self::default() {
                    if value.is_some() {
                        Self::check_font(&value.clone().unwrap())?;
                    }
                    self.hi_ansi_theme = value;
                } else {
                    return Err(RudocxStyleError::PropertyNotSet(String::from(
                        "FontSet contains no values",
                    )));
                }
            }
            FontType::EastAsiaTheme => {
                if self != &Self::default() {
                    if value.is_some() {
                        Self::check_font(&value.clone().unwrap())?;
                    }
                    self.east_asia_theme = value;
                } else {
                    return Err(RudocxStyleError::PropertyNotSet(String::from(
                        "FontSet contains no values",
                    )));
                }
            }
            FontType::CsTheme => {
                if self != &Self::default() {
                    if value.is_some() {
                        Self::check_font(&value.clone().unwrap())?;
                    }
                    self.cs = value;
                } else {
                    return Err(RudocxStyleError::PropertyNotSet(String::from(
                        "FontSet contains no values",
                    )));
                }
            }
            FontType::Default => {
                return Err(RudocxStyleError::Undefined(String::from(
                    "Default hint is fallback. Value must come from Software or System configuration.",
                )));
            }
        };
        Ok(())
    }

    fn check_font(font: &str) -> Result<()> {
        #[cfg(target_os = "linux")]
        {
            let dirs = [
                "/usr/share/fonts/",
                "/usr/local/share/fonts/",
                &format!("{}/.fonts", std::env::var("HOME").unwrap()),
            ];
            let fonts = dirs
                .iter()
                .flat_map(|x| list_fonts(x))
                .collect::<Vec<String>>();
            return check_installed(font, fonts);
        }

        #[cfg(target_os = "windows")]
        {
            let fonts = list_fonts("C:\\Windows\\Fonts");
            return check_installed(font, fonts);
        }

        #[cfg(target_os = "macos")]
        {
            let dirs = [
                "/System/Library/Fonts",
                "/Library/Fonts",
                &format!("{}/Library/Fonts", std::env::var("HOME").unwrap()),
            ];
            let fonts = dirs
                .iter()
                .flat_map(|x| list_fonts(x))
                .collect::<Vec<String>>();
            return check_installed(font, fonts);
        }

        fn list_fonts<P: AsRef<Path>>(path: P) -> Vec<String> {
            let mut fonts: Vec<String> = Vec::new();
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if let Some(ext) = p.extension() {
                        if ext == "ttf" || ext == "otf" {
                            if let Some(name) = p.file_name().and_then(|x| x.to_str()) {
                                fonts.push(name.to_string());
                            }
                        }
                    }
                }
            }

            fonts
        }

        fn check_installed(value: &str, fonts: Vec<String>) -> Result<()> {
            if fonts.is_empty() {
                Err(RudocxStyleError::SystemFontsNotFound)
            } else {
                match fonts.iter().any(|f| value == f) {
                    true => Ok(()),
                    false => Err(RudocxStyleError::FontNotInstalled(value.to_owned())),
                }
            }
        }
    }
}
