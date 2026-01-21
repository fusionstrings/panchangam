//! Nakshatra (Lunar Mansion) calculations
//! Moon's longitude divided into 27 segments of 13°20' each

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use alloc::string::{String, ToString};
use crate::astronomy::planets::moon_longitude;
use crate::astronomy::ayanamsha::{AyanamshaMode, get_ayanamsha, tropical_to_sidereal};
use crate::astronomy::solver::find_angle_crossing;

/// Nakshatra names (27 lunar mansions)
pub const NAKSHATRA_NAMES: [&str; 27] = [
    "Ashwini", "Bharani", "Krittika", "Rohini", "Mrigashirsha",
    "Ardra", "Punarvasu", "Pushya", "Ashlesha", "Magha",
    "Purva Phalguni", "Uttara Phalguni", "Hasta", "Chitra", "Swati",
    "Vishakha", "Anuradha", "Jyeshtha", "Mula", "Purva Ashadha",
    "Uttara Ashadha", "Shravana", "Dhanishta", "Shatabhisha", "Purva Bhadrapada",
    "Uttara Bhadrapada", "Revati",
];

/// Nakshatra lords (planetary rulers)
pub const NAKSHATRA_LORDS: [&str; 27] = [
    "Ketu", "Venus", "Sun", "Moon", "Mars",
    "Rahu", "Jupiter", "Saturn", "Mercury", "Ketu",
    "Venus", "Sun", "Moon", "Mars", "Rahu",
    "Jupiter", "Saturn", "Mercury", "Ketu", "Venus",
    "Sun", "Moon", "Mars", "Rahu", "Jupiter",
    "Saturn", "Mercury",
];

/// Nakshatra qualities
pub const NAKSHATRA_QUALITIES: [&str; 27] = [
    "Swift/Light", "Fierce/Severe", "Mixed", "Fixed/Permanent", "Soft/Gentle",
    "Sharp/Dreadful", "Movable/Ephemeral", "Light/Swift", "Sharp/Dreadful", "Fierce/Severe",
    "Fierce/Severe", "Fixed/Permanent", "Light/Swift", "Soft/Gentle", "Movable/Ephemeral",
    "Mixed", "Soft/Gentle", "Sharp/Dreadful", "Sharp/Dreadful", "Fierce/Severe",
    "Fixed/Permanent", "Movable/Ephemeral", "Movable/Ephemeral", "Movable/Ephemeral", "Fierce/Severe",
    "Fixed/Permanent", "Soft/Gentle",
];

/// Nakshatra information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct NakshatraInfo {
    /// Nakshatra index (1-27)
    pub index: u8,
    /// Nakshatra name
    pub name: String,
    /// Planetary ruler
    pub ruler: String,
    /// Quality/nature
    pub quality: String,
    /// Pada (quarter, 1-4)
    pub pada: u8,
}

/// Calculate Nakshatra for a given Julian Day
/// Each Nakshatra spans 13°20' (13.333... degrees)
#[wasm_bindgen]
pub fn calculate_nakshatra(jd: f64, ayanamsha_mode: AyanamshaMode) -> NakshatraInfo {
    let moon_tropical = moon_longitude(jd);
    let ayanamsha = get_ayanamsha(ayanamsha_mode, jd);
    let moon_sidereal = tropical_to_sidereal(moon_tropical, ayanamsha);
    
    // Each Nakshatra = 360/27 = 13.333... degrees
    let nakshatra_span = 360.0 / 27.0;
    let nakshatra_float = moon_sidereal / nakshatra_span;
    let nakshatra_index = (nakshatra_float.floor() as u8) + 1;
    
    // Pada (quarter within Nakshatra)
    let within_nakshatra = nakshatra_float - nakshatra_float.floor();
    let pada = (within_nakshatra * 4.0).floor() as u8 + 1;
    
    let idx = (nakshatra_index - 1) as usize;
    
    NakshatraInfo {
        index: nakshatra_index,
        name: NAKSHATRA_NAMES[idx].to_string(),
        ruler: NAKSHATRA_LORDS[idx].to_string(),
        quality: NAKSHATRA_QUALITIES[idx].to_string(),
        pada,
    }
}

/// Calculate Julian Day when Moon transitions to next Nakshatra
#[wasm_bindgen]
pub fn nakshatra_end_time(jd: f64, ayanamsha_mode: AyanamshaMode) -> f64 {
    let current = calculate_nakshatra(jd, ayanamsha_mode);
    let nakshatra_span = 360.0 / 27.0;
    let target_long = current.index as f64 * nakshatra_span; // End of current Nakshatra
    
    // Moon moves ~13.2 deg/day. 1 Nakshatra ~ 1 day. Search 1.2 days ahead.
    let start_search = jd;
    let end_search = jd + 1.2;
    
    find_angle_crossing(
        |t| {
            let mt = moon_longitude(t);
            let ayan = get_ayanamsha(ayanamsha_mode, t);
            tropical_to_sidereal(mt, ayan)
        },
        start_search,
        end_search,
        target_long
    ).unwrap_or(jd)
}

/// Calculate Julian Day when current Nakshatra started
#[wasm_bindgen]
pub fn nakshatra_start_time(jd: f64, ayanamsha_mode: AyanamshaMode) -> f64 {
    let current = calculate_nakshatra(jd, ayanamsha_mode);
    let nakshatra_span = 360.0 / 27.0;
    // Start of current is end of previous (index - 1)
    let target_long = (current.index as f64 - 1.0) * nakshatra_span;
    
    // Search backward 1.2 days
    let start_search = jd - 1.2;
    let end_search = jd;
    
    find_angle_crossing(
        |t| {
            let mt = moon_longitude(t);
            let ayan = get_ayanamsha(ayanamsha_mode, t);
            tropical_to_sidereal(mt, ayan)
        },
        start_search,
        end_search,
        target_long
    ).unwrap_or(jd)
}
