use serde::{Deserialize, Serialize};

/// The background is filled using the selected color
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundfillsolid>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct BackgroundFillSolid {
    /// The color of the background fill in the RGB24 format
    pub color: u32,
}
