use crate::{error::ParseError, reader::ByteReader, ParseResult};

use super::{chunk_part::FChunkPart, file_manifest::FFileManifest, shared::{FSHAHash, MD5_DIGEST_SIZE, SHA256_DIGEST_SIZE}};


#[derive(Debug, Clone)]
pub struct FFileManifestList {
    pub(crate) _version: u8,
    pub(crate) _size:u32,
    pub(crate) _count: u32,
    pub(crate) entries: Vec<FFileManifest>
}

impl FFileManifestList {
    pub fn parse(reader: &mut ByteReader) -> ParseResult<FFileManifestList> {
        let reader_start = reader.tell();

        let size = reader.read()?;
        let version = reader.read()?;
        let count = reader.read()?;

        let mut entries:Vec<FFileManifest> = vec![Default::default(); count as usize];

        for entry in entries.iter_mut() {
            entry.filename = reader.read()?;
        }

        for entry in entries.iter_mut() {
            entry.syslink_target = reader.read()?;
        }

        for entry in entries.iter_mut() {
            entry.hash = FSHAHash::from_byte_reader(reader)?;
        }

        for entry in entries.iter_mut() {
            entry.flags = reader.read()?;
        }

        for entry in entries.iter_mut() {
            entry.install_tags = reader.read_array(|reader|  reader.read())?;
        }

         for entry in entries.iter_mut() {
             let part_count = reader.read::<u32>()?;
             let mut file_offset = 0;

             //make sure we have enough capacity to push every parts without reallocating
             entry.chunk_parts.reserve(part_count as usize - entry.chunk_parts.capacity());
             for _ in 0..part_count {
                let part = FChunkPart::parse(reader, file_offset)?;
                file_offset += part.size() as usize;
                 entry.chunk_parts.push(part);
             }
         }

         if version >= 1 {
            for entry in entries.iter_mut() {
                let has_md5 = reader.read::<u32>()?;
                if has_md5 != 0 {
                    entry.hash_md5 = reader.read_bytes(MD5_DIGEST_SIZE)?.try_into().ok();
                }
            }

            for entry in entries.iter_mut() {
                entry.mime_type = reader.read().ok();
            }
         }

         if version >= 2 {
            for entry in entries.iter_mut() {
                entry.hash_sha256 = reader.read_bytes(SHA256_DIGEST_SIZE)?.try_into().ok();
            }
         }

        for entry in entries.iter_mut() {
            entry.file_size = entry.chunk_parts.iter().map(|part| part.size()).sum();
        }

        if reader_start + size as usize != reader.tell() {
            println!("FileManifestList size mismatch: expected {} but got {}\nFileManifestList version : {}", size, reader.tell() - reader_start, version);
            return Err(ParseError::InvalidData);
        }

        Ok(FFileManifestList {
            _version: version,
            _size: size,
            _count: count,
            entries
        })
    }

    pub fn entries(&self) -> &Vec<FFileManifest> {
        &self.entries
    }
}