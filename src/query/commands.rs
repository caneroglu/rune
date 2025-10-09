//! Command definitions for the Rune Query Language (RQL)

#[derive(Debug, Clone)]
pub enum Komut {
    Upsert { 
        db: String, 
        key: String, 
        value: String,
        // FIXME!: ayrı ayrı parse et, çünkü birden fazla *flag* olabilir. parse ettiğin flag'lara göre, ayrı ayrı mekanizma tasarla.
        flags: Option<Vec<ParserFlags>>,
    },
    Delete { 
        db: String, 
        key: String, 
        exact: bool 
    },
    Read { 
        db: String, 
        key: String, 
        exact: bool 
    },
    Rename { 
        db: String, 
        old_key: String, 
        new_key: String
    },
}

#[derive(Debug, Clone)]
pub enum ParserFlags {
    NX,
    XX,
    TTL(u32),
    None,
}