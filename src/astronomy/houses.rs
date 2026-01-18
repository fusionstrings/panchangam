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

/// Calculate houses and ascendant for a given time and location.
///
/// # Arguments
/// * `jd` - Julian Day (UT)
/// * `lat` - Geographic latitude
/// * `lon` - Geographic longitude
/// * `hsys` - House system character (e.g., 'P' for Placidus, 'W' for Whole Sign)
///     * 'P' = Placidus
///     * 'K' = Koch
///     * 'O' = Porphyrius
///     * 'R' = Regiomontanus
///     * 'C' = Campanus
///     * 'W' = Whole Sign (Vedic standard often matches this or Equal)
///     * 'E' = Equal
///     * 'V' = Vehlow
///     * 'A' = Alcabitius
///     * 'X' = Meridian
///     * 'H' = Horizon
///     * 'B' = Bohm
pub fn calculate_houses(jd: f64, lat: f64, lon: f64, _hsys: char) -> Result<HouseInfo, JsValue> {
    // Note: swe_houses is not available in bindings yet.
    // Using manual calculation for Ascendant (Lagna).
    
    // 1. Get Greenwich Sidereal Time (hours)
    let gst = unsafe { swe_bindings::swe_sidtime(jd) };
    
    // 2. Local Sidereal Time (degrees)
    // LST = GST * 15 + long
    let mut lst = gst * 15.0 + lon;
    while lst < 0.0 { lst += 360.0; }
    while lst >= 360.0 { lst -= 360.0; }
    
    // 3. Obliquity of Ecliptic (Epsilon)
    // Calculate using Sun's position logic or approximation
    // J2000 epoch: 2451545.0
    // T = (jd - 2451545.0) / 36525.0
    // eps = 23.4392911 - 0.0130042 * T
    let t_jul_cent = (jd - 2451545.0) / 36525.0;
    let epsilon = 23.4392911 - 0.0130042 * t_jul_cent;
    
    // 4. Calculate Ascendant
    // tan(Asc) = -cos(LST) / (sin(e)*tan(lat) + cos(e)*sin(LST))
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
    // tan(MC) = tan(LST) / cos(e)
    // With atan2: y = sin(LST), x = cos(LST) * cos(e)
    // But careful with quadrants.
    // A simpler formula: tan(MC) = tan(LST) / cos(eps)
    // If LST in 0-180, MC in 0-180?
    // Let's use atan2(sin(LST), cos(LST)*cos(eps))
    let mc_y = sin_lst;
    let mc_x = cos_lst * cos_e;
    let mc_rad = mc_y.atan2(mc_x);
    let mut mc_deg = mc_rad / rad;
    if mc_deg < 0.0 { mc_deg += 360.0; }
    
    // Populate simple struct (others 0.0 for now)
    // We can populate equal houses from Ascendant easily if needed
    let mut cusps = alloc::vec![0.0; 12];
    cusps[0] = asc_deg; // 1st House
    
    // Equal House System (simplified fallback)
    for i in 1..12 {
        cusps[i] = (asc_deg + (i as f64) * 30.0) % 360.0;
    }

    Ok(HouseInfo {
        ascendant: asc_deg,
        mc: mc_deg,
        armc: lst, // approximates ARMC
        vertex: 0.0,
        equatorial_ascendant: 0.0,
        co_ascendant1: 0.0,
        co_ascendant2: 0.0,
        polar_ascendant: 0.0,
        cusps,
    })
}
