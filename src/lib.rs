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
use swiss_eph; // Ensure it's linked
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::vec;



use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

// Include generated Swiss Ephemeris bindings
// Re-export bindings from swiss-eph
pub(crate) use swiss_eph as swe_bindings;

// Re-export Varga types for JS
pub use vedic::vargas::{VargaType, VargaConfig, VargaPosition, D2Variation, D3Variation, D9Variation, D10Variation};
// Re-export Shadbala/Jaimini types
pub use vedic::shadbala::{ShadbalaResult, ShadbalaProfile};
pub use vedic::jaimini::{KarakaObject, KarakaName, JaiminiProfile, CharaDashaPeriod};
pub use vedic::ashtakavarga::{AshtakavargaResult, Sarvashtakavarga, ReducedAshtakavarga, PrastaraResult};
pub use vedic::special_lagnas::SpecialLagnas;



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

/// Calculate Yogini Dasha details
#[wasm_bindgen]
pub fn calculate_yogini(moon_long: f64, birth_time_ms: f64, current_time_ms: f64) -> vedic::dasha::YoginiInfo {
    vedic::dasha::calculate_yogini(moon_long, birth_time_ms, current_time_ms)
}

/// Calculate specific Varga position
/// Calculate specific Varga position
/// Calculate specific Varga position
#[wasm_bindgen]
pub fn calculate_varga(long: f64, varga_val: i32, config: JsValue) -> Result<VargaPosition, JsValue> {
    let v_type = match varga_val {
        1 => VargaType::D1,
        2 => VargaType::D2,
        3 => VargaType::D3,
        4 => VargaType::D4,
        7 => VargaType::D7,
        9 => VargaType::D9,
        10 => VargaType::D10,
        12 => VargaType::D12,
        16 => VargaType::D16,
        20 => VargaType::D20,
        24 => VargaType::D24,
        27 => VargaType::D27,
        30 => VargaType::D30,
        40 => VargaType::D40,
        45 => VargaType::D45,
        60 => VargaType::D60,
        _ => return Err(JsValue::from_str("Invalid Varga ID")),
    };
    
    let default_conf = VargaConfig::new();
    let conf: VargaConfig = if config.is_undefined() || config.is_null() {
        default_conf
    } else {
        #[derive(Deserialize)]
        struct PartialConfig {
            #[serde(default)]
            d2_method: Option<i32>,
            #[serde(default)]
            d3_method: Option<i32>,
            #[serde(default)]
            d9_method: Option<i32>,
            #[serde(default)]
            d10_method: Option<i32>,
        }
        
        match serde_wasm_bindgen::from_value::<PartialConfig>(config) {
            Ok(p) => VargaConfig {
                d2_method: p.d2_method.unwrap_or(default_conf.d2_method),
                d3_method: p.d3_method.unwrap_or(default_conf.d3_method),
                d9_method: p.d9_method.unwrap_or(default_conf.d9_method),
                d10_method: p.d10_method.unwrap_or(default_conf.d10_method),
            },
            Err(e) => return Err(JsValue::from_str(&alloc::format!("Invalid config object: {}", e))),
        }
    };
    
    Ok(vedic::vargas::calculate_varga_position(long, v_type, &conf))
}

/// Calculate Shadbala for a single planet (Stub)
#[wasm_bindgen]
pub fn calculate_planet_strength(
    long: f64,
    planet_id: i32,
    jd: f64,
    ascendant: f64
) -> ShadbalaResult {
    vedic::shadbala::calculate_planet_shadbala(long, planet_id, jd, ascendant)
}

/// Calculate Full Shadbala Profile (All 7 planets)
#[wasm_bindgen]
pub fn calculate_full_shadbala(
    planet_longs: &JsValue, 
    jd: f64, 
    ascendant: f64
) -> Result<ShadbalaProfile, JsValue> {
    let data: Vec<vedic::shadbala::PlanetInput> = serde_wasm_bindgen::from_value(planet_longs.clone())?;
    Ok(vedic::shadbala::calculate_shadbala_profile(&data, jd, ascendant))
}

/// Calculate Jaimini Karakas
#[wasm_bindgen]
pub fn calculate_jaimini_karakas(
    planet_longs: &JsValue, // Array of {id, long} objects? Or flat array?
    use_8_karakas: bool
) -> Result<Box<[KarakaObject]>, JsValue> {
    // Parse input array of tuples/objects
    // For simplicity, accept Float64Array of longitudes indexed by planet ID [0..8]? 
    // Or accept flexible array.
    // Let's use simple deserialization of `Vec<(i32, f64)>`?
    
    let data: Vec<(i32, f64)> = serde_wasm_bindgen::from_value(planet_longs.clone())?;
    
    let karakas = vedic::jaimini::calculate_charakarakas(&data, use_8_karakas);
    Ok(karakas.into_boxed_slice())
}

/// Calculate Jaimini Chara Dasha Periods
#[wasm_bindgen]
pub fn calculate_chara_dasha_periods(
    planet_longs: &JsValue,
    ascendant_sign: i32,
    start_year: f64
) -> Result<Box<[CharaDashaPeriod]>, JsValue> {
    // Parse as tuples (id, f64)
    let data: Vec<(i32, f64)> = serde_wasm_bindgen::from_value(planet_longs.clone())?;
    
    // Ensure all 9 planets for best results
    let periods = vedic::jaimini::calculate_chara_dasha(&data, ascendant_sign as usize, start_year);
    Ok(periods.into_boxed_slice())
}

/// Calculate Sarvashtakavarga (Ashtakavarga Totals)

/// Calculate Sarvashtakavarga (Ashtakavarga Totals)
#[wasm_bindgen]
pub fn calculate_ashtakavarga(
    planet_longs: &JsValue, // Expecting 7 planets longitudes
    ascendant: f64
) -> Result<Sarvashtakavarga, JsValue> {
    // Parse input. List of longitudes? 
    // Or simpler: array of f64.
    // The previous Shadbala used PlanetInput objects.
    // We can reuse that or accept simpler [f64] array if user passes just longitudes.
    // Consistent with Shadbala, let's accept `Vec<PlanetInput>` or `Vec<f64>`.
    // Wait, the input logic needs to be robust. 
    // Let's accept the SAME structure as Shadbala for consistency: Array of objects.
    // We extract longitudes for id 0..6.
    
    let data: Vec<vedic::shadbala::PlanetInput> = serde_wasm_bindgen::from_value(planet_longs.clone())?;
    
    // Sort or map 0..6
    let mut longs = vec![0.0; 7];
    for p in data {
        if p.id >= 0 && p.id <= 6 {
            longs[p.id as usize] = p.longitude;
        }
    }
    
    Ok(vedic::ashtakavarga::calculate_sarvashtakavarga(&longs, ascendant))
}

/// Calculate Ashtakavarga Reductions (Trikona & Ekadhipatya)
#[wasm_bindgen]
pub fn calculate_reduced_ashtakavarga(
    bindus: &JsValue, // Int32Array or [numbers]
    planet_longs: &JsValue // PlanetInput array
) -> Result<ReducedAshtakavarga, JsValue> {
    let bindus_vec: Vec<i32> = serde_wasm_bindgen::from_value(bindus.clone())?;
    
    if bindus_vec.len() != 12 {
        return Err(JsValue::from_str("Bindus array must have 12 elements."));
    }
    
    let data: Vec<(i32, f64)> = serde_wasm_bindgen::from_value(planet_longs.clone())?;
    
    // We pass the data directly
    Ok(vedic::ashtakavarga::calculate_reductions(&bindus_vec, &data))
}

/// Calculate Prastara Ashtakavarga (Detailed Grid)
#[wasm_bindgen]
pub fn calculate_prastara_ashtakavarga(
    planet_id: i32,
    planet_longs: &JsValue,
    ascendant: f64
) -> Result<PrastaraResult, JsValue> {
    let data: Vec<vedic::shadbala::PlanetInput> = serde_wasm_bindgen::from_value(planet_longs.clone())?;
    let mut longs = vec![0.0; 7];
    for p in data {
        if p.id >= 0 && p.id <= 6 {
            longs[p.id as usize] = p.longitude;
        }
    }
    Ok(vedic::ashtakavarga::calculate_prastara_av(planet_id, &longs, ascendant))
}

/// Calculate Special Lagnas (Hora, Ghati, Sree Lagna)
#[wasm_bindgen]
pub fn calculate_special_lagnas(
    birth_jd: f64,
    sunrise_jd: f64,
    sunrise_sun_long: f64,
    lagna_long: f64,
    moon_long: f64
) -> SpecialLagnas {
    vedic::special_lagnas::calculate_special_lagnas(birth_jd, sunrise_jd, sunrise_sun_long, lagna_long, moon_long)
}


// Re-export Yoga types
pub use vedic::yogas::YogaResult;

/// Find active Yogas (Planetary Combinations)
#[wasm_bindgen]
pub fn find_active_yogas(
    planet_longs: &JsValue,
    ascendant: f64
) -> Result<Box<[YogaResult]>, JsValue> {
    let data: Vec<vedic::shadbala::PlanetInput> = serde_wasm_bindgen::from_value(planet_longs.clone())?;
    
    // Convert to slice
    let yogas = vedic::yogas::check_yogas(&data, ascendant);
    
    Ok(yogas.into_boxed_slice())
}

/// Calculate Julian Day number
/// 
/// # Arguments
/// * `year` - Year
/// * `month` - Month
/// * `day` - Day
/// * `hour` - Hour
/// * `gregflag` - Calendar flag (1 = Gregorian, 0 = Julian)
#[wasm_bindgen]
pub fn p_julday(year: i32, month: i32, day: i32, hour: f64, gregflag: i32) -> f64 {
    unsafe {
        swe_bindings::swe_julday(year, month, day, hour, gregflag)
    }
}



#[derive(Serialize)]
pub struct PlanetaryPosition {
    pub longitude: f64,
    pub latitude: f64,
    pub distance: f64,
    pub speed_long: f64,
    pub speed_lat: f64,
    pub speed_dist: f64,
}

/// Calculate planetary position (UT)
/// 
/// Returns simple struct with longitude, latitude, distance, speed values.
#[wasm_bindgen]
pub fn p_calc_ut(tjd_ut: f64, ipl: i32, iflag: i32) -> Result<JsValue, JsValue> {
    let mut xx = [0.0; 6];
    let mut serr = [0i8; 256];
    unsafe {
        let ret_flag = swe_bindings::swe_calc_ut(tjd_ut, ipl, iflag, xx.as_mut_ptr(), serr.as_mut_ptr());
        if ret_flag < 0 {
             let c_str = core::ffi::CStr::from_ptr(serr.as_ptr());
             return Err(JsValue::from_str(c_str.to_str().unwrap_or("Unknown error")));
        }
    }

    let result = PlanetaryPosition {
        longitude: xx[0],
        latitude: xx[1],
        distance: xx[2],
        speed_long: xx[3],
        speed_lat: xx[4],
        speed_dist: xx[5],
    };

    Ok(serde_wasm_bindgen::to_value(&result)?)
}
