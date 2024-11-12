use crate::{reader::ByteReader, ParseResult};

pub mod header;
pub mod shared;
pub mod meta;
pub mod chunk_list;
pub mod chunk_info;
pub mod file_manifest_list;
pub mod file_manifest;
pub mod chunk_part;
pub mod custom_fields;
pub mod chunks;

pub struct FManifestParser {
    pub data: Vec<u8>,
    pub reader: ByteReader
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FManifest {
    pub header: header::FManifestHeader,
    pub meta: meta::FManifestMeta,
    pub chunk_list: chunk_list::FChunkList,
    pub file_list: file_manifest_list::FFileManifestList,
    pub custom_fields: custom_fields::FCustomFields,
    pub data: Vec<u8>
}

impl FManifestParser {
    pub fn new(data: &[u8]) -> FManifestParser {
        FManifestParser {
            data: data.to_vec(),
            reader: ByteReader::new(data.to_vec())
        }
    }

    pub fn parse(mut self) -> ParseResult<FManifest> {
        let (header, mut reader) = header::FManifestHeader::parse(&mut self)?;

        let meta = meta::FManifestMeta::parse(&mut reader)?;
        let chunk_header = chunk_list::FChunkList::parse(&mut reader, header.version())?;
        let file_list = file_manifest_list::FFileManifestList::parse(&mut reader)?;
        let custom_fields = custom_fields::FCustomFields::parse(&mut reader)?;

        Ok(FManifest {
            header,
            meta,
            chunk_list: chunk_header,
            file_list,
            custom_fields,
            data: self.data
        })
    }
}