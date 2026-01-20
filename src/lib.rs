//! # Panchangam - High-Precision Vedic Calendar Library
//!
//! This library provides Drik Ganita (astronomical precision) calculations
//! for Vedic Panchangam (five-limbed calendar) including:
//! - Tithi (lunar day)
//! - Nakshatra (lunar mansion)  
//! - Yoga (luni-solar combination)
//! - Karana (half-tithi)
//! - Vara (weekday based on sunrise)
//!
//! It uses Swiss Ephemeris for planetary positions and `spa` for sunrise/sunset.

#![no_std]
extern crate alloc;
use alloc::string::{String, ToString};
use swisseph_wasm; // Ensure it's linked


use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

// Include generated Swiss Ephemeris bindings
// Re-export bindings from swisseph-wasm
pub(crate) use swisseph_wasm::swe_bindings;



// --- Modules ---
pub mod astronomy;
pub mod vedic;
pub mod geo;
pub mod muhurat;
pub mod constants;

/// Get the library version (panchangam)
#[wasm_bindgen]
pub fn get_version() -> String {
    String::from(env!("CARGO_PKG_VERSION"))
}

/// Get the underlying Swiss Ephemeris engine version
#[wasm_bindgen]
pub fn get_swisseph_version() -> String {
    let mut buf = [0i8; 256];
    unsafe {
        swe_bindings::swe_version(buf.as_mut_ptr());
    }
    let c_str = unsafe { core::ffi::CStr::from_ptr(buf.as_ptr()) };
    String::from(c_str.to_str().unwrap_or("Unknown"))
}



/// Location struct for geo-spatial calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}

#[wasm_bindgen]
impl Location {
    #[wasm_bindgen(constructor)]
    pub fn new(latitude: f64, longitude: f64, altitude: f64) -> Self {
        Self { latitude, longitude, altitude }
    }
}

/// Calculate sunrise time for a given date and location
/// Returns Unix timestamp in milliseconds
#[wasm_bindgen]
pub fn calculate_sunrise(year: i32, month: u32, day: u32, location: &Location) -> f64 {
    geo::sunrise_sunset::calculate_sunrise(year, month, day, location.latitude, location.longitude, location.altitude)
}

/// Calculate sunset time for a given date and location
/// Returns Unix timestamp in milliseconds
#[wasm_bindgen]
pub fn calculate_sunset(year: i32, month: u32, day: u32, location: &Location) -> f64 {
    geo::sunrise_sunset::calculate_sunset(year, month, day, location.latitude, location.longitude, location.altitude)
}

pub use astronomy::planets::PlanetData;

/// Calculate house system (Ascendant, MC, House Cusps)
/// 
/// # Arguments
/// * `jd` - Julian Day
/// * `lat` - Latitude
/// * `lon` - Longitude
/// * `hsys` - House System (e.g. 'P' for Placidus, 'W' for Whole Sign)
/// * `ayan_mode` - Ayanamsha mode (-1 for tropical, others for sidereal)
#[wasm_bindgen]
pub fn calculate_houses(
    jd: f64, 
    lat: f64, 
    lon: f64, 
    hsys: char, 
    ayan_mode: i32
) -> Result<astronomy::houses::HouseInfo, JsValue> {
    let mode = if ayan_mode < 0 { 
        None 
    } else { 
        Some(astronomy::ayanamsha::AyanamshaMode::from_i32(ayan_mode)) 
    };
    astronomy::houses::calculate_houses(jd, lat, lon, hsys, mode)
}

/// Calculate Sidereal Planet Positions for all 9 core planets
/// 
/// # Arguments
/// * `jd` - Julian Day
/// * `ayan_mode` - Ayanamsha mode (e.g., 1 for Lahiri)
#[wasm_bindgen]
pub fn calculate_planets(jd: f64, ayan_mode: i32) -> Result<JsValue, JsValue> {
    let mode = astronomy::ayanamsha::AyanamshaMode::from_i32(ayan_mode);
    let ayan_val = astronomy::ayanamsha::get_ayanamsha(mode, jd);
    
    let planets = astronomy::planets::get_planet_positions_bulk(jd, ayan_val);

    Ok(serde_wasm_bindgen::to_value(&planets)?)
}

/// Calculate Vimshottari Dasha details
/// 
/// # Arguments
/// * `moon_long` - Moon's sidereal longitude (degrees)
/// * `birth_time_ms` - Birth time (Unix ms)
/// * `current_time_ms` - Current time (Unix ms)
#[wasm_bindgen]
pub fn calculate_vimshottari(moon_long: f64, birth_time_ms: f64, current_time_ms: f64) -> vedic::dasha::DashaInfo {
    vedic::dasha::calculate_vimshottari(moon_long, birth_time_ms, current_time_ms)
}

