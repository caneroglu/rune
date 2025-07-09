use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "rql.pest"]
pub struct RQLParser;


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

            // TODO
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
                                   println!("{:?}", hangi_komut);
                               }
                           }
                       }
                   }
               }
           }
       }
   }
}