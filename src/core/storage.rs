// # Data models and persistence

use std::{collections::HashMap, fs::{self, File, OpenOptions}, path::PathBuf};

use bincode::{config, decode_from_std_read, encode_into_std_write, Decode, Encode};
use serde::{Deserialize, Serialize};


// for Pest.rs Parsing. Sadece dosya okumak ve kaydetmek için.
// girilen QUERY'i parse etmek için.
#[derive(Default,Debug,Clone,Serialize,Deserialize,Encode,Decode)]
pub struct DataModel {
    pub data: ExternalDataModel,
    pub db: String,
}

// Dosya adı = veritabanı adı.
#[derive(Default,Debug,Clone,Serialize,Deserialize,Encode,Decode)]
pub struct InternalDataModel { 
    pub ts: i64, // UNIX timestamp  
    //pub uuid: Uuid, Şu an gereksiz. Tek kaynaktan yazıyoruz, ilerde belki.
    pub val: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize,Deserialize,Encode,Decode)]
pub struct ExternalDataModel {
    pub key: String,
    pub val: InternalDataModel,
}

pub trait DataMethods {
    fn encode(&self) -> Vec<u8>;
    fn decode(data: Vec<u8>) -> Option<Self> where Self: Sized;
    fn save_db(&self) -> Result<(), anyhow::Error>;
    fn load_db(&self) -> Result<HashMap<String, InternalDataModel>, anyhow::Error>;
}


impl DataModel {
    pub fn new(data: ExternalDataModel, db: String) -> Self {
        Self { data, db }
    }
}

impl DataMethods for DataModel {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }

    fn decode(data: Vec<u8>) -> Option<Self> where Self: Sized {
        todo!()
    }

    fn save_db(&self) -> Result<(), anyhow::Error> {

        let bincode_cfg: config::Configuration = config::standard();
        let db_path = PathBuf::from(format!("./databases/{}.bin",self.db));
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }
                // APPEND & CREATE & OPEN
                // AUTO CREATE.
                // ? Belki, CLI info ekle.
        let mut dosya = OpenOptions::new().append(true).create(true).open(db_path)?;
        let result_bytes = encode_into_std_write(&self.data, &mut dosya, bincode_cfg)?;
        println!("Dosya yazıldı: {} bayt. \n İçerik: {:?}", result_bytes, self.data);

        Ok(())

    }

    fn load_db(&self) -> Result<HashMap<String, InternalDataModel>, anyhow::Error> {

        let bincode_cfg = config::standard();
        let db_path = PathBuf::from(format!("./databases/{}.bin",self.db));
        
        let mut dosya = File::open(db_path)?;
        let mut _hmap = HashMap::new();

        // Dosya sonuna kadar oku, EOF hatası normal
        loop {
            match decode_from_std_read::<ExternalDataModel, _,_>(&mut dosya, bincode_cfg) {
                Ok(record) => {
                    _hmap.insert(record.key, record.val);
                },
                Err(_) => break, // Dosya sonu veya parse hatası - döngüden çık
            }
        }
        
        println!("Dosya okundu.İçerik: {:?}", _hmap);
        Ok(_hmap)

     }
}

// key kısmını hashmap'tan alacağız. append-only yöntemiyle.
impl ExternalDataModel {
    pub fn new(key: String, val: InternalDataModel) -> Self {
        Self { key, val}
    }
}