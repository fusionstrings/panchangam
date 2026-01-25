//! Shadbala (Six-Fold Strength) System
//!
//! Calculates the six sources of strength for the 7 planets (Sun to Saturn).
//! Node (Rahu/Ketu) are generally not assigned Shadbala in traditional texts, though some variations exist.
//! We focus on Sun-Saturn.

use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use alloc::vec::Vec;
use alloc::vec;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetInput {
    pub id: i32,
    pub longitude: f64,
    pub speed: f64,
}

/// Detailed breakdown of Shadbala
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct ShadbalaResult {
    /// Total Shadbala in Rupas
    pub total_rupas: f64,
    /// Ishta Phala (0-60)
    pub ishta_phala: f64,
    /// Kashta Phala (0-60)
    pub kashta_phala: f64,
    
    // Breakdown
    pub sthana_bala: f64,
    pub dig_bala: f64,
    pub kala_bala: f64,
    pub chesta_bala: f64,
    pub naisargika_bala: f64,
    pub drik_bala: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct ShadbalaProfile {
    // Check if we can use a Map or fixed fields. For WASM, fixed fields or array is better.
    // We'll use a list for the 7 planets.
    pub sun: ShadbalaResult,
    pub moon: ShadbalaResult,
    pub mars: ShadbalaResult,
    pub mercury: ShadbalaResult,
    pub jupiter: ShadbalaResult,
    pub venus: ShadbalaResult,
    pub saturn: ShadbalaResult,
}

/// Constants for Naisargika Bala (Natural Strength) in Rupas
// Naisargika Bala constants are used inline below

// Helper: Get Deepest Exaltation degree
fn exaltation_point(planet_id: i32) -> f64 {
    match planet_id {
        0 => 10.0,   // Sun: Aries 10
        1 => 33.0,   // Moon: Taurus 3
        2 => 298.0,  // Mars: Capricorn 28
        3 => 165.0,  // Mercury: Virgo 15
        4 => 95.0,   // Jupiter: Cancer 5
        5 => 357.0,  // Venus: Pisces 27
        6 => 200.0,  // Saturn: Libra 20
        _ => 0.0,
    }
}

// 1. Sthana Bala Calculation
//    - Uchcha Bala (Exaltation)
//    - Saptavargaja Bala (Divisional)
//    - Ojhayugma Bala (Odd/Even)
//    - Kendra Bala (Angle)
//    - Drekkana Bala (Decanate)

fn calculate_uchcha_bala(long: f64, planet_id: i32) -> f64 {
    let deep_exalt = exaltation_point(planet_id);
    // Find distance from deep debilitation (which is 180 from exaltation).
    // Actually standard formula: Diff from Debilitation / 3.
    // If planet is at Exaltation, it has 60 Shashtiamsas (1 Rupa).
    // If at Debilitation (Exalt + 180), it has 0.
    
    let deep_deb = (deep_exalt + 180.0) % 360.0;
    

    
    // BUT! Uchcha Bala logic:
    // "Subtract the debilitation point from the planet. If result > 180, take 360 - res."
    // Max diff is 180.
    // Bala (Rupas) = Diff / 180.
    // Bala (Shashtiamsas) = Diff / 3.
    
    // We return Rupas. 0 to 1.
    // Wait, distance from *debilitation* gives the strength.
    // E.g. if at exaltation (180 from deb), strength is max.
    
    // Calculate arc distance from Debilitation
    let mut arc = (long - deep_deb).rem_euclid(360.0);
    if arc > 180.0 {
        arc = 360.0 - arc;
    }
    
    // Max arc is 180. 180 / 180 = 1.0 Rupa.
    // or arc / 3.0 gives Shashtiamsas.
    // We stick to Rupas for internal, convert for display? 
    // Usually Shadbala is summed in Shashtiamsas or Rupas. Let's use Shashtiamsas (0-60 scale) for precision, convert to Rupa (divide by 60) at end?
    // Let's use Shashtiamsas throughout as f64.
    
    arc / 3.0
}

fn calculate_saptavargaja_bala(
    planet_id: i32,
    planets: &[PlanetInput],
) -> f64 {
    use crate::vedic::vargas::{calculate_varga_position, VargaType, VargaConfig};
    use crate::vedic::dignity::{calculate_dignity, get_panchadha_relationship, Dignity};
    
    let mut total_bala = 0.0;
    let target_long = planets.iter().find(|p| p.id == planet_id).map(|p| p.longitude).unwrap_or(0.0);
    let target_sign = ((target_long / 30.0).floor() as u8 % 12) + 1;
    
    // The 7 Vargas for Shadbala: D1, D2, D3, D7, D9, D12, D30
    let vargas = [
        VargaType::D1, VargaType::D2, VargaType::D3,
        VargaType::D7, VargaType::D9, VargaType::D12, VargaType::D30,
    ];
    
    let config = VargaConfig::new();
    
    for varga_type in vargas.iter() {
        let v_pos = calculate_varga_position(target_long, *varga_type, &config);
        let v_sign = v_pos.sign; // 1-12
        
        // 1. Check direct dignity in this Varga sign
        // Note: For D1, we use full degree precision. 
        // For other Vargas, we usually treat them as a "sign" and check its Lord.
        // However, Moolatrikona and Exaltation in Shadbala are often handled by just Sign-Lord relationship
        // except in D1. 
        // But the "Moolatrikona" 45-point rule often applies if the planet's sign matches its Moolatrikona sign.
        
        // Let's get the lord of the varga sign
        let lord_id = match v_sign {
            1 | 8 => 2, // Mars
            2 | 7 => 5, // Venus
            3 | 6 => 3, // Mercury
            4 => 1,     // Moon
            5 => 0,     // Sun
            9 | 12 => 4,// Jupiter
            10 | 11 => 6,// Saturn
            _ => -1,
        };
        
        // Calculate relationship based on D1 positions
        // We need the lord's sign in D1
        let lord_data = planets.iter().find(|p| p.id == lord_id);
        let lord_sign_d1 = lord_data.map(|p| ((p.longitude / 30.0).floor() as u8 % 12) + 1).unwrap_or(0);
        
        // precise mapping for calculate_dignity
        let p_name = match planet_id {
            0 => "Sun",
            1 => "Moon",
            2 => "Mars",
            3 => "Mercury",
            4 => "Jupiter",
            5 => "Venus",
            6 => "Saturn",
            _ => "Sun",
        };
        
        let dig_status = calculate_dignity(p_name, v_pos.full_longitude);
        
        let strength = if dig_status == Dignity::Moolatrikona {
            45.0
        } else if dig_status == Dignity::OwnSign {
            30.0
        } else if planet_id == lord_id {
             // Fallback for OwnSign if distinct from Dignity check (should match)
             30.0
        } else {
            // Check Five-fold relationship with the lord in D1 positions
            let dignity = get_panchadha_relationship(planet_id, target_sign, lord_id, lord_sign_d1);
            match dignity {
                Dignity::GreatFriend => 22.5,
                Dignity::Friend => 15.0,
                Dignity::Neutral => 7.5,
                Dignity::Enemy => 3.75,
                Dignity::GreatEnemy => 1.875,
                _ => 15.0, // Should not happen
            }
        };
        
        total_bala += strength;
    }
    
    total_bala
}


fn calculate_sthana_bala(planet_id: i32, planets: &[PlanetInput], jd: f64) -> f64 {
    let long = planets.iter().find(|p| p.id == planet_id).map(|p| p.longitude).unwrap_or(0.0);
    let uchcha = calculate_uchcha_bala(long, planet_id);
    let saptavargaja = calculate_saptavargaja_bala(planet_id, planets);
    // ... others
    uchcha + saptavargaja 
    // TODO: Add Ojhayugma, Kendra, Drekkana
}

// 2. Dig Bala (Directional)
// Sun/Mars best in 10th (South) - MC
// Moon/Ven best in 4th (North) - IC
// Mer/Jup best in 1st (East) - Asc
// Sat best in 7th (West) - Dsc
fn calculate_dig_bala(long: f64, planet_id: i32, ascendant: f64) -> f64 {
    // Define powerful point
    let power_point = match planet_id {
        0 | 2 => (ascendant + 270.0) % 360.0, // 10th house is roughly Asc - 90 (or Asc + 270)
        1 | 5 => (ascendant + 90.0) % 360.0,  // 4th house
        3 | 4 => ascendant,                   // 1st house
        6 => (ascendant + 180.0) % 360.0,     // 7th house
        _ => 0.0,
    };
    
    // Arc dist from power point
    let mut arc = (long - power_point).rem_euclid(360.0);
    if arc > 180.0 {
        arc = 360.0 - arc;
    }
    
    // Max strength at point = 60 Shashtiamsas.
    // 0 at 180 deg away.
    // Formula: (180 - Arc) / 3.
    
    (180.0 - arc) / 3.0
}

// 3. Chesta Bala (Motion Strength)
// Simplification: based on speed.
// Retrograde: 60
// Anuvakra (Retrograde but slowing?): Let's just use Retrograde logic for MVP.
// Direct: 30?
// Sun/Moon: Chesta based on declination/position usually. 
// For MVP:
// If speed < 0 (Retrograde): 60.0
// If speed >= 0: 15.0 (Vikalala) or 30.0?
// Standard texts:
// Vakra (Retrograde): 60
// Anuvakra (Entering Retro?): 30
// Vikala (Stationary): 15
// Mandatara (Slow): 30
// Manda (avg): 15
// SeeGra/Sighratara (Fast): 45?
// 
// Simplified Logic:
// Speed < 0: 60.0
// Speed > 1.2 * Avg: 45.0
// Speed < 0.8 * Avg: 15.0
// Else: 30.0
fn calculate_chesta_bala(planet_id: i32, speed: f64) -> f64 {
    // Sun and Moon have different Chesta Bala rules (Ayana/Paksha based), 
    // but often assigned values or determined by other means.
    // For now, apply generic speed logic or return default.
    
    // Average speeds (deg/day)
    let avg_speed = match planet_id {
        0 => 0.98,
        1 => 13.17,
        2 => 0.52,
        3 => 1.38, // fast
        4 => 0.08,
        5 => 1.20, // fast
        6 => 0.03,
        _ => 1.0,
    };
    
    if speed < 0.0 {
        return 60.0; // Vakra
    }
    
    if speed.abs() < (0.1 * avg_speed) {
        return 15.0; // Vikala (Stationary)
    }
    
    if speed > 1.2 * avg_speed {
        return 45.0; // Sighra
    }
    
    if speed < 0.5 * avg_speed {
        return 15.0; // Manda
    }
    
    30.0 // Average
}


// 3. Drik Bala (Aspect Strength)
fn calculate_aspect_value(aspect_angle: f64, aspector_id: i32) -> f64 {
    let d = aspect_angle;
    let mut val = 0.0;

    // Standard Aspect (7th house emphasis)
    if d > 30.0 && d <= 60.0 {
        val = (d - 30.0) / 2.0;
    } else if d > 60.0 && d <= 90.0 {
        val = (d - 60.0) + 15.0;
    } else if d > 90.0 && d <= 120.0 {
        val = 45.0 - (d - 90.0) / 2.0;
    } else if d > 120.0 && d <= 150.0 {
        val = 30.0 - (d - 120.0);
    } else if d > 150.0 && d <= 180.0 {
        val = (d - 150.0) * 2.0;    
    } else if d > 180.0 && d <= 300.0 {
         // Standard is 0 beyond 180, except special aspects below
         val = 0.0;
    }

    // Special Aspects
    match aspector_id {
        2 => { // Mars (4th=90, 8th=210)
             // 4th House (60-120 range? Peak at 90)
             // Usually special aspect overrides or adds.
             // "Special aspect of Mars on 4th and 8th is full (60)"
             if (d - 90.0).abs() < 1e-1 { val = 60.0; } // Exact check impractical
             // Let's implement range boosts if needed? 
             // Classical text implies exact angle logic applies similarly or just "Full at 4th".
             // Simplified: If d approx 90 (+/- orb?), set 60.
             // Actually, usually algorithms use specific formulas for special aspects too.
             // Skipping complex continuous special formulas for now.
             // Using discrete check for "House" based aspect or sticking to standard if not peak?
             // Reverting to simpler logic: 
             // IF Mars and d=90 -> 60. If d=210 -> 60.
             
             // Improvement: Apply linear interpolation for special aspects? 
             // Standard practice: Add 15 to 4th house range?
             // Let's stick to Standard + Check if close to peak.
        },
        4 => { // Jupiter (5th=120, 9th=240)
             if (d - 120.0).abs() < 1e-1 { val = 60.0; }
             if (d - 240.0).abs() < 1e-1 { val = 60.0; }
        },
        6 => { // Saturn (3rd=60, 10th=270)
             if (d - 60.0).abs() < 1e-1 { val = 60.0; }
             if (d - 270.0).abs() < 1e-1 { val = 60.0; }
        },
        _ => {}
    }
    
    // override for peak values (simplified for this iteration)
    // NOTE: This is a simplified implementation. Full linear interpolation for special aspects requires more code.
    // We will assume "Full" means 60 at the exact/orb location.
    // For manual verification, standard 7th aspect calculation is critical.
    
    val
}

fn calculate_drik_bala(target_id: i32, target_long: f64, planets: &[PlanetInput]) -> f64 {
    let mut net_strength = 0.0;
    
    for p in planets.iter() {
        if p.id == target_id { continue; }
        
        // Angle from Aspector to Target
        // Target - Aspector
        let d = (target_long - p.longitude).rem_euclid(360.0);
        
        let val = calculate_aspect_value(d, p.id);
        
        // Benefic/Malefic
        // Natural Benefics: Jup(4), Ven(5). Mer(3) and Moon(1) conditional.
        // Natural Malefics: Sun(0), Mar(2), Sat(6). 
        // We stick to Natural for MVP.
        // Mer/Moon treated as Benefic for now (simplified).
        
        let is_benefic = matches!(p.id, 1 | 3 | 4 | 5);
        
        if is_benefic {
            net_strength += val;
        } else {
            net_strength -= val; // Malefics reduce strength? 
            // "Malefics give negative strength" -> reduces the planet's drik bala.
        }
    }
    
    net_strength / 4.0 // Quarter of net aspect value
}



/// Stub Function to Calculate Shadbala for one planet
/// Calculate Shadbala for all 7 planets
pub fn calculate_shadbala_profile(
    planets: &[PlanetInput], 
    jd: f64,
    ascendant: f64
) -> ShadbalaProfile {
    let mut results: Vec<ShadbalaResult> = Vec::with_capacity(7);
    
    // Sort or map? We need to ensure we return correct structure.
    // We assume 0..6 are present.
    
    for id in 0..7 {
        // Find planet's data
        let p_data = planets.iter().find(|p| p.id == id);
        let long = p_data.map(|p| p.longitude).unwrap_or(0.0);
        let speed = p_data.map(|p| p.speed).unwrap_or(0.0);
        
        let sthana = calculate_sthana_bala(id, planets, jd);
        let dig = calculate_dig_bala(long, id, ascendant);

        let kala = 45.0; // Placeholder
        let chesta = calculate_chesta_bala(id, speed);
        let naisargika = match id {
            0 => 60.0,       // Sun
            1 => 51.43,      // Moon
            2 => 17.14,      // Mars
            3 => 25.71,      // Mer
            4 => 34.28,      // Jup
            5 => 42.85,      // Ven
            6 => 8.57,       // Sat
            _ => 0.0,
        };
        let drik = calculate_drik_bala(id, long, planets);
        
        let total = sthana + dig + kala + chesta + naisargika + drik;
        
        results.push(ShadbalaResult {
            total_rupas: total / 60.0,
            ishta_phala: 0.0,
            kashta_phala: 0.0,
            sthana_bala: sthana,
            dig_bala: dig,
            kala_bala: kala,
            chesta_bala: chesta,
            naisargika_bala: naisargika,
            drik_bala: drik,
        });
    }
    
    ShadbalaProfile {
        sun: results[0],
        moon: results[1],
        mars: results[2],
        mercury: results[3],
        jupiter: results[4],
        venus: results[5],
        saturn: results[6],
    }
}

/// Stub Wrapper for single planet (Legacy support/Test)
pub fn calculate_planet_shadbala(
    long: f64,
    planet_id: i32,
    jd: f64,
    ascendant: f64
) -> ShadbalaResult {
    // We can't do full Drik without others. return partial.
    // Or mock others as 0? 0 pos?
    let planets = vec![PlanetInput { id: planet_id, longitude: long, speed: 1.0 }];
    let profile = calculate_shadbala_profile(&planets, jd, ascendant);
    
    // Return correct one
    match planet_id {
        0 => profile.sun,
        1 => profile.moon,
        2 => profile.mars,
        3 => profile.mercury,
        4 => profile.jupiter,
        5 => profile.venus,
        6 => profile.saturn,
        _ => profile.sun // fallback
    }
}
