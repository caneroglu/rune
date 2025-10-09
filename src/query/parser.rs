//! RQL (Rune Query Language) parser implementation
//! 
//! This module handles the parsing of RQL commands using the Pest parser generator.

use anyhow::{bail};
use pest::Parser;
use pest_derive::Parser;
use tracing::{debug, info};
use crate::{core::error::RuneError, query::commands::{Komut, ParserFlags}};



#[derive(Parser)]
#[grammar = "query/rql.pest"]
pub struct RQLParser;

impl RQLParser {
    /// Parse a single command from a pest Pair
    pub fn parse_command(ikili: pest::iterators::Pair<Rule>) -> Option<Komut> {
        match ikili.as_rule() {
/*             Rule::upsert_cmd => {
                let mut inner = ikili.into_inner();
                inner.next(); // skip operator - zaten biliyoruz.
                let db = inner.next()?.as_str().to_string();
                inner.next(); // skip access mode - upsert'te search alakasız.
                let key = inner.next()?.as_str().to_string();
                let value = inner.next()?.as_str().to_string();
                let flags= inner.next()?.as_str().to_string();
                let flags = if flags.is_empty() {
                    Option::None
                } else {
                    Option::Some(flags)
                };

                Some(Komut::Upsert { db, key, value, flags })
            } */
           
           Rule::upsert_cmd => {
            let mut db = String::new();
            let mut key = String::new();
            let mut value = String::new();
            let mut flags: Option<Vec<ParserFlags>> = None;

            // upsert_cmd = op_upsert ~ db_name ~ exact_access ~ key ~ "=" ~ value ~ flags
            for p in ikili.into_inner() {
                match p.as_rule() {
                    Rule::db_name => db = p.as_str().to_string(),
                    Rule::key     => key = p.as_str().to_string(),
                    Rule::value   => value = p.as_str().to_string(),
                    Rule::flags if !p.as_str().is_empty() => flags = Some(p.as_str()[1..p.as_str().len()-1].split(",").map(|el| {
                        match &el.to_ascii_lowercase().chars().filter(|ch| !ch.is_whitespace()).collect::<String>()[..] {
                            "nx" => ParserFlags::NX,
                            "xx" => ParserFlags::XX,
                            val if val.starts_with("ttl=") => {
                                 match u32::from_str_radix(&val[val.find("=").unwrap() + 1..], 10) {
                                    Ok(parsed_ttl_val) => {
                                        ParserFlags::TTL(parsed_ttl_val)},
                                    Err(_) => ParserFlags::None,
                                }
                            }
                            _ => ParserFlags::None 
                        }
                    }).collect()),
                    // Bunlar yapısal/süs: operator, exact_access, "=" – görmezden gel
                    Rule::op_upsert | Rule::exact_access => {}
                    _ => {}
                }
            }
            return Some(Komut::Upsert { db, key, value, flags });
           },
            Rule::read_cmd => {
                let mut db = String::new();
                let mut exact = false;
                let mut key = String::new();

                for p in ikili.into_inner() {
                    match p.as_rule() {
                        Rule::db_name => db = p.as_str().to_string(),
                        Rule::access_mode => {
                            // access_mode -> exact_access | radix_access
                            let inner = p.into_inner().next();
                            exact = matches!(inner.map(|x| x.as_rule()), Some(Rule::exact_access));
                        }
                        Rule::key => key = p.as_str().to_string(),
                        _ => {}
                    }
                }
                return Some(Komut::Read { db, key, exact });
            },
            Rule::delete_cmd => {
                let mut db = String::new();
                let mut exact = false;
                let mut key = String::new();

                for p in ikili.into_inner() {
                    match p.as_rule() {
                        Rule::db_name => db = p.as_str().to_string(),
                        Rule::access_mode => {
                            let inner = p.into_inner().next();
                            exact = matches!(inner.map(|x| x.as_rule()), Some(Rule::exact_access));
                        }
                        Rule::key => key = p.as_str().to_string(),
                        _ => {}
                    }
                }
                return Some(Komut::Delete { db, key, exact });
            },
            Rule::rename_cmd => {
                let mut db = String::new();
                let mut old_key = String::new();
                let mut new_key = String::new();
                // (istersen burada [nx] gibi rename flag'lerini de yakalayabilirsin)

                for p in ikili.into_inner() {
                    match p.as_rule() {
                        Rule::db_name => db = p.as_str().to_string(),
                        Rule::key => {
                            if old_key.is_empty() { old_key = p.as_str().to_string(); }
                            else { new_key = p.as_str().to_string(); }
                        }
                        _ => {}
                    }
                }
                return Some(Komut::Rename { db, old_key, new_key });
            }
            _ => None
        }
    }



    /// Parse a complete RQL query string
    pub fn parse_query_string(query: &str) -> Result<Komut, RuneError> {
        match Self::parse(Rule::program, query) {
            Ok(mut pairs) => {
                for pair in pairs {
                    // 'program' atomik ifadesini açtık (pest.rs)
                    // 'statement' atomik ifadesini de açalım,
                    for statement in pair.into_inner() {
                        // ya statement = 'pipeline' olacak ya da 'komut'. pipeline şu anda gereksiz, ileriye yönelik ekledim.
                        if statement.as_rule() == Rule::statement {
                            // 'komut' ifadesini açalım,
                            for komut in statement.into_inner() {
                                for komut_adi in komut.into_inner() {
                                    if let Some(parsed_command) = Self::parse_command(komut_adi) {
                                        debug!("dbg1: {:?}", parsed_command);
                                        return Ok(parsed_command)
                                    } else {
                                        return Err(RuneError::QuerySyntaxError);
                                    }
                                }
                            }
                        }
                    }
                }
                Err(RuneError::QuerySyntaxError)
            }
            Err(_) => Err(RuneError::QuerySyntaxError)
        }
    }
}
