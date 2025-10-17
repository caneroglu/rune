//! Command execution engine for RQL commands
//!
//! This module handles the execution of parsed RQL commands.

use anyhow::{bail, Ok};
use std::path::PathBuf;
use tracing::{debug, error, info};

use crate::{
    core::{error::RuneError, storage::*},
    query::commands::{Komut, ParserFlags},
};

pub struct CommandExecutor;

impl CommandExecutor {
    // Result<ParseResult, anyhow::Error>
    /// Execute a single RQL command
    pub fn execute_query(command: Komut)   {

        info!("Execute_query_command called: {:?}", command);

        match &command {
            v @ Komut::Upsert {
                        db_name,
                        db_path,
                        key,
                        val,
                        flags,
                    } => {
                        debug!("upsert_executed: {:?}",*v )
                    },
            Komut::Delete { db_name, db_path, key, exact } => todo!(),
            Komut::Read { db_name, db_path, key, exact } => todo!(),
            Komut::Rename { db_name, db_path, old_key, new_key } => todo!(),
                    }

                }
            }


/*     fn execute_upsert(object: Komut) -> Result<ParseResult, anyhow::Error> {

        Komut::new_upsert_cmd(db_name, key, val, flags)

        match Self::check_if_file_exist(db_name) {
            Ok(db_path) => {
                // ! 'Some(val)' yerine başka ve daha iyi bir sorgu ve kontrol sağla.
                // TODO: 'prev_off' hesapla.
                let appending_record = Record::new(key, Some(val), 00012, SchemaVersion::V1, 1);
                appending_record.append_record(db_path)
            }
            Err(e) => {
                error!("??");
                Err(e)
            }
        }

    }

    fn execute_delete(db: String, key: String, exact: bool) -> Result<usize, anyhow::Error> {
        // TODO: Implement delete logic
        todo!()
    }

    fn execute_read(db: String, key: String, exact: bool) -> Result<Vec<Record>, anyhow::Error> {
        todo!()

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

    fn execute_rename(
        db: String,
        old_key: String,
        new_key: String,
    ) -> Result<usize, anyhow::Error> {
        // TODO: Implement rename logic
        todo!()
    }

    fn check_if_file_exist(db_name: String) -> Result<PathBuf, anyhow::Error> {
        let _path = PathBuf::from(format!("./databases/{}.bin", db_name));

        if _path.try_exists()? {
            return Ok(_path);
        }
        bail!(RuneError::DatabaseNotFound { db_name })
    }

    fn parse_into_memory(db_path: PathBuf) -> Result<DbModel, anyhow::Error> {
        Ok(DbModel::try_from(db_path)?)
    }

    /// Execute multiple RQL commands in sequence
    pub fn execute_commands(commands: Vec<Komut>) {
        unimplemented!("Sonra.");
    }
}*/