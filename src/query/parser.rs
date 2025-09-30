//! RQL (Rune Query Language) parser implementation
//! 
//! This module handles the parsing of RQL commands using the Pest parser generator.

use pest::Parser;
use pest_derive::Parser;
use crate::{core::error::RuneError, query::commands::Komutlar};



#[derive(Parser)]
#[grammar = "query/rql.pest"]
pub struct RQLParser;

impl RQLParser {
    /// Parse a single command from a pest Pair
    pub fn parse_command(ikili: pest::iterators::Pair<Rule>) -> Option<Komutlar> {
        match ikili.as_rule() {
            Rule::upsert_cmd => {
                let mut inner = ikili.into_inner();
                inner.next(); // skip operator - zaten biliyoruz.
                let db = inner.next()?.as_str().to_string();
                inner.next(); // skip access mode - upsert'te search alakasız.
                let key = inner.next()?.as_str().to_string();
                let value = inner.next()?.as_str().to_string();
                Some(Komutlar::Upsert { db, key, value })
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
                Some(Komutlar::Read { db, key, exact })
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
                Some(Komutlar::Delete { db, key, exact })
            },
            Rule::rename_cmd => {
                let mut inner = ikili.into_inner();
                inner.next(); // operator skip
                let db = inner.next()?.as_str().to_string();
                inner.next()?; // skip exact mode, çünkü biliyoruz.
                let old_key = inner.next()?.as_str().to_string();
                let new_key = inner.next()?.as_str().to_string();
                Some(Komutlar::Rename { db, old_key, new_key })
            }
            _ => None
        }
    }



    /// Parse a complete RQL query string
    pub fn parse_query_string(query: &str) -> Result<Vec<Komutlar>, RuneError> {
        let mut commands = Vec::new();
        
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
                                        commands.push(parsed_command);
                                    } else {
                                        return Err(RuneError::QuerySyntaxError { message:"Hata parser.rs/90".to_owned() });
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(commands)
            }
            Err(_) => Err(RuneError::IndexError { message: "More ERROR Implementation needed!".to_owned() })
        }
    }
}
