use std::{fs::File, io::{Read, Seek, Write}};
use flate2::read::ZlibDecoder;

use epic_manifest_parser_rs::{error::ParseError, manifest::{chunks::chunk_header::FChunkHeader, shared::EFeatureLevel, FManifestParser}, reader::ByteReader};


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut parser = FManifestParser::new(include_bytes!("9eBVw8XMOirjbVeHl0me_LXIgQNUPg.manifest"));
    let manifest = parser.parse()?;

    println!("{:#?}", manifest);

    Ok(())
}
