//! Tithi (Lunar Day) calculations
//! Tithi = (Moon longitude - Sun longitude) / 12°

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use alloc::string::{String, ToString};
use crate::astronomy::planets::{sun_longitude, moon_longitude};

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

/// Calculate the Julian Day when the current Tithi ends
/// 
/// # Arguments
/// * `jd` - Julian Day to start search from
/// 
/// # Returns
/// Julian Day when the current Tithi ends
#[wasm_bindgen]
pub fn tithi_end_time(jd: f64) -> f64 {
    let current = calculate_tithi(jd);
    let target_angle = current.index as f64 * 12.0; // End of current Tithi
    
    // Iterative solver (simple Newton-Raphson)
    let mut search_jd = jd;
    for _ in 0..20 {
        let sun_long = sun_longitude(search_jd);
        let moon_long = moon_longitude(search_jd);
        let mut diff = moon_long - sun_long;
        if diff < 0.0 { diff += 360.0; }
        
        let error = target_angle - diff;
        if error.abs() < 0.001 {
            break;
        }
        
        // Moon moves ~13.2 deg/day, Sun moves ~1 deg/day
        // Net motion ~12.2 deg/day
        let step = error / 12.2;
        search_jd += step;
    }
    
    search_jd
}

/// Calculate the Julian Day when the current Tithi started
/// 
/// # Arguments
/// * `jd` - Julian Day to start search from
/// 
/// # Returns
/// Julian Day when the current Tithi started
#[wasm_bindgen]
pub fn tithi_start_time(jd: f64) -> f64 {
    let current = calculate_tithi(jd);
    // Start of current Tithi is end of previous Tithi
    let target_angle = (current.index as f64 - 1.0) * 12.0;
    
    // Search backwards
    let mut search_jd = jd;
    for _ in 0..20 {
        let sun_long = sun_longitude(search_jd);
        let moon_long = moon_longitude(search_jd);
        let mut diff = moon_long - sun_long;
        if diff < 0.0 { diff += 360.0; }
        
        let mut error = target_angle - diff;
        // Handle wrap-around (e.g., Tithi 1 starts at 0°)
        if error > 180.0 { error -= 360.0; }
        if error < -180.0 { error += 360.0; }
        
        if error.abs() < 0.001 {
            break;
        }
        
        let step = error / 12.2;
        search_jd += step;
    }
    
    search_jd
}
