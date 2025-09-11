//! Rune - A Patricia Tree-based Key-Value Database
//!
//! Rune is a high-performance key-value database that uses Patricia trees 
//! for efficient prefix-based searching and storage.

pub mod datamodel;
pub mod indexing;
pub mod terminal;
pub mod parser; // Legacy - consider migrating to query module
pub mod parser_commands;

// New modular structure
pub mod core;
pub mod query;
pub mod cli;
pub mod config;
pub mod utils;

// Re-export commonly used types
pub use query::{Komutlar, RQLParser, CommandExecutor};

/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}*/
