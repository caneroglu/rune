//! Legacy parser module - Consider using query::parser instead
//!
//! This module contains the legacy parsing logic. For new code, use the 
//! modularized query parsing system in the query module.

use crate::query::{RQLParser, CommandExecutor};

/// Legacy function that combines parsing and execution
/// TODO: Refactor callers to use query::parser and query::executor separately
pub fn parse_query(query: String) {
    match RQLParser::parse_query_string(&query) {
        Ok(commands) => {
            CommandExecutor::execute_commands(commands);
        }
        Err(e) => {
            println!("\n QUERY PARSE ERROR: {}", e);
        }
    }
}

 