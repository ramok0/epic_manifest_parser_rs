use std::collections::HashMap;

use crate::{error::ParseError, reader::ByteReader, ParseResult};


#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct FCustomFields {
    _size:u32,
    _version:u8,
    pub fields:HashMap<String, String>
}

impl FCustomFields {
    /// This function is used to parse Custom Fields from a ByteReader
    pub fn parse(reader:&mut ByteReader) -> ParseResult<FCustomFields> {
        let start = reader.tell();

        let size = reader.read()?;
        let version = reader.read()?;
        let count = reader.read()?;

        let mut fields = HashMap::new();
        fields.reserve(count as usize);

        for _ in 0..count {
            let key = reader.read()?;
            let value = reader.read()?;

            fields.insert(key, value);
        }

        if start + size as usize != reader.tell() {
            println!("CustomFields size mismatch: expected {} but got {}", size, reader.tell() - start);
            return Err(ParseError::SizeMismatch);
        }

        Ok(FCustomFields {
            _size: size,
            _version: version,
            fields
        })
    }
}