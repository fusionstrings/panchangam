//! Tithi (Lunar Day) calculations
//! Tithi = (Moon longitude - Sun longitude) / 12°

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use alloc::string::{String, ToString};
use crate::astronomy::planets::{sun_longitude, moon_longitude};
use crate::astronomy::solver::find_angle_crossing;

/// Tithi names (1-30)
pub const TITHI_NAMES: [&str; 30] = [
    "Pratipada", "Dwitiya", "Tritiya", "Chaturthi", "Panchami",
    "Shashthi", "Saptami", "Ashtami", "Navami", "Dashami",
    "Ekadashi", "Dwadashi", "Trayodashi", "Chaturdashi", "Purnima",
    "Pratipada", "Dwitiya", "Tritiya", "Chaturthi", "Panchami",
    "Shashthi", "Saptami", "Ashtami", "Navami", "Dashami",
    "Ekadashi", "Dwadashi", "Trayodashi", "Chaturdashi", "Amavasya",
];

/// Paksha (lunar fortnight)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[wasm_bindgen]
pub enum Paksha {
    Shukla = 0, // Waxing (bright fortnight)
    Krishna = 1, // Waning (dark fortnight)
}

/// Tithi information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct TithiInfo {
    /// Tithi index (1-30)
    pub index: u8,
    /// Tithi name
    pub name: String,
    /// Lunar fortnight
    pub paksha: Paksha,
    /// Completion percentage (0.0 to 1.0)
    pub completion: f64,
}

#[wasm_bindgen]
impl TithiInfo {
    #[wasm_bindgen(getter)]
    pub fn paksha_name(&self) -> String {
        match self.paksha {
            Paksha::Shukla => "Shukla".to_string(),
            Paksha::Krishna => "Krishna".to_string(),
        }
    }
}

/// Calculate Tithi for a given Julian Day
/// Returns TithiInfo with index, name, paksha, and completion percentage
#[wasm_bindgen]
pub fn calculate_tithi(jd: f64) -> TithiInfo {
    let sun_long = sun_longitude(jd);
    let moon_long = moon_longitude(jd);
    
    // Angular distance: Moon - Sun
    let mut diff = moon_long - sun_long;
    if diff < 0.0 {
        diff += 360.0;
    }
    
    // Each Tithi spans 12 degrees
    let tithi_float = diff / 12.0;
    let tithi_index = (tithi_float.floor() as u8) + 1;
    
    // Completion within current Tithi
    let completion = tithi_float - tithi_float.floor();
    
    // Paksha: Shukla (1-15), Krishna (16-30)
    let paksha = if tithi_index <= 15 {
        Paksha::Shukla
    } else {
        Paksha::Krishna
    };
    
    // Adjust index for display (1-15 for both fortnights)
    let _display_index = if tithi_index <= 15 { tithi_index } else { tithi_index - 15 };
    
    TithiInfo {
        index: tithi_index,
        name: TITHI_NAMES[(tithi_index - 1) as usize].to_string(),
        paksha,
        completion,
    }
}

/// Julian Day when the current Tithi ends
#[wasm_bindgen]
pub fn tithi_end_time(jd: f64) -> f64 {
    let current = calculate_tithi(jd);
    let target_angle = current.index as f64 * 12.0; // End of current Tithi
    
    // Tithi length is approx 0.9 to 1.0 day. Search up to 1.2 days ahead.
    let start_search = jd;
    let end_search = jd + 1.2;
    
    find_angle_crossing(
        |t| {
            let sl = sun_longitude(t);
            let ml = moon_longitude(t);
            let mut d = ml - sl;
            if d < 0.0 { d += 360.0; }
            d
        },
        start_search,
        end_search,
        target_angle
    ).unwrap_or(jd) // Fallback to input if not found (should not happen)
}

/// Julian Day when the current Tithi started
#[wasm_bindgen]
pub fn tithi_start_time(jd: f64) -> f64 {
    let current = calculate_tithi(jd);
    // Start of current Tithi is end of previous Tithi
    let target_angle = (current.index as f64 - 1.0) * 12.0;
    
    // Search backwards
    let start_search = jd - 1.2;
    let end_search = jd;
    
    find_angle_crossing(
        |t| {
            let sl = sun_longitude(t);
            let ml = moon_longitude(t);
            let mut d = ml - sl;
            if d < 0.0 { d += 360.0; }
            d
        },
        start_search,
        end_search,
        target_angle
    ).unwrap_or(jd)
}
