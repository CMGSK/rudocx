// --- Colors ---

use crate::errors::RudocxStyleError;
use std::fmt;
use std::fmt::Formatter;

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
    pub fn change_value(&mut self, value: &str) -> crate::elements::run_properties::Result<()> {
        match check_hex(value) {
            Ok(_) => Ok(self.value = value.to_string()),
            Err(e) => Err(e),
        }
    }
}

fn check_hex(value: &str) -> crate::elements::run_properties::Result<()> {
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
