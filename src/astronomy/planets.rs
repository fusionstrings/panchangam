use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use alloc::string::ToString;
use crate::swe_bindings;

/// Planet position data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct PlanetData {
    pub id: i32,
    pub name: alloc::string::String,
    pub longitude: f64,
    pub latitude: f64,
    pub distance: f64,
    pub speed: f64,
    pub is_retrograde: bool,
    pub is_combust: bool,
    pub dignity: crate::vedic::dignity::Dignity,
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
/// Calculate Julian Day from calendar date
pub fn julian_day(year: i32, month: u32, day: u32, hour: f64) -> f64 {
    swiss_eph::safe::julday(year, month as i32, day as i32, hour)
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
        let planet = match id {
            0 => swiss_eph::safe::Planet::Sun,
            1 => swiss_eph::safe::Planet::Moon,
            2 => swiss_eph::safe::Planet::Mercury,
            3 => swiss_eph::safe::Planet::Venus,
            4 => swiss_eph::safe::Planet::Mars,
            5 => swiss_eph::safe::Planet::Jupiter,
            6 => swiss_eph::safe::Planet::Saturn,
            10 => swiss_eph::safe::Planet::MeanNode,
            _ => swiss_eph::safe::Planet::Sun, // Should not happen
        };

        let flags = swiss_eph::safe::CalcFlags::new()
            .with_swiss_ephemeris()
            .with_speed();
        
        let pos = swiss_eph::safe::calc(jd, planet, flags)
             .unwrap_or(swiss_eph::safe::Position {
                 longitude: 0.0, latitude: 0.0, distance: 0.0,
                 longitude_speed: 0.0, latitude_speed: 0.0, distance_speed: 0.0 
             });
        
        // Note: calc returns Tropical usually. If we want Sidereal we subtract ayanamsha.
        // Existing code: `xx[0] - ayanamsha_val`.
        // So we just take pos.longitude.
        
        let mut sid_lon = pos.longitude - ayanamsha_val;
        if sid_lon < 0.0 { sid_lon += 360.0; }
        
        raw_positions.push((id, name, sid_lon % 360.0, pos.latitude, pos.distance, pos.longitude_speed));
    }

    let sun_lon = raw_positions[0].2;

    for (id, name, lon, lat, dist, speed) in raw_positions {
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

        let dignity = crate::vedic::dignity::calculate_dignity(name, lon);

        planets.push(PlanetData {
            id,
            name: name.to_string(),
            longitude: lon,
            latitude: lat,
            distance: dist,
            speed: speed,
            is_retrograde: is_retro,
            is_combust,
            dignity,
        });
    }

    // Add Ketu (Rahu + 180)
    let rahu_idx = planets.len() - 1;
    let rahu_lon = planets[rahu_idx].longitude;
    let ketu_lon = (rahu_lon + 180.0) % 360.0;
    planets.push(PlanetData {
        id: 11,
        name: "Ketu".to_string(),
        longitude: ketu_lon,
        latitude: -planets[rahu_idx].latitude,
        distance: planets[rahu_idx].distance,
        speed: planets[rahu_idx].speed,
        is_retrograde: planets[rahu_idx].is_retrograde,
        is_combust: false,
        dignity: crate::vedic::dignity::Dignity::Neutral,
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

pub fn get_planet_position_sidereal(planet_id: PlanetId, jd: f64, ayanamsha_val: f64) -> PlanetPos {
    let planet = match planet_id {
        PlanetId::Sun => swiss_eph::safe::Planet::Sun,
        PlanetId::Moon => swiss_eph::safe::Planet::Moon,
        PlanetId::Mercury => swiss_eph::safe::Planet::Mercury,
        PlanetId::Venus => swiss_eph::safe::Planet::Venus,
        PlanetId::Mars => swiss_eph::safe::Planet::Mars,
        PlanetId::Jupiter => swiss_eph::safe::Planet::Jupiter,
        PlanetId::Saturn => swiss_eph::safe::Planet::Saturn,
        PlanetId::Rahu => swiss_eph::safe::Planet::MeanNode,
    };
    
    let flags = swiss_eph::safe::CalcFlags::new()
        .with_swiss_ephemeris()
        .with_speed();
    
    let pos = swiss_eph::safe::calc(jd, planet, flags)
         .unwrap_or(swiss_eph::safe::Position {
             longitude: 0.0, latitude: 0.0, distance: 0.0,
             longitude_speed: 0.0, latitude_speed: 0.0, distance_speed: 0.0 
         });
    
    let mut sid_lon = pos.longitude - ayanamsha_val;
    if sid_lon < 0.0 { sid_lon += 360.0; }
    
    PlanetPos {
        longitude: sid_lon,
        latitude: pos.latitude,
        distance: pos.distance,
        speed: pos.longitude_speed,
    }
}


/// Calculate planetary position returning longitude
fn calculate_planet(jd: f64, planet_id: i32) -> f64 {
    // Note: planet_id here comes from swe_bindings constants used in sun/moon_longitude below
    // We should map it or use raw `calc_ut` if we want to support any ID?
    // Using simple mapping:
    let planet = match planet_id {
        0 => swiss_eph::safe::Planet::Sun,
        1 => swiss_eph::safe::Planet::Moon,
        _ => swiss_eph::safe::Planet::Sun, // Fallback
    };

    let flags = swiss_eph::safe::CalcFlags::new()
        .with_swiss_ephemeris()
        .with_speed();
    
    let pos = swiss_eph::safe::calc(jd, planet, flags)
        .unwrap_or(swiss_eph::safe::Position {
             longitude: 0.0, latitude: 0.0, distance: 0.0,
             longitude_speed: 0.0, latitude_speed: 0.0, distance_speed: 0.0 
        });
    
    let mut long = pos.longitude;
    if long < 0.0 { long += 360.0; }
    long % 360.0
}

/// Calculate planet magnitude (brightness)
pub fn get_planet_magnitude(planet: PlanetId, jd: f64) -> f64 {
    let swe_planet = match planet {
        PlanetId::Sun => swiss_eph::safe::Planet::Sun,
        PlanetId::Moon => swiss_eph::safe::Planet::Moon,
        PlanetId::Mercury => swiss_eph::safe::Planet::Mercury,
        PlanetId::Venus => swiss_eph::safe::Planet::Venus,
        PlanetId::Mars => swiss_eph::safe::Planet::Mars,
        PlanetId::Jupiter => swiss_eph::safe::Planet::Jupiter,
        PlanetId::Saturn => swiss_eph::safe::Planet::Saturn,
        PlanetId::Rahu => swiss_eph::safe::Planet::MeanNode,
    };
    
    let flags = swiss_eph::safe::CalcFlags::new()
        .with_swiss_ephemeris();
    
    let pheno = swiss_eph::safe::phenomena(jd, swe_planet, flags);
    
    match pheno {
        Ok(p) => p.magnitude,
        Err(_) => 0.0,
    }
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
    let flags = swiss_eph::safe::CalcFlags::new().with_swiss_ephemeris();
    let pheno = swiss_eph::safe::phenomena(jd, swiss_eph::safe::Planet::Moon, flags);
    
    match pheno {
        Ok(p) => p.phase,
        Err(_) => 0.0,
    }
}
