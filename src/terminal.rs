use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
            // Eğer --file veya --interactive bayrakları kullanılmazsa bu argüman zorunludur.
            required_unless_present_any = ["file", "interactive"]
        )]
        query: Option<String>,

        /// Sorguyu bir dosyadan oku ve çalıştır.
        #[clap(short, long, value_name = "FILE_PATH")]
        file: Option<PathBuf>,

        /// İnteraktif bir REPL (Read-Eval-Print Loop) oturumu başlat.
        #[clap(short, long)]
        interactive: bool,
    },
    // 'help' komutunu buraya eklemene gerek yok, clap bunu otomatik olarak yönetir.
}