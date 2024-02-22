use crate::{error::ParseError, manifest::shared::{FGuid, FSHAHash}, reader::ByteReader, ParseResult};

use super::{chunk_info::FChunkInfo, shared::EFeatureLevel};


#[derive(Debug, Clone, serde::Serialize)]
pub struct FChunkInfos {
    _manifest_version:EFeatureLevel,
    _size: u32,
    _version: u8,
    chunks: Vec<FChunkInfo>
}

impl FChunkInfos {
    /// This function is used to parse FChunkInfos from a ByteReader
    pub fn parse(data: &mut ByteReader, manifest_version:EFeatureLevel) -> ParseResult<FChunkInfos> {
        let reader_start = data.tell();

        let size = data.read()?;
        let version = data.read()?;
        let count:u32 = data.read()?;

        let mut chunks:Vec<FChunkInfo> = vec![Default::default(); count as usize];

        for chunk in chunks.iter_mut() {
            chunk.guid = FGuid::from_byte_reader(data)?;
        }

        for chunk in chunks.iter_mut() {
            chunk.hash = data.read()?;
        }

        for chunk in chunks.iter_mut() {
            chunk.sha_hash = FSHAHash::from_byte_reader(data)?;
        }

        for chunk in chunks.iter_mut() {
            chunk.group_num = data.read()?;
        }

        for chunk in chunks.iter_mut() {
            chunk.uncompressed_size = data.read()?;
        }

        for chunk in chunks.iter_mut() {
            chunk.compressed_size = data.read()?;
        }

        if reader_start + size as usize != data.tell() {
            println!("Chunk header size mismatch: expected {} but got {}\nChunkHeader version : {}", size, data.tell() - reader_start, version);
            return Err(ParseError::InvalidData);
        }

        Ok(FChunkInfos {
            _manifest_version: manifest_version,
            _size: size,
            _version: version,
            chunks
        })
    }

    pub fn chunks(&self) -> &Vec<FChunkInfo> {
        &self.chunks
    }
}