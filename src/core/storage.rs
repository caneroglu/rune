// # Data models and persistence

use crate::core::error::RuneError;
use anyhow::{bail};
use bincode::{Decode, Encode, config, decode_from_std_read, encode_into_std_write, encode_to_vec};
use chrono::Utc;
use crc32fast::Hasher;
use patricia_tree::GenericPatriciaMap;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    path::PathBuf,
};
use tracing::info;

  
/* pub trait DataMethods {
    fn encode(&self) -> Vec<u8>;
    fn decode(data: Vec<u8>) -> Option<Self>
    where
        Self: Sized;
    //fn save_db(&self) -> Result<(), anyhow::Error>;
    fn load_db(&self) -> Result<HashMap<String, InternalDataModel>, anyhow::Error>;
    fn get_size(&self) -> Result<u64, anyhow::Error>;
} */

 

 // * 
 
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Record {
    pub key: String,
    pub val: Option<String>, //None olabilir, anahtar var ama henüz değer yok. - TOMBSTONE - DEĞİL!
    key_hash: u64,
    ts: i64,           //timestamp
    pub prev_off: u64, //key'in varsa önceki dosya offseti
    key_len: usize,
    val_len: usize,
    pub schema_ver: SchemaVersion, //ilerde güncelleme olursa,
    pub tombstone: u8,             //bitmask
    crc32: u32,                    //crc hash - veri bozulma kontrolü için
}

impl Record {
    pub fn append_record(&self, path: PathBuf) -> Result<ParseResult, anyhow::Error> {
        let cfg = bincode_cfg();
        // CREATE gereksiz ÇÜNKÜ APPEND!
        let mut f = OpenOptions::new().append(true).open(&path)?;
        match encode_into_std_write(&path, &mut f, cfg) {
            Ok(written_bytes) => Ok(ParseResult::TotalWritten(written_bytes)),
            Err(e) => Err(anyhow::Error::from(e))
        }
  
    }

    pub fn new(
        key: String,
        val: Option<String>,
        prev_off: u64,
        schema_ver: SchemaVersion,
        tombstone: u8,
    ) -> Self {
        let ts = Utc::now().timestamp_millis();
        // TODO: Implement 'key_hash' from merklee tree.
        let hasher = Hasher::new();

        match schema_ver {
            SchemaVersion::V1 => {
                let key_len = key.len();
                let val_len = val.clone().unwrap().len();

                let mut _self = Self {
                    key,
                    val,
                    // ! not implemented yet!
                    key_hash: 0,
                    ts,
                    prev_off,
                    key_len,
                    // !!
                    val_len,
                    schema_ver,
                    tombstone,
                    crc32: 0,
                };
                let _self_vec = encode_to_vec(&_self, bincode_cfg()).unwrap();
                // !! 13.10.25 -> encode + u8 diziye çevirmeyi dene olmazsa direkt trait implement et veya '.as_bytes' yardımcı metodu ekle.
                let mut hasher = Hasher::new();
                hasher.update(&_self_vec);
                _self.crc32 = hasher.finalize();
                _self
            }
            _ => unimplemented!(),
        }
    }
}



#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum SchemaVersion {
    V1, // ilerde belki *v2* falan.
}

pub fn bincode_cfg() -> impl bincode::config::Config {
    bincode::config::standard()
}

pub enum ParseResult {
    Records(Vec<Record>),
    TotalWritten(usize),
}
