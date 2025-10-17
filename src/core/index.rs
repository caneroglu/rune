use crate::core::{error::RuneError, storage::{bincode_cfg, Record}};
use anyhow::{bail};
use bincode::{serde::decode_from_std_read, Decode, Encode};
use chrono::Utc;
use crc32fast::Hasher;
use patricia_tree::GenericPatriciaMap;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap, fs::{self, File, OpenOptions}, io::BufReader, path::PathBuf
};
use tracing::info;



#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct RecordsTree {
        #[bincode(with_serde)]
    records: GenericPatriciaMap<String, Option<String>>
}

// load file and p-tree it.
impl RecordsTree {
    pub fn new(db_path: PathBuf) -> () {
        let mut dosya = OpenOptions::new()
            .read(true)
            .open(db_path).unwrap();
        // patricia_tree == hashmap, o sebeple problem değil.

        let mut reader = BufReader::new(dosya);
        let it = decode_from_std_read::<Vec<Record>, _, _>(&mut reader, bincode_cfg()).unwrap();
 
        println!("vec: {:?}", it)
/*  
        let p_tree = decode_from_std_read(src, config)
        let result_bytes = encode_into_std_write(&self.db_path, &mut dosya, bincode_cfg)?; */
    }
}