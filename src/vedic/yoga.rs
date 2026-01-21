//! Yoga calculations
//! Yoga = (Moon longitude + Sun longitude) / 13°20'

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use alloc::string::{String, ToString};
use crate::astronomy::planets::{sun_longitude, moon_longitude};
use crate::astronomy::ayanamsha::{AyanamshaMode, get_ayanamsha, tropical_to_sidereal};
use crate::astronomy::solver::find_angle_crossing;

/// Yoga names (27 yogas)
pub const YOGA_NAMES: [&str; 27] = [
    "Vishkumbha", "Priti", "Ayushman", "Saubhagya", "Shobhana",
    "Atiganda", "Sukarman", "Dhriti", "Shula", "Ganda",
    "Vriddhi", "Dhruva", "Vyaghata", "Harshana", "Vajra",
    "Siddhi", "Vyatipata", "Variyana", "Parigha", "Shiva",
    "Siddha", "Sadhya", "Shubha", "Shukla", "Brahma",
    "Indra", "Vaidhriti",
];

/// Yoga information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct YogaInfo {
    /// Yoga index (1-27)
    pub index: u8,
    /// Yoga name
    pub name: String,
}

/// Calculate Yoga for a given Julian Day
/// Formula: Yoga = floor((Moon_long + Sun_long) / 13.333) + 1
#[wasm_bindgen]
pub fn calculate_yoga(jd: f64, ayanamsha_mode: AyanamshaMode) -> YogaInfo {
    let sun_tropical = sun_longitude(jd);
    let moon_tropical = moon_longitude(jd);
    let ayanamsha = get_ayanamsha(ayanamsha_mode, jd);
    
    let sun_sidereal = tropical_to_sidereal(sun_tropical, ayanamsha);
    let moon_sidereal = tropical_to_sidereal(moon_tropical, ayanamsha);
    
    // Sum of longitudes
    let mut sum = sun_sidereal + moon_sidereal;
    if sum >= 360.0 {
        sum -= 360.0;
    }
    
    // Each Yoga = 360/27 = 13.333... degrees
    let yoga_span = 360.0 / 27.0;
    let yoga_index = (sum / yoga_span).floor() as u8 + 1;
    
    YogaInfo {
        index: yoga_index,
        name: YOGA_NAMES[(yoga_index - 1) as usize].to_string(),
    }
}

/// Calculate Julian Day when current Yoga ends
#[wasm_bindgen]
pub fn yoga_end_time(jd: f64, ayanamsha_mode: AyanamshaMode) -> f64 {
    let current = calculate_yoga(jd, ayanamsha_mode);
    let yoga_span = 360.0 / 27.0;
    let target_angle = current.index as f64 * yoga_span;

    // Search window: Yoga changes every ~0.9 days. Safe to search 1.0 day ahead.
    let start_search = jd;
    let end_search = jd + 1.0;

    find_angle_crossing(
        |t| {
            let sl = sun_longitude(t);
            let ml = moon_longitude(t);
            let ayan = get_ayanamsha(ayanamsha_mode, t);
            let sl_sid = tropical_to_sidereal(sl, ayan);
            let ml_sid = tropical_to_sidereal(ml, ayan);
            let mut sum = sl_sid + ml_sid;
            if sum >= 360.0 { sum -= 360.0; }
            sum
        },
        start_search,
        end_search,
        target_angle
    ).unwrap_or(jd)
}

/// Calculate Julian Day when current Yoga started
#[wasm_bindgen]
pub fn yoga_start_time(jd: f64, ayanamsha_mode: AyanamshaMode) -> f64 {
    let current = calculate_yoga(jd, ayanamsha_mode);
    let yoga_span = 360.0 / 27.0;
    let target_angle = (current.index as f64 - 1.0) * yoga_span;

    // Search window: Yoga changes every ~0.9 days. Search 1.0 day backward.
    let start_search = jd - 1.0;
    let end_search = jd;

    find_angle_crossing(
        |t| {
            let sl = sun_longitude(t);
            let ml = moon_longitude(t);
            let ayan = get_ayanamsha(ayanamsha_mode, t);
            let sl_sid = tropical_to_sidereal(sl, ayan);
            let ml_sid = tropical_to_sidereal(ml, ayan);
            let mut sum = sl_sid + ml_sid;
            if sum >= 360.0 { sum -= 360.0; }
            sum
        },
        start_search,
        end_search,
        target_angle
    ).unwrap_or(jd)
}
