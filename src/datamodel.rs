use std::{fmt::Display, fs::{self, File, OpenOptions}, path::PathBuf};
use bincode::{config, decode_from_std_read, encode_into_std_write, Decode, Encode};
use chrono::{Date, DateTime, NaiveDateTime, Utc};
use rs_merkle::{Hasher};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest, digest::FixedOutput};
use uuid::{Timestamp, Uuid};

// column - bir nevi SQL column name.
#[derive(Default,Debug,Clone,Serialize,Deserialize )]
pub struct DataModel {
    pub table_name: String,
    pub timestamp: DateTime<Utc>, // UNIX timestamp - with padding. - added with this type in case of inserting custom time.
    //pub uuid: Uuid, Şu an gereksiz. Tek kaynaktan yazıyoruz, ilerde belki.
    pub key: String,
    pub value: Option<String>,
}

impl Display for DataModel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ts = self.timestamp.format("%Y%m%d%H%M%S%9f").to_string();
        // main.20250704102757099969.1210695b-1148-43b5-8419-ebc8f4fb72dc.dene-key
        // Since patricia tree works with *prefixes* we focus mainly working with Strings.

        let s = if let Some(g) = &self.value  {
            g
        } else {
            &"None".to_owned()
        };
        write!(f, "{}.{}.{}.{}", self.table_name, ts, self.key, s)
    }
}

// Since, we gonna load and save keys into bincode - it's smart to work with String instead of static &str.
impl DataModel {
    pub fn new(table_name: String, timestamp: DateTime<Utc>, key: String, value: Option<String>) -> DataModel {

        let ts = timestamp.format("%Y%m%d%H%M%S%9f").to_string();
 
        Self {
            table_name,
            timestamp,
            key,
            value,
        }
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

#[derive(Debug,Clone,Encode,Decode)]
pub struct EncodeDecodeDataModel {
    #[bincode(with_serde)]
    pub data: DataModel,
}

pub trait EncodeDecodeDataMethods {
    fn encode(&self) -> Vec<u8>;
    fn decode(data: Vec<u8>) -> Option<Self> where Self: Sized;
    fn save_db(&self) -> Result<(), anyhow::Error>;
    fn load_db(&self) -> Result<Self, anyhow::Error> where Self: Sized;
}

impl EncodeDecodeDataModel {
    pub fn new(data: DataModel) -> Self {
        Self { data }
    }
}

impl EncodeDecodeDataMethods for EncodeDecodeDataModel {
    fn encode(&self) -> Vec<u8> {
        let bincode_cfg = config::standard();
        bincode::encode_to_vec(self,bincode_cfg).unwrap()
    }

   fn decode(data: Vec<u8>) -> Option<Self> {
        let bincode_cfg = config::standard();
        let result: (EncodeDecodeDataModel,usize) = bincode::decode_from_slice(data.as_slice(),bincode_cfg).unwrap();
        Some(result.0)
    }

   fn save_db(&self) -> Result<(), anyhow::Error> {

        let bincode_cfg: config::Configuration = config::standard();
        let db_path = PathBuf::from(format!("./databases/{}.bin",self.data.table_name));

        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // create zaten UPSERT ayrı ayrı check etmeye gerek yok.
        let mut dosya = File::create(db_path)?;
        let result_bytes = encode_into_std_write(self, &mut dosya, bincode_cfg)?;
        println!("Dosya yazıldı: {} bayt. \n İçerik: {:?}", result_bytes, &self);

        Ok(())
    }

    fn load_db(&self) -> Result<Self, anyhow::Error> {
        let bincode_cfg = config::standard();

        let db_path = PathBuf::from(format!("./databases/{}.bin",self.data.table_name));
        let mut dosya = File::open(db_path)?;
        let _d = decode_from_std_read(&mut dosya,bincode_cfg)?;
        println!("Dosya okundu.İçerik: {:?}", _d);
        Ok(_d)
      
    }
}
 
impl EncodeDecodeDataMethods for Vec<EncodeDecodeDataModel> {
    fn encode(&self) -> Vec<u8> {
        let bincode_cfg = config::standard();
        bincode::encode_to_vec(self,bincode_cfg).unwrap()
    }

    fn decode(data: Vec<u8>) -> Option<Self> {
        let bincode_cfg = config::standard();
        let result: (Vec<EncodeDecodeDataModel>,usize) = bincode::decode_from_slice(data.as_slice(),bincode_cfg).unwrap();
        Some(result.0)
    }

   fn save_db(&self) -> Result<(), anyhow::Error> {

        let bincode_cfg: config::Configuration = config::standard();
        let db_path = PathBuf::from(format!("./databases/{}.bin",&self.first().unwrap().data.table_name));

        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // create zaten UPSERT ayrı ayrı check etmeye gerek yok.
        let mut dosya = File::create(db_path)?;
        let result_bytes = encode_into_std_write(self, &mut dosya, bincode_cfg)?;
        Ok(())
    }

    fn load_db(&self) -> Result<Self, anyhow::Error> {
        let bincode_cfg = config::standard();
        let db_path = PathBuf::from(format!("./databases/{}.bin",&self.first().unwrap().data.table_name));

        let mut dosya = File::open(db_path)?;
        Ok(decode_from_std_read(&mut dosya,bincode_cfg)?)
      
    }
} 