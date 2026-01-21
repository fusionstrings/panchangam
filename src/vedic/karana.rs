//! Karana (Half-Tithi) calculations
//! Each Tithi has 2 Karanas (6° increments)

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use alloc::string::ToString;
use crate::astronomy::planets::{sun_longitude, moon_longitude};
use crate::astronomy::solver::find_angle_crossing;

/// Fixed Karanas (occur once per lunar month at specific positions)
pub const FIXED_KARANAS: [&str; 4] = ["Shakuni", "Chatushpada", "Naga", "Kimstughna"];

/// Cycling Karanas (repeat 7 times per lunar month)
pub const CYCLING_KARANAS: [&str; 7] = [
    "Bava", "Balava", "Kaulava", "Taitila", "Gara", "Vanija", "Vishti"
];

/// Karana information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct KaranaInfo {
    /// Karana index (1-60 per lunar month)
    pub index: u8,
    /// Karana name
    pub name: alloc::string::String,
    /// Whether first or second half of Tithi
    pub half: u8,
}

/// Calculate Karana for a given Julian Day
/// There are 60 Karanas per lunar month (2 per Tithi)
#[wasm_bindgen]
pub fn calculate_karana(jd: f64) -> KaranaInfo {
    let sun_long = sun_longitude(jd);
    let moon_long = moon_longitude(jd);
    
    // Angular distance
    let mut diff = moon_long - sun_long;
    if diff < 0.0 {
        diff += 360.0;
    }
    
    // Each Karana = 6 degrees
    let karana_index = (diff / 6.0).floor() as u8 + 1;
    
    // Determine which half of Tithi
    let half = if (karana_index % 2) == 1 { 1 } else { 2 };
    
    // Determine Karana name
    // Karanas 1 = Kimstughna (last fixed), 2-57 = cycling, 58-60 = other fixed
    let name = match karana_index {
        1 => FIXED_KARANAS[3].to_string(), // Kimstughna
        58 => FIXED_KARANAS[0].to_string(), // Shakuni
        59 => FIXED_KARANAS[1].to_string(), // Chatushpada
        60 => FIXED_KARANAS[2].to_string(), // Naga
        n => {
            // Cycling: 2-57 maps to 0-55, then mod 7
            let cycle_idx = ((n - 2) % 7) as usize;
            CYCLING_KARANAS[cycle_idx].to_string()
        }
    };
    
    KaranaInfo {
        index: karana_index,
        name,
        half,
    }
}

/// Calculate Julian Day when current Karana ends
#[wasm_bindgen]
pub fn karana_end_time(jd: f64) -> f64 {
    let current = calculate_karana(jd);
    let target_angle = current.index as f64 * 6.0;

    // Search window: Karana is ~0.5 days. Search 0.6 days ahead.
    let start_search = jd;
    let end_search = jd + 0.6;

    find_angle_crossing(
        |t| {
            let sl = sun_longitude(t);
            let ml = moon_longitude(t);
            let mut diff = ml - sl;
            if diff < 0.0 { diff += 360.0; }
            diff
        },
        start_search,
        end_search,
        target_angle
    ).unwrap_or(jd)
}

/// Calculate Julian Day when current Karana started
#[wasm_bindgen]
pub fn karana_start_time(jd: f64) -> f64 {
    let current = calculate_karana(jd);
    let target_angle = (current.index as f64 - 1.0) * 6.0;

    // Search window: Karana is ~0.5 days. Search 0.6 days backward.
    let start_search = jd - 0.6;
    let end_search = jd;

    find_angle_crossing(
        |t| {
            let sl = sun_longitude(t);
            let ml = moon_longitude(t);
            let mut diff = ml - sl;
            if diff < 0.0 { diff += 360.0; }
            diff
        },
        start_search,
        end_search,
        target_angle
    ).unwrap_or(jd)
}
