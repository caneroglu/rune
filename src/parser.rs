use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}};

use chrono::Utc;
use patricia_tree::PatriciaMap;
use pest::Parser;
use pest_derive::Parser;
use uuid::Uuid;

use crate::datamodel::{DataMethods, DataModel, ExternalDataModel, InternalDataModel};


#[derive(Parser)]
#[grammar = "rql.pest"]
pub struct RQLParser;

//TODO
// * PatriciaMap, adı üstünde MAP. k/v olarak veri eklenir/silinir.
// * DB k/v == PatriciaMap k/v
// ! DB'nin kendisinin load/save ayrı. PatriciaMap sadece k/v olmalı. iterasyon yapacaksın.
// ! zaten, DB'nin adı için patricia yürütmene gerek yok. K/V için yeterli.

#[derive(Debug)]
pub enum Komutlar {
    Upsert { db: String, key: String, value: String },
    Delete { db: String, key: String, exact: bool },
    Read { db: String, key: String, exact: bool },
    Rename { db: String, old_key: String, new_key: String},
}

impl RQLParser {
    pub fn parse_command(ikili: pest::iterators::Pair<Rule>) -> Option<Komutlar> {

        match ikili.as_rule() {
            Rule::upsert_cmd => {
                let mut inner = ikili.into_inner();
                inner.next(); // skip operator - zaten biliyoruz.
                let db = inner.next()?.as_str().to_string();
                inner.next(); // skip access mode - upsert'te search alakasız.
                let key = inner.next()?.as_str().to_string();
                let value = inner.next()?.as_str().to_string();
                return Some(Komutlar::Upsert { db, key, value })
            },
            Rule::read_cmd => {
                let mut inner = ikili.into_inner();
                inner.next(); // skip operator - biliyoruz.
                let db = inner.next()?.as_str().to_string();
                let access_mode = inner.next()?;
                // access_mode : '.' | ':' true false olarak yazabiliriz.
                let exact = access_mode.into_inner()
                    .next()
                    .map(|a| matches!(a.as_rule(), Rule::exact_access))
                    .unwrap_or(false);
                 let key = inner.next()?.as_str().to_string();
                Some(Komutlar::Read {db,key,exact})
            },

            Rule::delete_cmd => {
                let mut inner = ikili.into_inner();
                inner.next();
                let db = inner.next()?.as_str().to_string();
                let access_mode = inner.next()?;
                // access_mode : '.' | ':' true false olarak yazabiliriz.
                let exact = access_mode.into_inner()
                    .next()
                    .map(|a| matches!(a.as_rule(), Rule::exact_access))
                    .unwrap_or(false);
                let key = inner.next()?.as_str().to_string();

                if !exact && key.ends_with('*') {
                    let prefix = key.trim_end_matches('*');

                    println!("Silinecek önekler: {}", prefix);
                }
                Some(Komutlar::Delete { db,key,exact })
            },
            Rule::rename_cmd => {
                let mut inner = ikili.into_inner();
                inner.next(); // operator skip
                let db = inner.next()?.as_str().to_string();
                inner.next()?; // skip exact mode, çünkü biliyoruz.

                let old_key = inner.next()?.as_str().to_string();
                let new_key = inner.next()?.as_str().to_string();
                Some(Komutlar::Rename { db,old_key,new_key})
            }
            _ => return None
        }
    }

   pub fn parse_query(query: String) {
       if let Ok(mut ikililer) = Self::parse(Rule::program,&query) { // TODO: ERROR HANDLING CHECK SYNTAX!
           for ikili in ikililer {
               // 'program' atomik ifadesini açtık (pest.rs)
               // 'statement' atomik ifadesini de açalım,
               for statement in ikili.into_inner() {
                   // ya statement = 'pipeline' olacak ya da 'komut'. pipeline şu anda gereksiz, ileriye yönelik ekledim.
                   if statement.as_rule() == Rule::statement {
                       // 'komut' ifadesini açalım,
                       for komut in statement.into_inner() {
                           for komut_adi in komut.into_inner() {
                               if let Some(hangi_komut) = Self::parse_command(komut_adi) {
                                 println!("\n QUERY: {:?}", hangi_komut);
                                match hangi_komut {
                                   
                                   // bin save load + encode/decode - ayrı ayrı.
                                    Komutlar::Upsert { db, key, value } => {
                                        Self::check_file_if_exist(db.clone());
 
                                        let _datamodel = DataModel::new(ExternalDataModel::new(key, 
                                            InternalDataModel::new(Utc::now(), Some(value))),db);
                                        println!("{:?}",_datamodel);

                                            _datamodel.save_db();

                                    },
                                    Komutlar::Delete { db, key, exact } => {
                                         Self::check_file_if_exist(db.clone());
                                    },
                                    Komutlar::Read { db, key, exact } => {
                                        Self::check_file_if_exist(db.clone());

                                        //let mut patricia_map = PatriciaMap::new();


                                        let _datamodel = DataModel::new(ExternalDataModel::new(key.clone(), 
                                            InternalDataModel::new(Utc::now(), None)),db);
                                        println!("Placeholder - datamodel{:?} \n",_datamodel);
                                        let loaded_b = _datamodel.load_db().unwrap();
                                    // ! SÜREKLİ FİLE RELOAD yapma! CACHE EKLE! Bir kere yükle - read için.
                                    // TODO: sorgulanan key'in değerini CLI'ye yansıt.
                                    // ! "*" için belki custom READ?
                                    // !! PATRICIA'YI IMPLEMENT ET
                                    // !!! hashmap'a gerek var mı? direkt PATRICIA'nın hashmap'ına ekleriz?

                                        let mut p_map = PatriciaMap::new();
                                        
                                        loaded_b.iter().for_each(|(k,v)| {
                                            p_map.insert(k, v);
                                        });

                                        println!("Loaded patricia_map: {:?} \n", p_map);

                                        if exact {
                                            println!("exact_key_search: {:?}", p_map.get(key))                                            
                                        } else {

                                            let _test: HashMap<String, InternalDataModel> = p_map.iter_prefix(key.as_bytes()).map(|(k, &v)| { 
                                                let s = String::from_utf8(k).unwrap();
                                                (s,v.clone())
                                            }).collect();
                                            println!("radix_search: {:?}", _test )
                                        }



 
                                    },
                                    Komutlar::Rename { db, old_key, new_key } => {
                                         Self::check_file_if_exist(db.clone());
                                    },
                                }
                                   
                               } else {
                                    println!("QUERY PARSE ERROR!")
                               }
                               
                           }
                       }
                   }
               }
           }
       } else {
            println!("\n QUERY PARSE ERROR!")
       }
   }

   fn check_file_if_exist(db_name: String) {
       println!("\nDB_NAME: {}", db_name)
   }
}

 