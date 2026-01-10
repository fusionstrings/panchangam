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
use alloc::string::String;
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

