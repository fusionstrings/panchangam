use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use crate::{Location, calculate_sunrise, calculate_sunset};
use crate::vedic::{tithi, nakshatra, yoga, vara, muhurat};
use crate::astronomy::{ayanamsha, solver};
use crate::astronomy::planets::{self, PlanetId};
use crate::astronomy::ayanamsha::AyanamshaMode;
use alloc::string::String;

#[derive(Debug, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct DailyPanchang {
    // Times
    /// Sunrise time in Unix milliseconds
    pub sunrise: f64,
    /// Sunset time in Unix milliseconds
    pub sunset: f64,
    
    // Tithi
    /// Tithi index (1-30). 15 = Purnima, 30 = Amavasya.
    pub tithi_index: u8,
    /// Name of the Tithi (e.g., "Shukla-Chaturdashi")
    #[wasm_bindgen(getter_with_clone)]
    pub tithi_name: String,
    /// Timestamp (Unix ms) when this Tithi ends. None if it doesn't end today.
    pub tithi_end_time: Option<f64>,

    // Nakshatra
    /// Nakshatra index (1-27). 1 = Ashwini.
    pub nakshatra_index: u8,
    /// Name of the Nakshatra (e.g., "Krittika")
    #[wasm_bindgen(getter_with_clone)]
    pub nakshatra_name: String,
    /// Timestamp (Unix ms) when this Nakshatra ends.
    pub nakshatra_end_time: Option<f64>,
    
    // Yoga
    /// Yoga index (1-27).
    pub yoga_index: u8,
    /// Name of the Yoga (e.g., "Vishkumbha")
    #[wasm_bindgen(getter_with_clone)]
    pub yoga_name: String,
    /// Timestamp (Unix ms) when this Yoga ends.
    pub yoga_end_time: Option<f64>,

    // Vara
    /// Solar Weekday name (e.g., "Adityawara")
    #[wasm_bindgen(getter_with_clone)]
    pub vara_name: String,
    
    // Config
    /// The Ayanamsha value (in degrees) used for calculations
    pub ayanamsha_value: f64,
    
    // Muhurat
    /// Auspicious and Inauspicious time periods for the day
    #[wasm_bindgen(getter_with_clone)]
    pub muhurats: muhurat::DayMuhurats,
}

/// Calculate the complete Panchangam for a given date and location.
///
/// This function performs the following steps:
/// 1. Calculates exact local Sunrise and Sunset times.
/// 2. Determines the Ayanamsha (precession) based on the selected mode.
/// 3. Computes the 5 Angas (Tithi, Nakshatra, Yoga, Karana, Vara) at the moment of Sunrise.
/// 4. Iteratively finds the end times for Tithi, Nakshatra, and Yoga using binary search.
/// 5. Calculates daily Muhurats (Rahu Kalam, Yamaganda, etc.) based on day division.
///
/// # Arguments
/// * `year` - Year (e.g., 2026)
/// * `month` - Month (1-12)
/// * `day` - Day of month (1-31)
/// * `location` - The geographical location of the observer
/// * `ayan_mode` - Ayanamsha mode:
///     * 1 = Lahiri (Chitrapaksha) [Default/Standard]
///     * 3 = Raman
///     * 5 = Krishnamurti
///     * 27 = True Chitrapaksha
#[wasm_bindgen]
pub fn calculate_daily_panchang(
    year: i32, 
    month: u32, 
    day: u32, 
    location: &Location, 
    ayan_mode: i32
) -> Result<DailyPanchang, JsValue> {


    // Ayanamsha mode selection
    let mode = match ayan_mode {
        1 => AyanamshaMode::Lahiri,
        3 => AyanamshaMode::Raman,
        5 => AyanamshaMode::Krishnamurti,
        27 => AyanamshaMode::TrueCitra,
        _ => AyanamshaMode::Lahiri,
    };

    let sunrise_ms = calculate_sunrise(year, month, day, location);
    if sunrise_ms.is_nan() {
        return Err(JsValue::from_str("Sunrise calculation failed (polar region?)"));
    }
    
    // Julian Day at Sunrise
    let sunrise_jd = (sunrise_ms / 86_400_000.0) + 2440587.5;
    let ayan_val = ayanamsha::get_ayanamsha(mode, sunrise_jd);

    // Tithi at Sunrise
    let t_info = tithi::calculate_tithi(sunrise_jd);
    let tithi_idx = t_info.index;
    
    // Calculate Tithi end time using binary search
    // Target is simply next integer index * 12 degrees
    let tithi_idx_val = tithi_idx as f64;
    let target_angle = tithi_idx_val * 12.0;
    
    let search_end_jd = sunrise_jd + 1.25; 
    
    let t_end_jd = solver::find_crossing_time(
        |jd| {
            use crate::astronomy::planets;
            let sun = planets::get_planet_position_sidereal(PlanetId::Sun, jd, ayanamsha::get_ayanamsha(mode, jd));
            let moon = planets::get_planet_position_sidereal(PlanetId::Moon, jd, ayanamsha::get_ayanamsha(mode, jd));
            let mut diff = moon.longitude - sun.longitude;
            if diff < 0.0 { diff += 360.0; }
            diff
        },
        sunrise_jd,
        search_end_jd,
        target_angle,
        360.0
    );
    let tithi_end_ms = t_end_jd.map(|jd| (jd - 2440587.5) * 86_400_000.0);

    // Nakshatra at Sunrise
    let n_info = nakshatra::calculate_nakshatra(sunrise_jd, mode);
    let n_idx = n_info.index;
    let nak_len = 360.0 / 27.0;
    let target_nak_angle = (n_idx as f64) * nak_len;
    
    let n_end_jd = solver::find_crossing_time(
        |jd| {
            let moon = planets::get_planet_position_sidereal(PlanetId::Moon, jd, ayanamsha::get_ayanamsha(mode, jd));
            moon.longitude
        },
        sunrise_jd,
        search_end_jd,
        target_nak_angle,
        360.0
    );
    let nak_end_ms = n_end_jd.map(|jd| (jd - 2440587.5) * 86_400_000.0);

    // Yoga at Sunrise
    let y_info = yoga::calculate_yoga(sunrise_jd, mode);
    let y_idx = y_info.index;
    let yoga_len = 360.0 / 27.0;
    let target_yoga_angle = (y_idx as f64) * yoga_len;
    
    let y_end_jd = solver::find_crossing_time(
        |jd| {
            use crate::astronomy::planets;
            let sun = planets::get_planet_position_sidereal(PlanetId::Sun, jd, ayanamsha::get_ayanamsha(mode, jd));
            let moon = planets::get_planet_position_sidereal(PlanetId::Moon, jd, ayanamsha::get_ayanamsha(mode, jd));
            let sum = moon.longitude + sun.longitude;
            sum % 360.0
        },
        sunrise_jd,
        search_end_jd,
        target_yoga_angle, 
        360.0
    );

    let yoga_end_ms = y_end_jd.map(|jd| (jd - 2440587.5) * 86_400_000.0);

    // Vara (Weekday)
    let v_info = vara::calculate_vara(sunrise_jd);
    let weekday_idx = ((sunrise_jd + 1.5).floor() as i64 % 7) as u8;
    
    // Muhurats
    let sunset_ms = calculate_sunset(year, month, day, location);
    let muhurats = muhurat::calculate_muhurats(sunrise_ms, sunset_ms, weekday_idx);

    Ok(DailyPanchang {
        sunrise: sunrise_ms,
        sunset: sunset_ms,
        
        tithi_index: tithi_idx as u8,
        tithi_name: t_info.name,
        tithi_end_time: tithi_end_ms,
        
        nakshatra_index: n_idx as u8,
        nakshatra_name: n_info.name,
        nakshatra_end_time: nak_end_ms,
        
        yoga_index: y_idx as u8,
        yoga_name: y_info.name,
        yoga_end_time: yoga_end_ms,
        
        vara_name: v_info.name,
        
        ayanamsha_value: ayan_val,
        muhurats: muhurats,
    })
}
