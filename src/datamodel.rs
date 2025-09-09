use std::{collections::HashMap, fmt::Display, fs::{self, File, OpenOptions}, path::PathBuf};
use bincode::{config, decode_from_std_read, encode_into_std_write, Decode, Encode};
use chrono::{Date, DateTime, NaiveDateTime, Utc};
use rs_merkle::{Hasher};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest, digest::FixedOutput};
use uuid::{timestamp, Timestamp, Uuid};

// Dosya adı = veritabanı adı.
#[derive(Default,Debug,Clone,Serialize,Deserialize,Encode,Decode )]
pub struct InternalDataModel { 
    pub ts: i64, // UNIX timestamp  
    //pub uuid: Uuid, Şu an gereksiz. Tek kaynaktan yazıyoruz, ilerde belki.
    pub val: Option<String>,
}


// daha sonra, farklı veritiplerini implement edebilirim. Şu anlık enough.
impl InternalDataModel {
    pub fn new(ts: DateTime<Utc>, val: Option<String>) -> Self {
        let ts_debug = ts.format("%Y%m%d%H%M%S%9f").to_string();
        Self { ts: ts.timestamp(), val }
    }

    // DateTime'a geri dönüştürme helper'ı
    pub fn get_datetime(&self) -> DateTime<Utc> {
        DateTime::from_timestamp(self.ts, 0).unwrap_or_default()
    }
}
// Sadece file-load ve save kısmında bu objeyi kullanmamız yeterli.

#[derive(Default,Debug,Clone,Serialize,Deserialize,Encode,Decode)]
pub struct ExternalDataModel {
    pub key: String,
    pub val: InternalDataModel,
}

// for Pest.rs Parsing. Sadece dosya okumak ve kaydetmek için.
// girilen QUERY'i parse etmek için.
#[derive(Default,Debug,Clone,Serialize,Deserialize,Encode,Decode)]
pub struct DataModel {
    pub data: ExternalDataModel,
    pub db: String,
}

impl DataModel {
    pub fn new(data: ExternalDataModel, db: String) -> Self {
        Self { data, db }
    }
}

// key kısmını hashmap'tan alacağız. append-only yöntemiyle.
impl ExternalDataModel {
    pub fn new(key: String, val: InternalDataModel) -> Self {
        Self { key, val}
    }
}




#[derive(Clone)]
pub struct Sha256Algorithm {}
// Check, kyber.

impl Hasher for Sha256Algorithm {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();

        hasher.update(data);
        <[u8; 32]>::from(hasher.finalize_fixed())
    }
    
}
 
pub trait DataMethods {
    fn encode(&self) -> Vec<u8>;
    fn decode(data: Vec<u8>) -> Option<Self> where Self: Sized;
    fn save_db(&self,data_m: DataModel) -> Result<(), anyhow::Error>;
    fn load_db(data_m: DataModel) -> Result<Self, anyhow::Error> where Self: Sized;
}

impl DataMethods for ExternalDataModel {
    fn encode(&self) -> Vec<u8> {
        let bincode_cfg = config::standard();
        bincode::encode_to_vec(self,bincode_cfg).unwrap()
    }

    fn decode(data: Vec<u8>) -> Option<Self> where Self: Sized {
        let bincode_cfg = config::standard();
        let result: (ExternalDataModel,usize) = bincode::decode_from_slice(data.as_slice(),bincode_cfg).unwrap();
        Some(result.0)
    }

    fn save_db(&self, data_m: DataModel) -> Result<(), anyhow::Error> {

        // * FEATURE: CUSTOM PATH: Later.
         let bincode_cfg: config::Configuration = config::standard();
        let db_path = PathBuf::from(format!("./databases/{}.bin",data_m.db));

        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }
                // create zaten UPSERT ayrı ayrı check etmeye gerek yok.
        let mut dosya = File::create(db_path)?;
        let result_bytes = encode_into_std_write(&data_m.data, &mut dosya, bincode_cfg)?;
        println!("Dosya yazıldı: {} bayt. \n İçerik: {:?}", result_bytes, data_m.data);

        Ok(())
    }

    fn load_db(data_m: DataModel) -> Result<Self, anyhow::Error> where Self: Sized {
        let bincode_cfg = config::standard();

        let db_path = PathBuf::from(format!("./databases/{}.bin",data_m.db));
        let mut dosya = File::open(db_path)?;
        let _d = decode_from_std_read(&mut dosya,bincode_cfg)?;
        println!("Dosya okundu.İçerik: {:?}", _d);
        Ok(_d)
    }
}
 
impl DataMethods for HashMap<String, InternalDataModel> {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }

    fn decode(data: Vec<u8>) -> Option<Self> where Self: Sized {

        todo!()
    }

    fn save_db(&self, data_m: DataModel) -> Result<(), anyhow::Error> {
        // * FEATURE: CUSTOM PATH: Later.
        let bincode_cfg: config::Configuration = config::standard();
        let db_path = PathBuf::from(format!("./databases/{}.bin",data_m.db));

        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }
                // APPEND & CREATE & OPEN
        let mut dosya = OpenOptions::new().append(true).create(true).open(db_path)?;


        let result_bytes = encode_into_std_write(ExternalDataModel::new(self.k, val), &mut dosya, bincode_cfg)?;
        println!("Dosya yazıldı: {} bayt. \n İçerik: {:?}", result_bytes, data_m.data);

        Ok(())
    }

    fn load_db(data_m: DataModel) -> Result<Self, anyhow::Error> where Self: Sized {
        let bincode_cfg = config::standard();
        let db_path = PathBuf::from(format!("./databases/{}.bin",data_m.db));
        let mut dosya = File::open(db_path)?;
        let _d: Vec<ExternalDataModel> = decode_from_std_read(&mut dosya,bincode_cfg)?;

        let mut _hmap = _d.into_iter().map(|el|(el.key,el.val)).collect();
        println!("Dosya okundu.İçerik: {:?}", _hmap);
        Ok(_hmap)
    }
}

impl DataMethods for Vec<ExternalDataModel> {

    fn encode(&self) -> Vec<u8> {
        let bincode_cfg = config::standard();
        // ! Error handling
        bincode::encode_to_vec(self,bincode_cfg).unwrap()
    }

    fn decode(data: Vec<u8>) -> Option<Self> where Self: Sized {
        let bincode_cfg = config::standard();
        let result: (Vec<ExternalDataModel>,usize) = bincode::decode_from_slice(data.as_slice(),bincode_cfg).unwrap();
        Some(result.0)
    }

    fn save_db(&self, data_m: DataModel) -> Result<(), anyhow::Error> {
                // * FEATURE: CUSTOM PATH: Later.
        let bincode_cfg: config::Configuration = config::standard();
        let db_path = PathBuf::from(format!("./databases/{}.bin",data_m.db));

        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }
                // create zaten UPSERT ayrı ayrı check etmeye gerek yok.
        let mut dosya = File::create(db_path)?;
        let result_bytes = encode_into_std_write(&data_m.data, &mut dosya, bincode_cfg)?;
        println!("Dosya yazıldı: {} bayt. \n İçerik: {:?}", result_bytes, data_m.data);

        Ok(())
    }

    fn load_db(data_m: DataModel) -> Result<Self, anyhow::Error> where Self: Sized {
        todo!()
    }
}