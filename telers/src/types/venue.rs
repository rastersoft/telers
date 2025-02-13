use super::Location;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object represents a venue.
/// # Documentation
/// <https://core.telegram.org/bots/api#venue>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Venue {
    /// Venue location. Can't be a live location
    pub location: Location,
    /// Name of the venue
    pub title: Box<str>,
    /// Address of the venue
    pub address: Box<str>,
    /// Foursquare identifier of the venue
    pub foursquare_id: Option<Box<str>>,
    /// Foursquare type of the venue. (For example, 'arts_entertainment/default', 'arts_entertainment/aquarium' or 'food/icecream'.)
    pub foursquare_type: Option<Box<str>>,
    /// Google Places identifier of the venue
    pub google_place_id: Option<Box<str>>,
    /// Google Places type of the venue. (See [`supported types`](https://developers.google.com/places/web-service/supported_types).)
    pub google_place_type: Option<Box<str>>,
}
