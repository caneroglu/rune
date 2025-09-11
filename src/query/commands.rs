//! Command definitions for the Rune Query Language (RQL)

#[derive(Debug, Clone)]
pub enum Komutlar {
    Upsert { 
        db: String, 
        key: String, 
        value: String 
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
