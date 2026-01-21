//! Ayanamsha calculations using Swiss Ephemeris

use wasm_bindgen::prelude::*;

/// Ayanamsha modes
#[derive(Debug, Clone, Copy, PartialEq)]
#[wasm_bindgen]
pub enum AyanamshaMode {
    /// Fagan-Bradley (0)
    FaganBradley = 0,
    /// Lahiri (1)
    Lahiri = 1,
    /// De Luce (2)
    DeLuce = 2,
    /// Raman (3)
    Raman = 3,
    /// Krishnamurti (5)
    Krishnamurti = 5,
    /// Yukteshwar (7)
    Yukteshwar = 7,
    /// J.N. Bhasin (8)
    JNBhasin = 8,
    /// True Chitrapaksha (27)
    TrueCitra = 27,
}

impl AyanamshaMode {
    pub fn from_i32(val: i32) -> Self {
        match val {
            0 => AyanamshaMode::FaganBradley,
            1 => AyanamshaMode::Lahiri,
            2 => AyanamshaMode::DeLuce,
            3 => AyanamshaMode::Raman,
            5 => AyanamshaMode::Krishnamurti,
            7 => AyanamshaMode::Yukteshwar,
            8 => AyanamshaMode::JNBhasin,
            27 => AyanamshaMode::TrueCitra,
            _ => AyanamshaMode::Lahiri,
        }
    }
    
    pub fn to_swe_mode(&self) -> i32 {
        *self as i32
    }
}

/// Get Ayanamsha value for a given mode and Julian Day
/// Get Ayanamsha value for a given mode and Julian Day
#[wasm_bindgen]
pub fn get_ayanamsha(mode: AyanamshaMode, jd: f64) -> f64 {
    let swe_mode = match mode {
        AyanamshaMode::FaganBradley => swiss_eph::safe::SiderealMode::FaganBradley,
        AyanamshaMode::Lahiri => swiss_eph::safe::SiderealMode::Lahiri,
        AyanamshaMode::DeLuce => swiss_eph::safe::SiderealMode::DeLuce,
        AyanamshaMode::Raman => swiss_eph::safe::SiderealMode::Raman,
        AyanamshaMode::Krishnamurti => swiss_eph::safe::SiderealMode::Krishnamurti,
        AyanamshaMode::Yukteshwar => swiss_eph::safe::SiderealMode::Yukteshwar,
        AyanamshaMode::JNBhasin => swiss_eph::safe::SiderealMode::JNBhasin,
        AyanamshaMode::TrueCitra => swiss_eph::safe::SiderealMode::TrueCitra,
    };

    swiss_eph::safe::set_sidereal_mode(swe_mode);
    swiss_eph::safe::get_ayanamsa(jd)
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
