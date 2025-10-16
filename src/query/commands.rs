//! Command definitions for the Rune Query Language (RQL)

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Komut {

// ! Exact yok lan. Upsertte
    Upsert {
        db_name: String,
        db_path: PathBuf,
        key: String,
        val: String,
        flags: Option<Vec<ParserFlags>>,
    },
    Delete {
        db_name: String,
        db_path: PathBuf,
        key: String,
        exact: bool,
    },
    Read {
        db_name: String,
        db_path: PathBuf,
        key: String,
        exact: bool,
    },
    Rename {
        db_name: String,
        db_path: PathBuf,
        old_key: String,
        new_key: String,
    },
}

impl Komut {

    // Bunun yerine **SETTER** Tanımlayabilir miyiz?
    pub fn new_upsert_cmd(db_name: String, key: String, val: String, flags: Option<Vec<ParserFlags>>) -> Self {
        Self::Upsert { 
            db_name: db_name.clone(),
            // ! Sonra 'custom path' seçmeye imkan verirsin. Sonraki patch'te.
            db_path: PathBuf::from(format!("./databases/{}.bin", db_name)),
            key, 
            val, 
            flags, 
        }    
    }

    pub fn new_del_cmd(db_name: String, key: String, val: String, flags: Option<Vec<ParserFlags>>) -> Self {
        Self::Delete { 
            db_name: db_name.clone(),
            // ! Sonra 'custom path' seçmeye imkan verirsin. Sonraki patch'te.
            db_path: PathBuf::from(format!("./databases/{}.bin", db_name)),
            key, 
            val, 
            flags, 
        }   
    }

    pub fn new_read_cmd() -> Self {}
    pub fn new_rename_cmd() -> Self {}
}

#[derive(Debug, Clone)]
pub enum ParserFlags {
    NX,
    XX,
    TTL(u32),
    None,
}
