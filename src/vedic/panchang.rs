use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use crate::{Location, calculate_sunrise, calculate_sunset};
use crate::vedic::{tithi, nakshatra, yoga, vara, muhurat, karana};
use crate::astronomy::ayanamsha;
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
    /// Timestamp (Unix ms) when this Tithi started.
    pub tithi_start_time: Option<f64>,
    /// Timestamp (Unix ms) when this Tithi ends. None if it doesn't end today.
    pub tithi_end_time: Option<f64>,

    // Nakshatra
    /// Nakshatra index (1-27). 1 = Ashwini.
    pub nakshatra_index: u8,
    /// Name of the Nakshatra (e.g., "Krittika")
    #[wasm_bindgen(getter_with_clone)]
    pub nakshatra_name: String,
    /// Timestamp (Unix ms) when this Nakshatra started.
    pub nakshatra_start_time: Option<f64>,
    /// Timestamp (Unix ms) when this Nakshatra ends.
    pub nakshatra_end_time: Option<f64>,
    
    // Yoga
    /// Yoga index (1-27).
    pub yoga_index: u8,
    /// Name of the Yoga (e.g., "Vishkumbha")
    #[wasm_bindgen(getter_with_clone)]
    pub yoga_name: String,
    /// Timestamp (Unix ms) when this Yoga started.
    pub yoga_start_time: Option<f64>,
    /// Timestamp (Unix ms) when this Yoga ends.
    pub yoga_end_time: Option<f64>,

    // Karana
    /// Karana index (1-60).
    pub karana_index: u8,
    /// Name of the Karana (e.g., "Bava")
    #[wasm_bindgen(getter_with_clone)]
    pub karana_name: String,
    /// Timestamp (Unix ms) when this Karana started.
    pub karana_start_time: Option<f64>,
    /// Timestamp (Unix ms) when this Karana ends.
    pub karana_end_time: Option<f64>,

    // Vara
    /// Solar Weekday name (e.g., "Adityawara")
    #[wasm_bindgen(getter_with_clone)]
    pub vara_name: String,
    
    // Houses
    /// Sidereal Ascendant (Lagna) at sunrise (degrees)
    pub ascendant: f64,
    /// Sidereal Midheaven (MC) at sunrise (degrees)
    pub mc: f64,
    
    // Planets
    /// Array of planetary positions at sunrise
    #[wasm_bindgen(skip)]
    pub planets: alloc::vec::Vec<crate::astronomy::planets::PlanetData>,
    
    // Config
    /// The Ayanamsha value (in degrees) used for calculations
    pub ayanamsha_value: f64,
    
    // Muhurat
    /// Auspicious and Inauspicious time periods for the day
    #[wasm_bindgen(getter_with_clone)]
    pub muhurats: muhurat::DayMuhurats,
}

#[wasm_bindgen]
impl DailyPanchang {
    #[wasm_bindgen(getter)]
    pub fn planets(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.planets).unwrap_or(JsValue::NULL)
    }
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
        0 => AyanamshaMode::FaganBradley,
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
    
    // Tithi Times
    let tithi_start_jd = tithi::tithi_start_time(sunrise_jd);
    let t_start_ms = Some((tithi_start_jd - 2440587.5) * 86_400_000.0);
    
    let tithi_end_jd = tithi::tithi_end_time(sunrise_jd);
    let t_end_ms = Some((tithi_end_jd - 2440587.5) * 86_400_000.0);

    // Nakshatra Times
    let n_info = nakshatra::calculate_nakshatra(sunrise_jd, mode);
    let n_idx = n_info.index;
    
    let nak_start_jd = nakshatra::nakshatra_start_time(sunrise_jd, mode);
    let n_start_ms = Some((nak_start_jd - 2440587.5) * 86_400_000.0);
    
    let nak_end_jd = nakshatra::nakshatra_end_time(sunrise_jd, mode);
    let n_end_ms = Some((nak_end_jd - 2440587.5) * 86_400_000.0);

    // Yoga Times
    let y_info = yoga::calculate_yoga(sunrise_jd, mode);
    let y_idx = y_info.index;
    
    let yoga_start_jd = yoga::yoga_start_time(sunrise_jd, mode);
    let y_start_ms = Some((yoga_start_jd - 2440587.5) * 86_400_000.0);
    
    let yoga_end_jd = yoga::yoga_end_time(sunrise_jd, mode);
    let y_end_ms = Some((yoga_end_jd - 2440587.5) * 86_400_000.0);

    // Karana Times
    let k_info = karana::calculate_karana(sunrise_jd);
    
    let karana_start_jd = karana::karana_start_time(sunrise_jd);
    let k_start_ms = Some((karana_start_jd - 2440587.5) * 86_400_000.0);
    
    let karana_end_jd = karana::karana_end_time(sunrise_jd);
    let k_end_ms = Some((karana_end_jd - 2440587.5) * 86_400_000.0);

    // Vara (Weekday)
    let v_info = vara::calculate_vara(sunrise_jd);
    let weekday_idx = ((sunrise_jd + 1.5).floor() as i64 % 7) as u8;
    
    // Muhurats
    let sunset_ms = calculate_sunset(year, month, day, location);
    let muhurats = muhurat::calculate_muhurats(sunrise_ms, sunset_ms, weekday_idx);

    // Houses at Sunrise
    let h_info = crate::astronomy::houses::calculate_houses(
        sunrise_jd,
        location.latitude,
        location.longitude,
        'P',
        Some(mode)
    )?;

    // Planets at Sunrise (Vec<PlanetData>)
    let p_list = crate::astronomy::planets::get_planet_positions_bulk(sunrise_jd, ayan_val);

    Ok(DailyPanchang {
        sunrise: sunrise_ms,
        sunset: sunset_ms,
        
        tithi_index: tithi_idx as u8,
        tithi_name: t_info.name,
        tithi_start_time: t_start_ms,
        tithi_end_time: t_end_ms,
        
        nakshatra_index: n_idx as u8,
        nakshatra_name: n_info.name,
        nakshatra_start_time: n_start_ms,
        nakshatra_end_time: n_end_ms,
        
        yoga_index: y_idx as u8,
        yoga_name: y_info.name,
        yoga_start_time: y_start_ms,
        yoga_end_time: y_end_ms,
        
        karana_index: k_info.index,
        karana_name: k_info.name,
        karana_start_time: k_start_ms,
        karana_end_time: k_end_ms,
        
        vara_name: v_info.name,
        
        ascendant: h_info.ascendant,
        mc: h_info.mc,
        
        planets: p_list,
        
        ayanamsha_value: ayan_val,
        muhurats: muhurats,
    })
}
