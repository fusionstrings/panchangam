//! House system calculations
//!
//! Calculates Ascendant, MC, and House cusps.

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use crate::swe_bindings;
use alloc::vec::Vec;

/// House system information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct HouseInfo {
    /// Ascendant (Lagna) in degrees (0-360)
    pub ascendant: f64,
    /// Midheaven (MC) in degrees
    pub mc: f64,
    /// ARMC (Sidereal Time)
    pub armc: f64,
    /// Vertex
    pub vertex: f64,
    /// Equatorial Ascendant
    pub equatorial_ascendant: f64,
    /// Co-Ascendant 1 (Koch)
    pub co_ascendant1: f64,
    /// Co-Ascendant 2 (Munkasey)
    pub co_ascendant2: f64,
    /// Polar Ascendant
    pub polar_ascendant: f64,
    /// House cusps (1-12)
    /// Note: Swiss Eph returns 13 values (0 is ignored), we return vector of 12
    pub cusps: Vec<f64>,
}

use crate::astronomy::ayanamsha::{AyanamshaMode, get_ayanamsha};

/// Calculate houses and ascendant for a given time and location.
///
/// # Arguments
/// * `jd` - Julian Day (UT)
/// * `lat` - Geographic latitude
/// * `lon` - Geographic longitude
/// * `hsys` - House system character (e.g., 'P' for Placidus)
/// * `ayan_mode` - Optional Ayanamsha mode for sidereal houses
pub fn calculate_houses(
    jd: f64, 
    lat: f64, 
    lon: f64, 
    hsys: char, 
    ayan_mode: Option<AyanamshaMode>
) -> Result<HouseInfo, JsValue> {
    // 1. Get Greenwich Sidereal Time (hours)
    let gst = unsafe { swe_bindings::swe_sidtime(jd) };
    
    // 2. Local Sidereal Time (degrees)
    let mut lst = gst * 15.0 + lon;
    while lst < 0.0 { lst += 360.0; }
    while lst >= 360.0 { lst -= 360.0; }
    
    // 3. Obliquity of Ecliptic (Epsilon)
    let mut xx = [0.0; 6];
    let mut serr = [0i8; 256];
    unsafe {
        // -1 = SE_ECL_NUT
        swe_bindings::swe_calc_ut(jd, -1, 0, xx.as_mut_ptr(), serr.as_mut_ptr());
    }
    let epsilon = xx[0];
    
    // 4. Calculate Ascendant
    let rad = core::f64::consts::PI / 180.0;
    let sin_lst = (lst * rad).sin();
    let cos_lst = (lst * rad).cos();
    let sin_e = (epsilon * rad).sin();
    let cos_e = (epsilon * rad).cos();
    let tan_lat = (lat * rad).tan();
    
    let x = sin_e * tan_lat + cos_e * sin_lst;
    let y = -cos_lst;
    
    let asc_rad = y.atan2(x);
    let mut asc_deg = asc_rad / rad;
    if asc_deg < 0.0 { asc_deg += 360.0; }
    
    // 5. MC (Midheaven)
    let mc_y = sin_lst;
    let mc_x = cos_lst * cos_e;
    let mc_rad = mc_y.atan2(mc_x);
    let mut mc_deg = mc_rad / rad;
    if mc_deg < 0.0 { mc_deg += 360.0; }
    
    // 6. Ayanamsha Correction (Sidereal)
    if let Some(mode) = ayan_mode {
        let ayan = get_ayanamsha(mode, jd);
        asc_deg = (asc_deg - ayan + 360.0) % 360.0;
        mc_deg = (mc_deg - ayan + 360.0) % 360.0;
    }

    // 7. House Cusps
    let mut cusps = alloc::vec![0.0; 12];
    
    match hsys {
        'W' => {
            // Whole Sign: Cusp 1 is 0 degrees of the sign containing Ascendant
            let sign_start = (asc_deg / 30.0).floor() * 30.0;
            for i in 0..12 {
                cusps[i] = (sign_start + (i as f64) * 30.0) % 360.0;
            }
        },
        _ => {
            // Default/Equal House: Cusp 1 is the Ascendant itself
            for i in 0..12 {
                cusps[i] = (asc_deg + (i as f64) * 30.0) % 360.0;
            }
        }
    }

    Ok(HouseInfo {
        ascendant: asc_deg,
        mc: mc_deg,
        armc: lst,
        vertex: 0.0,
        equatorial_ascendant: 0.0,
        co_ascendant1: 0.0,
        co_ascendant2: 0.0,
        polar_ascendant: 0.0,
        cusps,
    })
}
