// --- Underlines ---

use std::fmt;
use std::fmt::Formatter;

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

    pub fn change_value(
        &mut self,
        value: Option<UnderlineStyle>,
    ) -> crate::elements::run_properties::Result<()> {
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

///Note that it will not return the correct value if you dont follow OOXML standard capitalization
impl<T: Into<String>> From<T> for UnderlineStyle {
    fn from(v: T) -> Self {
        match v.into().as_ref() {
            "single" => UnderlineStyle::Single,
            "words" => UnderlineStyle::Words,
            "double" => UnderlineStyle::Double,
            "thick" => UnderlineStyle::Thick,
            "dotted" => UnderlineStyle::Dotted,
            "dottedHeavy" => UnderlineStyle::DottedHeavy,
            "dash" => UnderlineStyle::Dash,
            "dashedHeavy" => UnderlineStyle::DashedHeavy,
            "dashLong" => UnderlineStyle::DashLong,
            "dashLongHeavy" => UnderlineStyle::DashLongHeavy,
            "dotDash" => UnderlineStyle::DotDash,
            "dashDotHeavy" => UnderlineStyle::DashDotHeavy,
            "dotDotDash" => UnderlineStyle::DotDotDash,
            "dashDotDotHeavy" => UnderlineStyle::DashDotDotHeavy,
            "wave" => UnderlineStyle::Wave,
            "wavyHeavy" => UnderlineStyle::WavyHeavy,
            "wavyDouble" => UnderlineStyle::WavyDouble,
            _ => UnderlineStyle::Single,
        }
    }
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
