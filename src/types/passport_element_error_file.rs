use serde::{Deserialize, Serialize};

/// Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
/// <https://core.telegram.org/bots/api#passportelementerrorfile>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorFile {
    /// Error source, must be *file*
    #[serde(default = "file")]
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of 'utility_bill', 'bank_statement', 'rental_agreement', 'passport_registration', 'temporary_registration'
    #[serde(rename = "type")]
    pub element_type: String,
    /// Base64-encoded file hash
    pub file_hash: String,
    /// Error message
    pub message: String,
}

fn file() -> String {
    "file".to_string()
}
