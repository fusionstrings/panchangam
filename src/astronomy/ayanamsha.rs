//! Ayanamsha calculations using Swiss Ephemeris

use wasm_bindgen::prelude::*;
use crate::swe_bindings;

/// Ayanamsha modes
#[derive(Debug, Clone, Copy, PartialEq)]
#[wasm_bindgen]
pub enum AyanamshaMode {
    /// Lahiri (Chitrapaksha)
    Lahiri = 0,
    /// Raman
    Raman = 1,
    /// Krishnamurti (KP)
    Krishnamurti = 2,
    /// True Chitrapaksha
    TrueCitra = 27,
    /// Fagan-Bradley
    FaganBradley = 4, // Custom ID, doesn't matter as long as unique
}

impl AyanamshaMode {
    pub fn to_swe_mode(&self) -> i32 {
        // Swiss Ephemeris sidereal mode constants (from swephexp.h)
        const SE_SIDM_LAHIRI: i32 = 1;
        const SE_SIDM_RAMAN: i32 = 3;
        const SE_SIDM_KRISHNAMURTI: i32 = 5;
        const SE_SIDM_TRUE_CITRA: i32 = 27;
        const SE_SIDM_FAGAN_BRADLEY: i32 = 0;
        
        match self {
            AyanamshaMode::Lahiri => SE_SIDM_LAHIRI,
            AyanamshaMode::Raman => SE_SIDM_RAMAN,
            AyanamshaMode::Krishnamurti => SE_SIDM_KRISHNAMURTI,
            AyanamshaMode::TrueCitra => SE_SIDM_TRUE_CITRA,
            AyanamshaMode::FaganBradley => SE_SIDM_FAGAN_BRADLEY,
        }
    }
}

/// Get Ayanamsha value for a given mode and Julian Day
#[wasm_bindgen]
pub fn get_ayanamsha(mode: AyanamshaMode, jd: f64) -> f64 {
    unsafe {
        swe_bindings::swe_set_sid_mode(mode.to_swe_mode(), 0.0, 0.0);
        swe_bindings::swe_get_ayanamsa_ut(jd)
    }
}

/// Convert tropical (sayana) longitude to sidereal (nirayana)
/// Formula: L_nirayana = L_sayana - Ayanamsha(t)
pub fn tropical_to_sidereal(tropical_long: f64, ayanamsha: f64) -> f64 {
    let mut sidereal = tropical_long - ayanamsha;
    if sidereal < 0.0 {
        sidereal += 360.0;
    }
    sidereal % 360.0
}
