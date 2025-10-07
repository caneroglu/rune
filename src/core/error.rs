use thiserror::Error;
use crate::query::parser::Rule;

// TODO: Serde için daha elegant hata tipi tanımla.
#[derive(Error, Debug)]
pub enum RuneError {
    #[error("I/O hatası oluştu: {0}")]
    IoError(#[from] std::io::Error),

    // Bincode serileştirme hataları için
    #[error("Bincode serializasyon hatası: {0}")]
    BincodeError(#[from] bincode::error::EncodeError),

    #[error("Bincode deserializasyon hatası: {0}")]
    BincodeDecodeError(#[from] bincode::error::DecodeError),

    // Generic serde hataları için
    #[error("Serializasyon hatası: {message}")]
    SerializationError { message: String },

    // Database spesifik hatalar
    #[error("Database bulunamadı: {db_name}")]
    DatabaseNotFound { db_name: String },

    #[error("'{key}' anahtarı bulunamadı")]
    KeyNotFoundError { key: String },

    // Query hataları
    #[error("Hatalı sorgu biçimi.")]
    QuerySyntaxError,

    #[error("Query parse hatası: {0}")]
    QueryParseError(#[from] pest::error::Error<Rule>),

    // File system hataları
    #[error("Disk dolu")]
    DiskFullError,

    #[error("Database dosyası bozuk: {path}")]
    CorruptedDatabase { path: String },

    // Patricia tree hataları
    #[error("Index hatası: {message}")]
    IndexError { message: String },
}

// Utility type alias - Result<T> yerine RuneResult<T> kullanabilirsin
pub type RuneResult<T> = Result<T, RuneError>;

impl RuneError {
    // Custom serde hatası oluşturmak için
    pub fn serde_error<E: std::fmt::Display>(err: E) -> Self {
        RuneError::SerializationError {
            message: err.to_string(),
        }
    }

    // Database hatası oluşturmak için
    pub fn db_not_found(db_name: impl Into<String>) -> Self {
        RuneError::DatabaseNotFound {
            db_name: db_name.into(),
        }
    }

    // Key hatası oluşturmak için
    pub fn key_not_found(key: impl Into<String>) -> Self {
        RuneError::KeyNotFoundError {
            key: key.into(),
        }
    }

 
}