use crate::{error::ParseError, reader::ByteReader, ParseResult};

use super::shared::EFeatureLevel;


#[derive(Debug, Clone)]
pub struct FManifestMeta {
    pub feature_level:EFeatureLevel,
    pub b_is_file_data:bool,
    pub app_id:u32,
    pub app_name:String,
    pub build_version:String,
    pub launch_exe:String,
    pub launch_command:String,
    pub prerequisites:Vec<String>,
    pub prereq_name:String,
    pub prereq_path:String,
    pub prereq_args:String,
    pub build_id:Option<String>,
    pub prereq_ids:Vec<String>,
    pub uninstall_action_path:Option<String>,
    pub uninstall_action_args:Option<String>,

}

//TODO : MAKE THE PARSER FOR METADATA

impl FManifestMeta {
    pub fn parse(reader:&mut ByteReader) -> ParseResult<FManifestMeta> {
        let meta_size = reader.read::<u32>()?;
        let data_version = reader.read::<u8>()?;

        let feature_level = EFeatureLevel::from_i32(reader.read()?).ok_or(ParseError::InvalidData)?;
        let b_is_file_data = reader.read::<u8>()? == 1;
        let app_id = reader.read()?;

        let app_name = reader.read()?;
        let build_version = reader.read()?;
        let launch_exe = reader.read()?;
        let launch_command = reader.read()?;

        let prereq_ids = reader.read_array(|reader| reader.read())?;
        let prereq_name = reader.read()?;
        let prereq_path = reader.read()?;
        let prereq_args = reader.read()?;

        let mut metadata = FManifestMeta {
            feature_level,
            b_is_file_data,
            app_id,
            app_name,
            build_version,
            launch_exe,
            launch_command,
            prerequisites:vec![],
            prereq_name,
            prereq_path,
            prereq_args,
            build_id:None,
            prereq_ids,
            uninstall_action_path:None,
            uninstall_action_args:None,
        };

         if data_version >= 1 {
             let build_id = reader.read().ok();
             metadata.build_id = build_id;
         }

         if data_version >= 2 {
            let uninstall_action_path = reader.read().ok();
            let uninstall_action_args = reader.read().ok();
            metadata.uninstall_action_path = uninstall_action_path;
            metadata.uninstall_action_args = uninstall_action_args;
         }

         if reader.tell() != meta_size as usize {
            println!("Metadata size mismatch, {} bytes are missing, version : {}", meta_size - reader.tell() as u32, data_version);
            return Err(ParseError::InvalidData);
         }

        Ok(metadata)
    }
}