// --- Colors ---

use crate::errors::RudocxStyleError;
use std::fmt;
use std::fmt::Formatter;

type Result<T> = std::result::Result<T, RudocxStyleError>;

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

impl HLColor {
    pub fn new(color: HighlightPalette) -> Self {
        Self { value: Some(color) }
    }

    pub fn value(&self) -> String {
        match &self.value {
            Some(v) => v.to_string(),
            None => String::new(),
        }
    }

    pub fn change_value(&mut self, value: Option<HighlightPalette>) -> Result<()> {
        self.value = value;
        Ok(())
    }
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

///Note that it will not return the correct value if you dont follow OOXML standard capitalization
impl<T: Into<String>> From<T> for HighlightPalette {
    fn from(color: T) -> Self {
        match color.into().as_ref() {
            "yellow" => Self::Yellow,
            "darkYellow" => Self::DarkYellow,
            "green" => Self::Green,
            "darkGreen" => Self::DarkGreen,
            "cyan" => Self::Cyan,
            "darkCyan" => Self::DarkCyan,
            "magenta" => Self::Magenta,
            "darkMagenta" => Self::DarkMagenta,
            "blue" => Self::Blue,
            "darkBlue" => Self::DarkBlue,
            "red" => Self::Red,
            "darkRed" => Self::DarkRed,
            "black" => Self::Black,
            "white" => Self::White,
            _ => Self::White,
        }
    }
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
