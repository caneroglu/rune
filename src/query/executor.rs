//! Command execution engine for RQL commands
//! 
//! This module handles the execution of parsed RQL commands.

use std::collections::HashMap;
use chrono::Utc;
use patricia_tree::PatriciaMap;

use crate::query::commands::Komutlar;
use crate::datamodel::{DataMethods, DataModel, ExternalDataModel, InternalDataModel};

pub struct CommandExecutor;

impl CommandExecutor {
    /// Execute a single RQL command
    pub fn execute_command(command: Komutlar) {
        println!("\n QUERY: {:?}", command);
        
        match command {
            Komutlar::Upsert { db, key, value } => {
                Self::execute_upsert(db, key, value);
            },
            Komutlar::Delete { db, key, exact } => {
                Self::execute_delete(db, key, exact);
            },
            Komutlar::Read { db, key, exact } => {
                Self::execute_read(db, key, exact);
            },
            Komutlar::Rename { db, old_key, new_key } => {
                Self::execute_rename(db, old_key, new_key);
            },
        }
    }

    /// Execute multiple RQL commands in sequence
    pub fn execute_commands(commands: Vec<Komutlar>) {
        for command in commands {
            Self::execute_command(command);
        }
    }

    fn execute_upsert(db: String, key: String, value: String) {
        Self::check_file_if_exist(db.clone());
        
        let datamodel = DataModel::new(
            ExternalDataModel::new(
                key, 
                InternalDataModel::new(Utc::now(), Some(value))
            ), 
            db
        );
        println!("{:?}", datamodel);
        datamodel.save_db();
    }

    fn execute_delete(db: String, key: String, exact: bool) {
        Self::check_file_if_exist(db.clone());
        // TODO: Implement delete logic
    }

    fn execute_read(db: String, key: String, exact: bool) {
        Self::check_file_if_exist(db.clone());

        let datamodel = DataModel::new(
            ExternalDataModel::new(
                key.clone(), 
                InternalDataModel::new(Utc::now(), None)
            ), 
            db
        );
        println!("Placeholder - datamodel{:?} \n", datamodel);
        
        let loaded_b = datamodel.load_db().unwrap();
        // ! SÜREKLİ FİLE RELOAD yapma! CACHE EKLE! Bir kere yükle - read için.
        // TODO: sorgulanan key'in değerini CLI'ye yansıt.
        // ! "*" için belki custom READ?
        // !! PATRICIA'YI IMPLEMENT ET
        // !!! hashmap'a gerek var mı? direkt PATRICIA'nın hashmap'ına ekleriz?

        let mut p_map = PatriciaMap::new();
        
        loaded_b.iter().for_each(|(k, v)| {
            p_map.insert(k, v);
        });

        println!("Loaded patricia_map: {:?} \n", p_map);

        if exact {
            println!("exact_key_search: {:?}", p_map.get(&key))                                            
        } else {
            let _test: HashMap<String, InternalDataModel> = p_map
                .iter_prefix(key.as_bytes())
                .map(|(k, &v)| { 
                    let s = String::from_utf8(k).unwrap();
                    (s, v.clone())
                })
                .collect();
            println!("radix_search: {:?}", _test)
        }
    }

    fn execute_rename(db: String, old_key: String, new_key: String) {
        Self::check_file_if_exist(db.clone());
        // TODO: Implement rename logic
    }

    fn check_file_if_exist(db_name: String) {
        println!("\nDB_NAME: {}", db_name)
    }
}
