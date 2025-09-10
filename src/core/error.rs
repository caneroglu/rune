use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvError {
    #[error("I/O hatası oluştu")]
    Io(#[from] std::io::Error), // io::Error'dan otomatik dönüşüm

    #[error("Veri serileştirme/deserileştirme hatası")]
    Serialization(#[from] serde_json::Error), // serde_json::Error'dan otomatik dönüşüm

    #[error("'{0}' anahtarı bulunamadı")]
    KeyNotFound(String), // Kendi özel durumumuz

    #[error("Disk dolu")]
    DiskFull,
}