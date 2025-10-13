//! Rune - A Patricia Tree-based Key-Value Database
//!
//! Rune is a high-performance key-value database that uses Patricia trees
//! for efficient prefix-based searching and storage.

pub mod utils;

pub mod cli;
pub mod config;
pub mod core;
pub mod query;

// Re-export commonly used types
pub use query::{Komut, RQLParser};

/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}*/
