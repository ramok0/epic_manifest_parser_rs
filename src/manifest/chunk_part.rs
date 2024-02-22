use crate::{error::ParseError, reader::ByteReader, ParseResult};

use super::shared::FGuid;


#[derive(Debug, Default, Clone)]
pub struct FChunkPart {
    size:u32,
    guid: FGuid,
    offset: u32,
    file_offset: usize,
}

impl FChunkPart {
    pub fn parse(reader:&mut ByteReader, file_offset:usize) -> ParseResult<FChunkPart>
    {
        let start = reader.tell();

        let struct_size = reader.read::<u32>()?;
        let guid = FGuid::from_byte_reader(reader)?;
        let offset = reader.read()?;
        let size = reader.read()?;

        if start + struct_size as usize != reader.tell() {
            println!("ChunkPart size mismatch: expected {} but got {}", struct_size, reader.tell() - start);
            return Err(ParseError::SizeMismatch);
        }

        Ok(FChunkPart {
            size,
            guid,
            offset,
            file_offset,
        })
    }

    pub fn file_offset(&self) -> usize {
        self.file_offset
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn guid(&self) -> &FGuid {
        &self.guid
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }
}