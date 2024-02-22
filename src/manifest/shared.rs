use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use sha1::{Sha1, Digest};

use crate::{reader::ByteReader, ParseResult};

pub const SHA1_DIGEST_SIZE:usize = 20;
pub const MD5_DIGEST_SIZE:usize = 16;
pub const SHA256_DIGEST_SIZE:usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FGuid {
    //Private:
    a:u32,
    //	Holds the second component.
    b:u32,
    //	Holds the third component.
    c:u32,
    //    Holds the fourth component.
    d:u32
}

impl FGuid {
    pub fn from_byte_reader(reader: &mut ByteReader) -> ParseResult<FGuid> {
        Ok(FGuid {
            a: reader.read()?,
            b: reader.read()?,
            c: reader.read()?,
            d: reader.read()?
        })
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EManifestStorageFlags {
    // Stored as raw data.
    None = 0,
    // Flag for compressed data.
    Compressed = 1,
    // Flag for encrypted. If also compressed, decrypt first. Encryption will ruin compressibility.
    Encrypted = 1 << 1,
}

impl From<u8> for EManifestStorageFlags {
    fn from(value: u8) -> Self {
        match value {
            0 => EManifestStorageFlags::None,
            1 => EManifestStorageFlags::Compressed,
            2 => EManifestStorageFlags::Encrypted,
            _ => panic!("Invalid EManifestStorageFlags value")
        }
    }

}

/**
 * An enum type to describe supported features of a certain manifest.
 */
#[derive(Debug, Clone, Copy)]
pub enum EFeatureLevel {
    // The original version.
    Original,
    // Support for custom fields.
    CustomFields,
    // Started storing the version number.
    StartStoringVersion,
    // Made after data files where renamed to include the hash value, these chunks now go to ChunksV2.
    DataFileRenames,
    // Manifest stores whether build was constructed with chunk or file data.
    StoresIfChunkOrFileData,
    // Manifest stores group number for each chunk/file data for reference so that external readers don't need to know how to calculate them.
    StoresDataGroupNumbers,
    // Added support for chunk compression, these chunks now go to ChunksV3. NB: Not File Data Compression yet.
    ChunkCompressionSupport,
    // Manifest stores product prerequisites info.
    StoresPrerequisitesInfo,
    // Manifest stores chunk download sizes.
    StoresChunkFileSizes,
    // Manifest can optionally be stored using UObject serialization and compressed.
    StoredAsCompressedUClass,
    // These two features were removed and never used.
    Unused0,
    Unused1,
    // Manifest stores chunk data SHA1 hash to use in place of data compare, for faster generation.
    StoresChunkDataShaHashes,
    // Manifest stores Prerequisite Ids.
    StoresPrerequisiteIds,
    // The first minimal binary format was added. UObject classes will no longer be saved out when binary selected.
    StoredAsBinaryData,
    // Temporary level where manifest can reference chunks with dynamic window size, but did not serialize them. Chunks from here onwards are stored in ChunksV4.
    VariableSizeChunksWithoutWindowSizeChunkInfo,
    // Manifest can reference chunks with dynamic window size, and also serializes them.
    VariableSizeChunks,
    // Manifest uses a build id generated from its metadata.
    UsesRuntimeGeneratedBuildId,
    // Manifest uses a build id generated unique at build time, and stored in manifest.
    UsesBuildTimeGeneratedBuildId,

    // !! Always after the latest version entry, signifies the latest version plus 1 to allow the following Latest alias.
    LatestPlusOne,
    // An alias for the actual latest version value.
    Latest,
    // An alias to provide the latest version of a manifest supported by file data (nochunks).
    LatestNoChunks,
    // An alias to provide the latest version of a manifest supported by a json serialized format.
    LatestJson,
    // An alias to provide the first available version of optimised delta manifest saving.
    FirstOptimisedDelta,

    // More aliases, but this time for values that have been renamed
    StoresUniqueBuildId,

    // JSON manifests were stored with a version of 255 during a certain CL range due to a bug.
    // We will treat this as being StoresChunkFileSizes in code.
    BrokenJsonVersion,
    // This is for UObject default, so that we always serialize it.
    Invalid,
}

impl PartialEq for EFeatureLevel {
    fn eq(&self, other: &Self) -> bool {
        self.to_i32() == other.to_i32()
    }
}

//impl FeatureLevel => int32
impl EFeatureLevel {
    pub fn to_i32(&self) -> i32 {
        match self {
            EFeatureLevel::Original => 0,
            EFeatureLevel::CustomFields => 1,
            EFeatureLevel::StartStoringVersion => 2,
            EFeatureLevel::DataFileRenames => 3,
            EFeatureLevel::StoresIfChunkOrFileData => 4,
            EFeatureLevel::StoresDataGroupNumbers => 5,
            EFeatureLevel::ChunkCompressionSupport => 6,
            EFeatureLevel::StoresPrerequisitesInfo => 7,
            EFeatureLevel::StoresChunkFileSizes => 8,
            EFeatureLevel::StoredAsCompressedUClass => 9,
            EFeatureLevel::Unused0 => 10,
            EFeatureLevel::Unused1 => 11,
            EFeatureLevel::StoresChunkDataShaHashes => 12,
            EFeatureLevel::StoresPrerequisiteIds => 13,
            EFeatureLevel::StoredAsBinaryData => 14,
            EFeatureLevel::VariableSizeChunksWithoutWindowSizeChunkInfo => 15,
            EFeatureLevel::VariableSizeChunks => 16,
            EFeatureLevel::UsesRuntimeGeneratedBuildId => 17,
            EFeatureLevel::UsesBuildTimeGeneratedBuildId => 18,
            EFeatureLevel::LatestPlusOne => 19,
            EFeatureLevel::Latest => EFeatureLevel::LatestPlusOne.to_i32() - 1,
            EFeatureLevel::LatestNoChunks => EFeatureLevel::StoresChunkFileSizes.to_i32(),
            EFeatureLevel::LatestJson => EFeatureLevel::StoresPrerequisiteIds.to_i32(),
            EFeatureLevel::FirstOptimisedDelta =>EFeatureLevel::UsesRuntimeGeneratedBuildId.to_i32(),
            EFeatureLevel::StoresUniqueBuildId => EFeatureLevel::UsesRuntimeGeneratedBuildId.to_i32(),
            EFeatureLevel::BrokenJsonVersion => 255,
            EFeatureLevel::Invalid => -1,
        }
    }

    pub fn from_i32(value: i32) -> Option<EFeatureLevel> {
        match value {
            0 => Some(EFeatureLevel::Original),
            1 => Some(EFeatureLevel::CustomFields),
            2 => Some(EFeatureLevel::StartStoringVersion),
            3 => Some(EFeatureLevel::DataFileRenames),
            4 => Some(EFeatureLevel::StoresIfChunkOrFileData),
            5 => Some(EFeatureLevel::StoresDataGroupNumbers),
            6 => Some(EFeatureLevel::ChunkCompressionSupport),
            7 => Some(EFeatureLevel::StoresPrerequisitesInfo),
            8 => Some(EFeatureLevel::StoresChunkFileSizes),
            9 => Some(EFeatureLevel::StoredAsCompressedUClass),
            10 => Some(EFeatureLevel::Unused0),
            11 => Some(EFeatureLevel::Unused1),
            12 => Some(EFeatureLevel::StoresChunkDataShaHashes),
            13 => Some(EFeatureLevel::StoresPrerequisiteIds),
            14 => Some(EFeatureLevel::StoredAsBinaryData),
            15 => Some(EFeatureLevel::VariableSizeChunksWithoutWindowSizeChunkInfo),
            16 => Some(EFeatureLevel::VariableSizeChunks),
            17 => Some(EFeatureLevel::UsesRuntimeGeneratedBuildId),
            18 => Some(EFeatureLevel::UsesBuildTimeGeneratedBuildId),
            19 => Some(EFeatureLevel::LatestPlusOne),
            20 => Some(EFeatureLevel::Latest),
            21 => Some(EFeatureLevel::LatestNoChunks),
            22 => Some(EFeatureLevel::LatestJson),
            23 => Some(EFeatureLevel::FirstOptimisedDelta),
            24 => Some(EFeatureLevel::StoresUniqueBuildId),
            255 => Some(EFeatureLevel::BrokenJsonVersion),
            -1 => Some(EFeatureLevel::Invalid),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
pub struct FSHAHash {
    data: [u8; SHA1_DIGEST_SIZE]
}

impl Default for FSHAHash {
    fn default() -> Self {
        FSHAHash {
            data: [0; SHA1_DIGEST_SIZE]
        }
    }
}

impl PartialEq for FSHAHash {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl FSHAHash {
    pub fn new(data: [u8; SHA1_DIGEST_SIZE]) -> FSHAHash {
        FSHAHash {
            data
        }
    }

    pub fn new_from_hashable(data:impl Hash + std::convert::AsRef<[u8]>) -> FSHAHash {
        let mut hasher = Sha1::new();
        hasher.update(data);

        FSHAHash {
            data: hasher.finalize().into()
        }
    }

    pub fn from_byte_reader(reader: &mut ByteReader) -> ParseResult<FSHAHash> {

        Ok(FSHAHash {
            data: reader.read_bytes(SHA1_DIGEST_SIZE)?.try_into().map_err(|_| crate::error::ParseError::InvalidData)?
        })
    }

    pub fn data(&self) -> [u8; SHA1_DIGEST_SIZE] {
        self.data
    }

    pub fn to_hex_string(&self) -> String {
        let mut result = String::with_capacity(SHA1_DIGEST_SIZE*2);
        for byte in self.data.iter() {
            result.push_str(&format!("{:02x}", byte));
        }
        result
    }

    pub fn to_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        
        self.data.hash(&mut hasher);

        hasher.finish()
    }
}
