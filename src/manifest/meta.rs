use crate::{error::ParseError, reader::ByteReader, ParseResult};

use super::shared::EFeatureLevel;


#[derive(Debug, Clone, serde::Serialize)]
pub struct FManifestMeta {
    feature_level:EFeatureLevel,
    b_is_file_data:bool,
    app_id:u32,
    app_name:String,
    build_version:String,
    launch_exe:String,
    launch_command:String,
    prerequisites:Vec<String>,
    prereq_name:String,
    prereq_path:String,
    prereq_args:String,
    build_id:Option<String>,
    prereq_ids:Vec<String>,
    uninstall_action_path:Option<String>,
    uninstall_action_args:Option<String>,
}

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

    pub fn app_id(&self) -> u32 {
        self.app_id
    }

    pub fn app_name(&self) -> &str {
        &self.app_name
    }

    pub fn build_version(&self) -> &str {
        &self.build_version
    }

    pub fn launch_exe(&self) -> &str {
        &self.launch_exe
    }

    pub fn launch_command(&self) -> &str {
        &self.launch_command
    }

    pub fn prerequisites(&self) -> &Vec<String> {
        &self.prerequisites
    }

    pub fn prereq_name(&self) -> &str {
        &self.prereq_name
    }

    pub fn prereq_path(&self) -> &str {
        &self.prereq_path
    }

    pub fn prereq_args(&self) -> &str {
        &self.prereq_args
    }

    pub fn build_id(&self) -> Option<&String> {
        self.build_id.as_ref()
    }

    pub fn prereq_ids(&self) -> &Vec<String> {
        &self.prereq_ids
    }

    pub fn uninstall_action_path(&self) -> Option<&String> {
        self.uninstall_action_path.as_ref()
    }

    pub fn uninstall_action_args(&self) -> Option<&String> {
        self.uninstall_action_args.as_ref()
    }

    pub fn feature_level(&self) -> EFeatureLevel {
        self.feature_level
    }

    pub fn is_file_data(&self) -> bool {
        self.b_is_file_data
    }
}