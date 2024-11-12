pub mod manifest;
pub mod reader;
pub mod error;
pub mod helper;

pub type ParseResult<T> = Result<T, error::ParseError>;