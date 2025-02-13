use serde::{Deserialize, Serialize};

/// This object represents a file uploaded to Telegram Passport. Currently all Telegram Passport files are in JPEG format when decrypted and don't exceed 10MB.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportfile>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct PassportFile {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: Box<str>,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: Box<str>,
    /// File size in bytes
    pub file_size: i64,
    /// Unix time when the file was uploaded
    pub file_date: i64,
}
