use super::shared::{FGuid, FSHAHash};


#[derive(Default, Debug, Clone)]
pub struct FChunkInfo {
    pub(crate) guid: FGuid,
    pub(crate) hash: u64,
    pub(crate) sha_hash: FSHAHash,
    pub(crate) group_num:u8,
    pub(crate) uncompressed_size: u32,
    pub(crate) compressed_size: i64,
}

impl FChunkInfo {
    pub fn guid(&self) -> &FGuid {
        &self.guid
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }

    pub fn sha_hash(&self) -> &FSHAHash {
        &self.sha_hash
    }

    pub fn group_num(&self) -> u8 {
        self.group_num
    }

    pub fn uncompressed_size(&self) -> u32 {
        self.uncompressed_size
    }

    pub fn compressed_size(&self) -> i64 {
        self.compressed_size
    }
}