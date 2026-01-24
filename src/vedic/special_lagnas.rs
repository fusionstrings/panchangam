use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct SpecialLagnas {
    pub hora_lagna: f64,
    pub ghati_lagna: f64,
    pub sree_lagna: f64,
}

/// Calculate Special Lagnas (Hora, Ghati, Sree Lagna)
/// 
/// # Arguments
/// * `birth_jd` - Julian Day of birth
/// * `sunrise_jd` - Julian Day of sunrise on birth day
/// * `sunrise_sun_long` - Sun's sidereal longitude at sunrise (degrees)
/// * `lagna_long` - Ascendant (Lagna) sidereal longitude at birth (degrees)
/// * `moon_long` - Moon's sidereal longitude at birth (degrees)
pub fn calculate_special_lagnas(
    birth_jd: f64,
    sunrise_jd: f64,
    sunrise_sun_long: f64,
    lagna_long: f64,
    moon_long: f64,
) -> SpecialLagnas {
    let diff_days = birth_jd - sunrise_jd;
    let diff_hours = diff_days * 24.0;
    
    // Hora Lagna: 30 degrees (1 sign) per hour
    // HL = Sun_at_sunrise + (hours_since_sunrise * 30)
    let hora_lagna = (sunrise_sun_long + diff_hours * 30.0) % 360.0;
    
    // Ghati Lagna: 60 degrees (2 signs) per hour (completes 4 circles per day)
    // GL = Sun_at_sunrise + (hours_since_sunrise * 60)
    let ghati_lagna = (sunrise_sun_long + diff_hours * 60.0) % 360.0;
    
    // Sree Lagna: Lagna + (Moon's portion of current Nakshatra * 360)
    // Nakshatra span is 360/27 = 13.3333... degrees
    let nk_span = 360.0 / 27.0;
    let nk_portion = (moon_long % nk_span) / nk_span;
    let sree_lagna = (lagna_long + nk_portion * 360.0) % 360.0;
    
    SpecialLagnas {
        hora_lagna,
        ghati_lagna,
        sree_lagna,
    }
}
