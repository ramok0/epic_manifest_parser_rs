use std::io::Read;

use crate::{error::ParseError, reader::ByteReader, ParseResult};

use super::{shared::{EFeatureLevel, EManifestStorageFlags, FSHAHash}, FManifestParser};
use flate2::read::ZlibDecoder;

pub const MANIFEST_MAGIC:u32 = 0x44BEC00C;

#[derive(Debug, Clone, serde::Serialize)]
pub struct FManifestHeader {
    magic: u32,
    header_size: u32,
    data_size_uncompressed: u32,
    data_size_compressed: u32,
    sha_hash: FSHAHash,
    stored_as: EManifestStorageFlags,
    version: EFeatureLevel,
}

impl FManifestHeader {
    pub fn parse(manifest:&mut FManifestParser) -> ParseResult<(FManifestHeader, ByteReader)> {
        manifest.reader.seek(0);
        let magic = manifest.reader.read()?;

        if magic != MANIFEST_MAGIC {
            return Err(ParseError::InvalidMagic)
        }

        let header_size = manifest.reader.read()?;
        let data_size_uncompressed = manifest.reader.read()?;
        let data_size_compressed = manifest.reader.read()?;
        let header_hash = FSHAHash::from_byte_reader(&mut manifest.reader)?;

        let stored_as = EManifestStorageFlags::try_from(manifest.reader.read::<u8>()?).map_err(|_| ParseError::InvalidStorageFlag)?;
        let version = EFeatureLevel::from_i32(manifest.reader.read()?).ok_or(ParseError::InvalidData)?;

        if header_size != manifest.reader.tell() as u32 {
            dbg!(header_size, manifest.reader.tell() as u32);
            return Err(ParseError::OffsetMismatch)
        }

        let data = manifest.reader.read_bytes(manifest.reader.length() - header_size as usize)?; //actual manifest data
        let proper_data = if stored_as == EManifestStorageFlags::Compressed {
            let mut decoder = ZlibDecoder::new(&data[..]);
            let mut buffer:Vec<u8> = Vec::with_capacity(data_size_uncompressed as usize);
            let length = decoder.read_to_end(&mut buffer).map_err(|_| ParseError::DecompressionError)?;

            if length != data_size_uncompressed as usize {
                return Err(ParseError::DecompressionError)
            }

           let in_hash = FSHAHash::new_from_hashable(&buffer[..]);
           
           if in_hash != header_hash {
               return Err(ParseError::HashMismatch)
           }


            buffer
        } else {
            data
        };

        let header = FManifestHeader {
            magic,
            header_size,
            data_size_uncompressed,
            data_size_compressed,
            sha_hash: header_hash,
            stored_as,
            version
        };

        Ok((header, ByteReader::new(proper_data)))
    }

    pub fn version(&self) -> EFeatureLevel {
        self.version
    }

    pub fn data_size_uncompressed(&self) -> u32 {
        self.data_size_uncompressed
    }

    pub fn data_size_compressed(&self) -> u32 {
        self.data_size_compressed
    }

    pub fn sha_hash(&self) -> &FSHAHash {
        &self.sha_hash
    }

    pub fn stored_as(&self) -> EManifestStorageFlags {
        self.stored_as
    }

    pub fn magic(&self) -> u32 {
        self.magic
    }

    pub fn header_size(&self) -> u32 {
        self.header_size
    }
}