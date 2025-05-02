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
    Unsupported(String)
}
