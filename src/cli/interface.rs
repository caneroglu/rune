use anyhow::{anyhow };
use clap::{Parser, Subcommand};
use std::{path::PathBuf, process::exit};
use tracing::{error, info};
use crate::{core::storage::ParseResult, query::executor::CommandExecutor, RQLParser};

/*
Terminal komutları:

rune create db _db_name_
rune query _query_
*/

/// Patricia Tree K/V DB için bir CLI aracı.
/// Sorgu çalıştırmak için 'rune query'.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, verbatim_doc_comment)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Sorgu dilini kullanarak bir veya daha fazla komut çalıştırır.
    Query {
        /// Çalıştırılacak sorgu metni.
        /// Örnek: "+ my_db:user.1=john ; $ my_db.user."
        #[clap(
            value_name = "QUERY_STRING",
            // file veya interactive yoksa en az 1 parça lazım
            required_unless_present_any = ["file", "interactive"],
            // bir veya daha fazla argüman topla
            num_args = 1..,
            // sonrasını opsiyon olarak yorumlama (örn. -foo gibi şeyler)
            trailing_var_arg = true,
            // '-' ile başlayan değerleri kabul et
            allow_hyphen_values = true
        )]
        query: Option<Vec<String>>,

        /// Sorguyu bir dosyadan oku ve çalıştır.
        #[clap(short, long, value_name = "FILE_PATH")]
        file: Option<PathBuf>,

        /// İnteraktif bir REPL (Read-Eval-Print Loop) oturumu başlat.
        #[clap(short, long)]
        interactive: bool,
    },
    // 'help' komutunu buraya eklemene gerek yok, clap bunu otomatik olarak yönetir.
}

impl Command {

    pub fn parse_command() -> Result<(), anyhow::Error> {
        
        let cli = Cli::parse();
        
        match cli.command {
            Self::Query {
                query,
                file,
                interactive,
            } => {

                if let Some(query_string) = query {

                    info!("CLI parse_command CALLED: {:?}", query_string);
                    info!("Query validation started...");

                    let query_concatted = query_string.join(" ");

                    match RQLParser::parse_query(query_concatted.as_str()) {

                        Ok(parsed_query) => {
                            info!("Query is CORRECT!... Calling execute_query_command...");
                             CommandExecutor::execute_query(parsed_query);
                             Ok(())
                        }

                        Err(e) => {
                            error!("Query is WRONG!: {}", e);
                            exit(1);
                        }

                    }
                    
                } else if let Some(file_path) = file {
                    print!("Debug: \n FILE_PATH: {}", file_path.to_str().unwrap());
                    todo!()
                } else if interactive {
                    print!("Debug: \n REPLY MODE");
                    todo!()
                } else {
                    return Err(anyhow!("CLI Parsing Error."));
                }

            }
        }

    }
}
