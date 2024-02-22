use crate::{error::ParseError, manifest::shared::FGuid, reader::ByteReader, ParseResult};

use super::{chunk_info::FChunkInfo, shared::EFeatureLevel};


#[derive(Debug, Clone, serde::Serialize)]
pub struct FChunkList {
    _manifest_version:EFeatureLevel,
    _size: u32,
    _version: u8,
    chunks: Vec<FChunkInfo>
}

impl FChunkList {
    /// This function is used to parse FChunkInfos from a ByteReader
    pub fn parse(reader: &mut ByteReader, manifest_version:EFeatureLevel) -> ParseResult<FChunkList> {
        let reader_start = reader.tell();

        let size = reader.read()?;
        let version = reader.read()?;
        let count:u32 = reader.read()?;

        let mut chunks:Vec<FChunkInfo> = vec![Default::default(); count as usize];

        for chunk in chunks.iter_mut() {
            chunk.guid = reader.read()?;
        }

        for chunk in chunks.iter_mut() {
            chunk.hash = reader.read()?;
        }

        for chunk in chunks.iter_mut() {
            chunk.sha_hash = reader.read()?;
        }

        for chunk in chunks.iter_mut() {
            chunk.group_num = reader.read()?;
        }

        for chunk in chunks.iter_mut() {
            chunk.uncompressed_size = reader.read()?;
        }

        for chunk in chunks.iter_mut() {
            chunk.compressed_size = reader.read()?;
        }

        if reader_start + size as usize != reader.tell() {
            println!("Chunk header size mismatch: expected {} but got {}\nChunkHeader version : {}", size, reader.tell() - reader_start, version);
            return Err(ParseError::InvalidData);
        }

        Ok(FChunkList {
            _manifest_version: manifest_version,
            _size: size,
            _version: version,
            chunks
        })
    }

    pub fn find_by_guid(&self, guid:&FGuid) -> Option<&FChunkInfo> {
        self.chunks.iter().find(|chunk| chunk.guid() == guid)
    }

    pub fn chunks(&self) -> &Vec<FChunkInfo> {
        &self.chunks
    }
}