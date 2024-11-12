use epic_manifest_parser_rs::manifest::FManifestParser;


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut parser = FManifestParser::new(include_bytes!("9eBVw8XMOirjbVeHl0me_LXIgQNUPg.manifest"));
    let manifest = parser.parse()?;

    //println!("{:#?}", manifest);


    Ok(())
}
