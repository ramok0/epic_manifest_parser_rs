use crate::{reader::ByteReader, ParseResult};

pub mod header;
pub mod shared;
pub mod meta;
pub mod chunk_header;
pub mod chunk_info;
pub mod file_manifest_list;
pub mod file_manifest;
pub mod chunk_part;
pub mod custom_fields;

pub struct FManifestParser {
    pub reader: ByteReader
}

#[derive(Debug, Clone)]
pub struct FManifest {
    pub header: header::FManifestHeader,
    pub meta: meta::FManifestMeta,
    pub chunk_header: chunk_header::FChunkInfos,
    pub file_list: file_manifest_list::FFileManifestList,
    pub custom_fields: custom_fields::FCustomFields
}

impl FManifestParser {
    pub fn new(data: &[u8]) -> FManifestParser {
        FManifestParser {
            reader: ByteReader::new(data.to_vec())
        }
    }

    pub fn parse(&mut self) -> ParseResult<FManifest> {
        let (header, mut reader) = header::FManifestHeader::parse(self)?;

        let meta = meta::FManifestMeta::parse(&mut reader)?;
        let chunk_header = chunk_header::FChunkInfos::parse(&mut reader, header.version())?;
        let file_list = file_manifest_list::FFileManifestList::parse(&mut reader)?;
        let custom_fields = custom_fields::FCustomFields::parse(&mut reader)?;

        Ok(FManifest {
            header,
            meta,
            chunk_header,
            file_list,
            custom_fields
        })
    }
}