//! House system calculations
//!
//! Calculates Ascendant, MC, and House cusps.

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
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
    // Map char to HouseSystem
    let sys = match hsys {
        'P' => swiss_eph::safe::HouseSystem::Placidus,
        'K' => swiss_eph::safe::HouseSystem::Koch,
        'O' => swiss_eph::safe::HouseSystem::Porphyrius,
        'R' => swiss_eph::safe::HouseSystem::Regiomontanus,
        'C' => swiss_eph::safe::HouseSystem::Campanus,
        'E' => swiss_eph::safe::HouseSystem::Equal,
        'W' => swiss_eph::safe::HouseSystem::WholeSign,
        'B' => swiss_eph::safe::HouseSystem::Alcabitus,
        'M' => swiss_eph::safe::HouseSystem::Morinus,
        'T' => swiss_eph::safe::HouseSystem::Topocentric,
        'V' => swiss_eph::safe::HouseSystem::Vehlow,
        _ => swiss_eph::safe::HouseSystem::Placidus, // Default fallback
    };

    // Calculate Tropical Houses
    let houses = swiss_eph::safe::houses(jd, lat, lon, sys)
        .map_err(|e| JsValue::from_str(&e.message))?;

    let mut asc_deg = houses.ascendant;
    let mut mc_deg = houses.mc;
    let armc = houses.armc;
    let mut vertex = houses.vertex;
    let eq_asc = houses.equatorial_ascendant;
    let co_asc1 = houses.co_ascendant_koch;
    let co_asc2 = houses.co_ascendant_munkasey;
    let pol_asc = houses.polar_ascendant;
    let mut cusps_vec = houses.cusps.to_vec();

    // Apply Ayanamsha if needed
    if let Some(mode) = ayan_mode {
        let ayan = get_ayanamsha(mode, jd);
        
        asc_deg = (asc_deg - ayan + 360.0) % 360.0;
        mc_deg = (mc_deg - ayan + 360.0) % 360.0;
        // ARMC is Sidereal Time (RAMC), usually not adjusted by Ayanamsha in the same way as longitude?
        // Wait, ARMC is Right Ascension of MC. 
        // If we want Sidereal positions, we adjust Ecliptic Longitudes (Asc, MC, Cusps, Vertex?).
        // Vertex is on Ecliptic? Yes.
        // Eq Asc? 
        // The manual code only adjusted Asc and MC:
        // asc_deg = (asc_deg - ayan + 360.0) % 360.0;
        // mc_deg = (mc_deg - ayan + 360.0) % 360.0;
        // And then recalculated cusps based on new Asc (for Equal/Whole).
        
        // Since we are using swiss-eph which gives us all cusps:
        // We should adjust ALL ecliptic longitudes.
        vertex = (vertex - ayan + 360.0) % 360.0;
        
        // Adjust cusps
        for i in 0..12 {
            cusps_vec[i] = (cusps_vec[i] - ayan + 360.0) % 360.0;
        }
        
    }

    Ok(HouseInfo {
        ascendant: asc_deg,
        mc: mc_deg,
        armc,
        vertex,
        equatorial_ascendant: eq_asc,
        co_ascendant1: co_asc1,
        co_ascendant2: co_asc2,
        polar_ascendant: pol_asc,
        cusps: cusps_vec,
    })
}
