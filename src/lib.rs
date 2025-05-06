///
/// Minimal library for reading and writing `docx` files.
///
/// Docx is a format based on a zip file with a detailed internal file structure
/// defined in the OOXML standard. This library aims to provide a simplified way of
/// creating, loading and modifying those files into the rust type system.
///
pub mod elements;
pub mod errors;
pub mod rels;
pub mod xml;
pub mod zip;
