use std::fmt::Display;
use bincode::{Decode, Encode};
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
    pub uuid: Uuid,
    pub column: String,
    pub value: String,
}

impl Display for DataModel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ts = self.timestamp.format("%Y%m%d%H%M%S%9f").to_string();
        // main.20250704102757099969.1210695b-1148-43b5-8419-ebc8f4fb72dc.dene-key
        // Since patricia tree works with *prefixes* we focus mainly working with Strings.
        write!(f, "{}.{}.{}.{}.{}", self.table_name, ts, self.uuid, self.column, self.value)
    }
}

// Since, we gonna load and save keys into bincode - it's smart to work with String instead of static &str.
impl DataModel {
    pub fn new(table_name: String, timestamp: DateTime<Utc>, uuid: Uuid, key_name: String,value: String) -> DataModel {

        let ts = timestamp.format("%Y%m%d%H%M%S%9f").to_string();

        Self {
            table_name,
            timestamp,
            uuid,
            column: key_name,
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