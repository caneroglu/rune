use std::{fs::File, path::{Path, PathBuf}};
use anyhow::{anyhow, Error, Ok};
use bincode::{config, decode_from_std_read, encode_into_std_write, Decode, Encode};
use crate::datamodel::DataModel;

#[derive(Debug,Clone,Encode,Decode)]
pub struct EncodeDecodeDataModel {
    #[bincode(with_serde)]
    pub data: DataModel,
}

pub trait EncodeDecodeDataMethods {
    fn encode(&self) -> Vec<u8>;
    fn decode(data: Vec<u8>) -> Option<Self> where Self: Sized;
    fn save_db(&self, path: String) -> Result<(), anyhow::Error>;
    fn load_db(path: String) -> Result<Self, anyhow::Error> where Self: Sized;
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

 
 

   fn save_db(&self, path: String) -> Result<(), anyhow::Error> {

        let bincode_cfg: config::Configuration = config::standard();
        let db_path = PathBuf::from(format!("/databases/{}.bin",path));

        if db_path.try_exists()? {
            let mut dosya = File::create(path)?;
            let result_bytes = encode_into_std_write(self, &mut dosya, bincode_cfg)?;
        } else {
            let mut dosya = File::open(db_path)?;
            let result_bytes = encode_into_std_write(self, &mut dosya, bincode_cfg)?;
        }
        Ok(())
    }

    fn load_db(path: String) -> Result<Self, anyhow::Error> {
        let bincode_cfg = config::standard();
        let mut dosya = File::open(path)?;
        Ok(decode_from_std_read(&mut dosya,bincode_cfg)?)
      
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

   fn save_db(&self, path: String) -> Result<(), anyhow::Error> {

        let bincode_cfg: config::Configuration = config::standard();
        let db_path = PathBuf::from(format!("/databases/{}.bin",path));

        if db_path.try_exists()? {
            let mut dosya = File::create(path)?;
            let result_bytes = encode_into_std_write(self, &mut dosya, bincode_cfg)?;
        } else {
            let mut dosya = File::open(db_path)?;
            let result_bytes = encode_into_std_write(self, &mut dosya, bincode_cfg)?;
        }
        Ok(())
    }

    fn load_db(path: String) -> Result<Self, anyhow::Error> {
        let bincode_cfg = config::standard();
        let mut dosya = File::open(path)?;
        Ok(decode_from_std_read(&mut dosya,bincode_cfg)?)
      
    }
}