// # Data models and persistence

use std::{collections::HashMap, fs::{self, File, OpenOptions}, path::PathBuf};
use anyhow::{bail};
use bincode::{config, decode_from_std_read, encode_into_std_write, Decode, Encode};
use chrono::Utc;
use crc32fast::Hasher;
use patricia_tree::{GenericPatriciaMap, PatriciaMap};
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::core::error::RuneError;


// for Pest.rs Parsing. Sadece dosya okumak ve kaydetmek için.
// girilen QUERY'i parse etmek için.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct DbModel {
     //tabiatı itibariyle k/v array.
    pub table: DataModel,
    pub db_path: PathBuf,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct DataModel {
    #[bincode(with_serde)]
    pub data: GenericPatriciaMap<String, InternalDataModel>,
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
    //fn save_db(&self) -> Result<(), anyhow::Error>;
    fn load_db(&self) -> Result<HashMap<String, InternalDataModel>, anyhow::Error>;
    fn get_size(&self) -> Result<u64, anyhow::Error>;
}

// ?: Direkt GenericPatriciaMap olarak kaydedip okuyabiliriz. Fakat, temiz kod için değer mi? - TEST ET!
impl TryFrom<PathBuf> for DbModel {
    type Error = RuneError;
    fn try_from(db_path: PathBuf) -> Result<Self, Self::Error> {
        let bincode_cfg = config::standard();
        let mut dosya = File::open(&db_path)?;
        let mut new_db = DbModel::new(DataModel::new(), db_path);
        // Dosya sonuna kadar oku, EOF hatası normal
        loop {
            match decode_from_std_read::<ExternalDataModel, _,_>(&mut dosya, bincode_cfg) {
                Ok(row) => {
                    new_db.table.data.insert(row.key, row.val);
                },
                Err(_) => break, // Dosya sonu veya parse hatası - döngüden çık
            }
        }
        Ok(new_db)
    }
}

impl DataMethods for DbModel {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }
    fn decode(data: Vec<u8>) -> Option<Self> where Self: Sized {
        todo!()
    }

/*     fn save_db(&self) -> Result<(), anyhow::Error> {

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

    } */


    fn load_db(&self) -> Result<HashMap<String, InternalDataModel>, anyhow::Error> {
        todo!();

/*      let bincode_cfg = config::standard();
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
        Ok(_hmap) */

     }
     fn get_size(&self) -> Result<u64, anyhow::Error> {
        match fs::metadata(&self.db_path)?.len() {
            n if n > 0 => Ok(n),
            n if n <= 0 => Err(RuneError::DiskFullError.into()),
            _ => Err(bail!("dee"))
        }
     }
}

// ---

 impl DataModel {
    pub fn new() -> Self{
        Self { data: GenericPatriciaMap::new() }
    }
}

// key kısmını hashmap'tan alacağız. append-only yöntemiyle.
impl InternalDataModel {
    pub fn new(ts: i64, val:Option<String>) -> Self {
        Self { ts, val}
    }
}

// key kısmını hashmap'tan alacağız. append-only yöntemiyle.
impl ExternalDataModel {
    pub fn new(key: String, val: InternalDataModel) -> Self {
        Self { key, val}
    }
}

impl DbModel {
    pub fn new(table: DataModel, db_path: PathBuf) -> Self {
        Self { table, db_path }
    }

    fn save_db(&self) -> Result<u64,anyhow::Error> {
        let bincode_cfg: config::Configuration = config::standard();
        info!("db_path_debug:{:?}", &self.db_path);
        if let Some(parent) = &self.db_path.parent() {
            fs::create_dir_all(parent)?;
        }
                // APPEND & CREATE & OPEN
                // AUTO CREATE.
                // ? Belki, CLI info ekle.
        let mut dosya = OpenOptions::new().append(true).create(true).open(&self.db_path)?;
        let result_bytes = encode_into_std_write(&self.db_path, &mut dosya, bincode_cfg)?;
        println!("Dosya yazıldı: {} bayt. \n İçerik: {:?}", result_bytes, self.table);
        self.get_size()
    }

    fn load_db(db_path: PathBuf) -> Result<Self, anyhow::Error> {
        Self::try_from(db_path).map_err(|e|e.into())
    }

    fn append_db(db_path: PathBuf) -> Result<(), anyhow::Error>{
        let bincode_cfg: config::Configuration = config::standard();
        info!("db_path_debug:{:?}", db_path);

        // * .DB dosyasının sonuna .APPEND et.
        let mut dosya = OpenOptions::new().append(true).open(&db_path)?;


    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Record {
    pub key: String,
    pub val: Option<String>, //None olabilir, anahtar var ama henüz değer yok. - TOMBSTONE - DEĞİL!
    key_hash: u64,
    ts: i64, //timestamp
    pub prev_off: u64, //key'in varsa önceki dosya offseti
    key_len: usize,
    val_len: usize,
    pub schema_ver: SchemaVersion, //ilerde güncelleme olursa,
    pub tombstone: u8, //bitmask
    crc32: u32 //crc hash - veri bozulma kontrolü için
}

impl Record {
    pub fn append_record(&self, path: PathBuf) -> Result<usize,anyhow::Error> {
        let cfg = bincode_cfg();
        // CREATE gereksiz ÇÜNKÜ APPEND!
        let mut f = OpenOptions::new().append(true).open(&path)?;
        encode_into_std_write(&path, &mut f, cfg).map_err(|e| e.into())
    }

    pub fn new(key: String, val: Option<String>, prev_off: u64, schema_ver: SchemaVersion, tombstone: u8) -> Self {
        let ts = Utc::now().timestamp_millis();
        // TODO: Implement 'key_hash' from merklee tree.
        let mut hasher = Hasher::new();
 
        match schema_ver {
            SchemaVersion::V1 => {

                let _self = Self {
                    key,
                    val,
                    // ! not implemented yet!
                    key_hash: 0,
                    ts,
                    prev_off,
                    key_len: key.len(),
                    // !!
                    val_len: val.unwrap().len(),
                    schema_ver,
                    tombstone,
                    crc32: 0
                };

                // !! 13.10.25 -> encode + u8 diziye çevirmeyi dene olmazsa direkt trait implement et veya '.as_bytes' yardımcı metodu ekle.
                hasher.update(&_self.encode(encoder).as_bytes);

                _self



            },
            _ => unimplemented!()
        }

    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
enum SchemaVersion {
    V1
    // ilerde belki *v2* falan.
}

 fn bincode_cfg() -> impl bincode::config::Config {
    bincode::config::standard()
 }
 