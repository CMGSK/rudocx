use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::slice::Windows;
use crate::error::RudocxStyleError;

type Result<T> = std::result::Result<T, RudocxStyleError>;

// --- Colors ---

///Represents a HEX color code, without the `#` character.
#[derive(Debug, Clone, PartialEq)]
pub struct HexColor {
    value: String,
}

impl Default for HexColor {
    fn default() -> Self {
        Self {value: String::from("FFFFFF")}
    }
}

impl HexColor {
    /// Receives a HEX color code. `#` is **NOT** required. Alpha is not supported. Wrong input defaults to Black.
    pub fn new(color: &str) -> Self {
       match check_hex(color) {
           Ok(_) => Self { value: String::from(color) },
           Err(_) => Self { value: String::from("FFFFFF") },
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
            Err(e) => Err(e)
        }
    }
}

fn check_hex(value: &str) ->  Result<()> {
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
    value: Option<HighlightPalette>
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    value: Option<UnderlineStyle>,
}

impl Default for Underline {
    fn default() -> Self {
        Self{ value: None }
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
/// > - `hint`: Font rendering hint (e.g. `default`, `eastAsia`, `cs`...).
///
#[derive(Debug, Clone, PartialEq)]
pub struct FontSet {
    ascii: Option<String>,
    hi_ansi: Option<String>,
    east_asia: Option<String>,
    cs: Option<String>,
    ascii_theme: Option<String>,
    hi_ansi_theme: Option<String>,
    east_asia_theme: Option<String>,
    cs_theme: Option<String>,
    hint: FontType,
}

impl Default for FontSet {
    fn default() -> Self {
        Self {
            ascii: Some(String::from("Arial")),
            hi_ansi: Some(String::from("Arial")),
            east_asia: None,
            cs: None,
            ascii_theme: None,
            hi_ansi_theme: None,
            east_asia_theme: None,
            cs_theme: None,
            hint: FontType::Ascii,
        }
    }
}

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
        }
    }
}

impl FontSet {
    pub fn new(value: String, r#type: FontType) -> Self {
        let mut new_font = Self::default();
        //TODO: Check font availability

        match r#type {
            FontType::Ascii => new_font.ascii = Some(value),
            FontType::HiAnsi => new_font.hi_ansi = Some(value),
            FontType::EastAsia => new_font.east_asia = Some(value),
            FontType::Cs => new_font.cs = Some(value),
            FontType::AsciiTheme => new_font.ascii_theme = Some(value),
            FontType::HiAnsiTheme => new_font.hi_ansi_theme = Some(value),
            FontType::EastAsiaTheme => new_font.east_asia_theme = Some(value),
            FontType::CsTheme => new_font.cs_theme = Some(value),
        };

        new_font
    }
    
    pub fn value(&self) -> Result<String> {
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

    pub fn set_hint_value(&mut self, value: FontType) -> Result<()> {
        self.hint = value;
        Ok(())
    }

    pub fn change_value(&mut self, value: Option<String>, r#type: FontType ) -> Result<()> {
        match r#type {
            FontType::Ascii => self.ascii = value,
            FontType::HiAnsi => self.hi_ansi = value,
            FontType::EastAsia => self.east_asia = value,
            FontType::Cs => self.cs = value,
            FontType::AsciiTheme => self.ascii_theme = value,
            FontType::HiAnsiTheme => self.hi_ansi_theme = value,
            FontType::EastAsiaTheme => self.east_asia_theme = value,
            FontType::CsTheme => self.cs_theme = value,
        };

        //TODO: Check this is not full of None
        Ok(())
    }

    fn check_font(font: &str) -> Result<()> {
        #[cfg(target_os = "linux")]
        {
            let dirs = ["/usr/share/fonts/", "/usr/local/share/fonts/", &format!("{}/.fonts", std::env::var("HOME").unwrap())];
            let fonts = dirs.iter().flat_map(|x| list_fonts(x)).collect::<Vec<String>>();
            return check_installed(font, fonts);
        }

        #[cfg(target_os = "windows")]
        {
            let fonts = list_fonts("C:\\Windows\\Fonts");
            return check_installed(font, fonts);
        }

        #[cfg(target_os = "macos")]
        {
            let dirs = ["/System/Library/Fonts", "/Library/Fonts", &format!("{}/Library/Fonts", std::env::var("HOME").unwrap())];
            let fonts = dirs.iter().flat_map(|x| list_fonts(x)).collect::<Vec<String>>();
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

        fn check_installed(value: &str, fonts: Vec<String>) -> Result<()>{
            if fonts.is_empty() {
                Err(RudocxStyleError::SystemFontsNotFound)
            } else {
                match fonts.iter().any(|f| { value == f }) {
                    true => Ok(()),
                    false => Err(RudocxStyleError::FontNotInstalled(value.to_owned())),
                }
            }
        }
    }

}


