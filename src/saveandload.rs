use std::fs::File;
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
    fn encode_and_save(&self, path: String) -> ();
    fn load_and_decode(path: String) -> Option<Self> where Self: Sized;
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

   fn encode_and_save(&self, path: String) -> () {
        let bincode_cfg = config::standard();
        let mut dosya = File::create(path).unwrap();
        let c = encode_into_std_write(self, &mut dosya, config::standard()).unwrap();
    }

    fn load_and_decode(path: String) -> Option<Self> {
        let bincode_cfg = config::standard();
        let mut dosya = File::open(path).unwrap();
        let sonuc: Self = decode_from_std_read(&mut dosya,bincode_cfg).unwrap();
        Some(sonuc)
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

    fn encode_and_save(&self, path: String) -> () {
        let bincode_cfg = config::standard();
        let mut dosya = File::create(path).unwrap();
        let c = encode_into_std_write(self, &mut dosya, config::standard()).unwrap();
    }

    fn load_and_decode(path: String) -> Option<Self> {
        let bincode_cfg = config::standard();
        let mut dosya = File::open(path).unwrap();
        let sonuc: Self = decode_from_std_read(&mut dosya,bincode_cfg).unwrap();
        Some(sonuc)
    }
}