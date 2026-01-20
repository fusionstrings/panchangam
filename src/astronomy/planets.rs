use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use alloc::string::ToString;
use crate::swe_bindings;

/// Planet position data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct PlanetData {
    pub name: alloc::string::String,
    pub longitude: f64,
    pub latitude: f64,
    pub distance: f64,
    pub speed: f64,
    pub is_retrograde: bool,
    pub is_combust: bool,
}

#[derive(Clone, Copy)]
pub enum PlanetId {
    Sun = 0,
    Moon = 1,
    Mercury = 2,
    Venus = 3, 
    Mars = 4,
    Jupiter = 5,
    Saturn = 6,
    Rahu = 10, // Mean Node
}

/// Calculate Julian Day from calendar date
pub fn julian_day(year: i32, month: u32, day: u32, hour: f64) -> f64 {
    unsafe {
        crate::swe_bindings::swe_julday(year, month as i32, day as i32, hour, crate::swe_bindings::SE_GREG_CAL as i32)
    }
}

pub fn get_planet_positions_bulk(jd: f64, ayanamsha_val: f64) -> alloc::vec::Vec<PlanetData> {
    let mut planets = alloc::vec::Vec::new();
    
    // Core 7 planets + Mean Rahu (10)
    let planet_ids = [
        (0, "Sun"), (1, "Moon"), (2, "Mercury"), (3, "Venus"), 
        (4, "Mars"), (5, "Jupiter"), (6, "Saturn"), (10, "Rahu")
    ];

    // First pass: get all raw data
    let mut raw_positions = alloc::vec::Vec::new();
    for (id, name) in planet_ids {
        let mut xx = [0.0; 6];
        let mut serr = [0i8; 256];
        let flags = 2 | 256; // SEFLG_SWIEPH | SEFLG_SPEED
        
        unsafe {
            crate::swe_bindings::swe_calc_ut(jd, id, flags, xx.as_mut_ptr(), serr.as_mut_ptr());
        }
        
        let mut sid_lon = xx[0] - ayanamsha_val;
        if sid_lon < 0.0 { sid_lon += 360.0; }
        
        raw_positions.push((name, sid_lon % 360.0, xx[1], xx[2], xx[3]));
    }

    let sun_lon = raw_positions[0].1;

    for (name, lon, lat, dist, speed) in raw_positions {
        let is_retro = speed < 0.0;
        
        // Combustion logic
        let mut diff = (lon - sun_lon).abs();
        if diff > 180.0 { diff = 360.0 - diff; }
        
        let is_combust = match name {
            "Moon" => diff < 12.0,
            "Mars" => diff < 17.0,
            "Mercury" => if is_retro { diff < 12.0 } else { diff < 14.0 },
            "Jupiter" => diff < 11.0,
            "Venus" => if is_retro { diff < 8.0 } else { diff < 10.0 },
            "Saturn" => diff < 15.0,
            _ => false,
        };

        planets.push(PlanetData {
            name: name.to_string(),
            longitude: lon,
            latitude: lat,
            distance: dist,
            speed: speed,
            is_retrograde: is_retro,
            is_combust,
        });
    }

    // Add Ketu (Rahu + 180)
    let rahu_idx = planets.len() - 1;
    let rahu_lon = planets[rahu_idx].longitude;
    let ketu_lon = (rahu_lon + 180.0) % 360.0;
    planets.push(PlanetData {
        name: "Ketu".to_string(),
        longitude: ketu_lon,
        latitude: -planets[rahu_idx].latitude,
        distance: planets[rahu_idx].distance,
        speed: planets[rahu_idx].speed,
        is_retrograde: planets[rahu_idx].is_retrograde,
        is_combust: false,
    });

    planets
}

#[derive(Debug, Clone, Copy)]
pub struct PlanetPos {
    pub longitude: f64,
    pub latitude: f64,
    pub distance: f64,
    pub speed: f64,
}

pub fn get_planet_position_sidereal(planet: PlanetId, jd: f64, ayanamsha_val: f64) -> PlanetPos {
    let mut xx = [0.0; 6];
    let mut serr = [0i8; 256];
    let flags = swe_bindings::SEFLG_SWIEPH | swe_bindings::SEFLG_SPEED;
    
    unsafe {
        swe_bindings::swe_calc_ut(jd, planet as i32, flags as i32, xx.as_mut_ptr(), serr.as_mut_ptr());
    }
    
    let mut sid_lon = xx[0] - ayanamsha_val;
    if sid_lon < 0.0 { sid_lon += 360.0; }
    
    PlanetPos {
        longitude: sid_lon,
        latitude: xx[1],
        distance: xx[2],
        speed: xx[3],
    }
}


/// Calculate planetary position returning longitude
fn calculate_planet(jd: f64, planet_id: i32) -> f64 {
    let mut xx = [0.0; 6];
    let mut serr = [0i8; 256];
    let flags = swe_bindings::SEFLG_SWIEPH | swe_bindings::SEFLG_SPEED;
    
    unsafe {
        swe_bindings::swe_calc_ut(jd, planet_id, flags as i32, xx.as_mut_ptr(), serr.as_mut_ptr());
    }
    
    // xx[0] is longitude
    let mut long = xx[0];
    if long < 0.0 { long += 360.0; }
    long % 360.0
}

/// Calculate planet magnitude (brightness)
pub fn get_planet_magnitude(planet: PlanetId, jd: f64) -> f64 {
    let mut attr = [0.0; 20];
    let mut serr = [0i8; 256];
    let flags = swe_bindings::SEFLG_SWIEPH; // No speed needed for magnitude
    
    unsafe {
        swe_bindings::swe_pheno_ut(jd, planet as i32, flags as i32, attr.as_mut_ptr(), serr.as_mut_ptr());
    }
    
    // attr[4] is visual magnitude
    attr[4]
}

/// Calculate Sun's geocentric ecliptic longitude (degrees)
pub fn sun_longitude(jd: f64) -> f64 {
    calculate_planet(jd, swe_bindings::SE_SUN as i32)
}

/// Calculate Moon's geocentric ecliptic longitude (degrees)
pub fn moon_longitude(jd: f64) -> f64 {
    calculate_planet(jd, swe_bindings::SE_MOON as i32)
}

/// Calculate Moon's illumination fraction (0.0 to 1.0)
pub fn moon_illumination(jd: f64) -> f64 {
    let mut xmas = [0.0; 6];
    let mut serr = [0i8; 256];
    
    unsafe {
        swe_bindings::swe_pheno_ut(jd, swe_bindings::SE_MOON as i32, swe_bindings::SEFLG_SWIEPH as i32, xmas.as_mut_ptr(), serr.as_mut_ptr());
    }
    
    // xmas[1] is phase (illumination fraction)
    xmas[1]
}
