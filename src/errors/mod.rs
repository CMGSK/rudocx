use crate::elements::FontType;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RudocxError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Zip error: {0}")]
    ZipError(#[from] zip::result::ZipError),
    #[error("XML error: {0}")]
    XmlError(#[from] quick_xml::Error),
    #[error("XML Attribute error: {0}")]
    XmlAttributeError(#[from] quick_xml::events::attributes::AttrError),
    #[error("UTF8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("Required part not found: {0}")]
    MissingPart(String),
    #[error("Content structure mismatch: {0}")]
    LoadContentMismatch(String),
    #[error("Feature not supported: {0}")]
    Unsupported(String),
    #[error("Run property error: {0}")]
    RunPropertyError(RudocxStyleError),
}

#[derive(Error, Debug, Clone)]
pub enum RudocxStyleError {
    #[error("HEX code not valid: {0}")]
    InvalidHex(String),
    #[error("Property not set: {0}")]
    PropertyNotSet(String),
    #[error("Hint points to None value: {0} FontType is None")]
    HintPointsNone(FontType),
    #[error("System fonts could not be found")]
    SystemFontsNotFound,
    #[error("Font not installed in your system: {0}")]
    FontNotInstalled(String),
    #[error("Empty FontSet. This is discouraged to use.")]
    EmptyFontSet,
    #[error("Default font type is not modifiable. Fallbacks to Software/System/Language.")]
    DefaultHintIsUnmodifiable,
    #[error("{0}")]
    Undefined(String),
}

pub type Result<T> = std::result::Result<T, RudocxError>;
