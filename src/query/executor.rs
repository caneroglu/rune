//! Command execution engine for RQL commands
//! 
//! This module handles the execution of parsed RQL commands.

use std::{collections::HashMap, path::{Path, PathBuf}};
use anyhow::{bail};
use chrono::Utc;
use patricia_tree::PatriciaMap;
use tracing::{error, info};

use crate::{core::{error::RuneError, storage::*}, query::commands::Komut};

pub struct CommandExecutor;

impl CommandExecutor {
    /// Execute a single RQL command
    pub fn execute_command(command: Komut) -> Result<usize, anyhow::Error> {
        info!("\n QUERY: {:?}", command);
        
        match command {
            Komut::Upsert { db, key, value, flags } => {
                Self::execute_upsert(db, key, value)?;
            },
            Komut::Delete { db, key, exact } => {
                Self::execute_delete(db, key, exact);}
            Komut::Read { db, key, exact } => {
                Self::execute_read(db, key, exact);
            },
            Komut::Rename { db, old_key, new_key } => {
                Self::execute_rename(db, old_key, new_key);
            },
        }
    }

    /// Execute multiple RQL commands in sequence
    pub fn execute_commands(commands: Vec<Komut>) {
        for command in commands {
            Self::execute_command(command);
        }
    }

    //FIXME: key ve val *redundant* gözüküyor. 
    fn execute_upsert(db: String, key: String, value: String) -> Result<(), anyhow::Error> {
        match Self::check_if_file_exist(db) {
            Ok(db_path) => {
                let _datamemory = Self::parse_into_memory(db_path)?;
                info!("Parsed: {:?}", _datamemory);
                Ok(())
            },
            Err(e) => {
                error!("??");
                Err(anyhow::Error::from(e))
            },
        }
        
/*         let datamodel = DataModel::new(
            ExternalDataModel::new(
                key, 
                InternalDataModel::new(Utc::now().timestamp(), Some(value))
            ), 
            db
        );
        println!("{:?}", datamodel);
        datamodel.save_db(); */
    }

    fn execute_delete(db: String, key: String, exact: bool) {
         // TODO: Implement delete logic
    }

    fn execute_read(db: String, key: String, exact: bool) {
 
 /*        let datamodel = DataModel::new(
            ExternalDataModel::new(
                key.clone(), 
                InternalDataModel::new(Utc::now().timestamp(), None)
            ), 
            db
        );
        println!("Placeholder - datamodel{:?} \n", datamodel);
        
        let loaded_b = datamodel.load_db().unwrap();
 

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
        } */
    }

    fn execute_rename(db: String, old_key: String, new_key: String) {
         // TODO: Implement rename logic
    }

    fn check_if_file_exist(db_name: String) -> Result<PathBuf, anyhow::Error> {

        let _path = PathBuf::from(format!("./databases/{}.bin",db_name));
        
        if _path.try_exists()? {
            return Ok(_path)
        }
        bail!(RuneError::DatabaseNotFound { db_name })
 
    }

    fn parse_into_memory(db_path: PathBuf) -> Result<DbModel, anyhow::Error> {
        Ok(DbModel::try_from(db_path)?)
    }
}
