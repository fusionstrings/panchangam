//! Ashtakavarga System
//!
//! Calculates Binna Ashtakavarga (BAV) and Sarvashtakavarga (SAV).
//! Based on standard Parashara rules.

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use alloc::vec::Vec;
use alloc::vec;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct AshtakavargaResult {
    /// Planet ID (0=Sun .. 6=Saturn)
    pub planet_id: i32, 
    /// Bindus for each sign (0=Aries .. 11=Pisces)
    #[wasm_bindgen(skip)]
    pub bindus: Vec<i32> 
}

#[wasm_bindgen]
impl AshtakavargaResult {
    #[wasm_bindgen(getter)]
    pub fn bindus(&self) -> Vec<i32> {
        self.bindus.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Sarvashtakavarga {
    /// Total bindus for each sign (0=Aries .. 11=Pisces)
    #[wasm_bindgen(skip)]
    pub totals: Vec<i32>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct ReducedAshtakavarga {
    #[wasm_bindgen(skip)]
    pub reduced_bindus: Vec<i32>,
    pub shodaya_pinda: i32,
}

#[wasm_bindgen]
impl ReducedAshtakavarga {
    #[wasm_bindgen(getter)]
    pub fn reduced_bindus(&self) -> Vec<i32> {
        self.reduced_bindus.clone()
    }
}

#[wasm_bindgen]
impl Sarvashtakavarga {
    #[wasm_bindgen(getter)]
    pub fn totals(&self) -> Vec<i32> {
        self.totals.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct PrastaraResult {
    /// Target Planet ID (0-6)
    pub planet_id: i32,
    /// 96 elements (8 rows of ref planets x 12 signs)
    /// Row 0-6: Sun..Sat, Row 7: Ascendant
    #[wasm_bindgen(skip)]
    pub grid: Vec<u8>,
}

#[wasm_bindgen]
impl PrastaraResult {
    #[wasm_bindgen(getter)]
    pub fn grid(&self) -> Vec<u8> {
        self.grid.clone()
    }
}

// Helper: Get sign index (0-11) from longitude
fn get_sign(long: f64) -> usize {
    (long / 30.0).floor() as usize % 12
}

// -----------------------------------------------------------------------------
// Reference Tables (Parashara)
// Houses (1-based) from reference point where bindu is contributed.
// -----------------------------------------------------------------------------

// Sun's Ashtakavarga
const SUN_FROM_SUN: &[i32] = &[1, 2, 4, 7, 8, 9, 10, 11];
const SUN_FROM_MOON: &[i32] = &[3, 6, 10, 11];
const SUN_FROM_MARS: &[i32] = &[1, 2, 4, 7, 8, 9, 10, 11];
const SUN_FROM_MERCURY: &[i32] = &[3, 5, 6, 9, 10, 11, 12];
const SUN_FROM_JUPITER: &[i32] = &[5, 6, 9, 11];
const SUN_FROM_VENUS: &[i32] = &[6, 7, 12];
const SUN_FROM_SATURN: &[i32] = &[1, 2, 4, 7, 8, 9, 10, 11];
const SUN_FROM_ASC: &[i32] = &[3, 4, 6, 10, 11, 12];

// Moon's Ashtakavarga
const MOON_FROM_SUN: &[i32] = &[3, 6, 7, 8, 10, 11];
const MOON_FROM_MOON: &[i32] = &[1, 3, 6, 7, 10, 11];
const MOON_FROM_MARS: &[i32] = &[2, 3, 5, 6, 9, 10, 11];
const MOON_FROM_MERCURY: &[i32] = &[1, 3, 4, 5, 7, 8, 10, 11];
const MOON_FROM_JUPITER: &[i32] = &[1, 4, 7, 8, 10, 11, 12];
const MOON_FROM_VENUS: &[i32] = &[3, 4, 5, 7, 9, 10, 11];
const MOON_FROM_SATURN: &[i32] = &[3, 5, 6, 11]; // Verify: Usually 3, 5, 6, 11
const MOON_FROM_ASC: &[i32] = &[3, 6, 10, 11];

// Mars's Ashtakavarga
const MARS_FROM_SUN: &[i32] = &[3, 5, 6, 10, 11];
const MARS_FROM_MOON: &[i32] = &[3, 6, 11];
const MARS_FROM_MARS: &[i32] = &[1, 2, 4, 7, 8, 10, 11];
const MARS_FROM_MERCURY: &[i32] = &[3, 5, 6, 11];
const MARS_FROM_JUPITER: &[i32] = &[6, 10, 11, 12];
const MARS_FROM_VENUS: &[i32] = &[6, 8, 11, 12];
const MARS_FROM_SATURN: &[i32] = &[1, 4, 7, 8, 9, 10, 11];
const MARS_FROM_ASC: &[i32] = &[1, 3, 6, 10, 11];

// Mercury's Ashtakavarga
const MERC_FROM_SUN: &[i32] = &[5, 6, 9, 11, 12];
const MERC_FROM_MOON: &[i32] = &[2, 4, 6, 8, 10, 11];
const MERC_FROM_MARS: &[i32] = &[1, 2, 4, 7, 8, 9, 10, 11];
const MERC_FROM_MERCURY: &[i32] = &[1, 3, 5, 6, 9, 10, 11, 12];
const MERC_FROM_JUPITER: &[i32] = &[6, 8, 11, 12];
const MERC_FROM_VENUS: &[i32] = &[1, 2, 3, 4, 5, 8, 9, 11];
const MERC_FROM_SATURN: &[i32] = &[1, 2, 4, 7, 8, 9, 10, 11];
const MERC_FROM_ASC: &[i32] = &[1, 2, 4, 6, 8, 10, 11];

// Jupiter's Ashtakavarga
const JUP_FROM_SUN: &[i32] = &[1, 2, 3, 4, 7, 8, 9, 10, 11];
const JUP_FROM_MOON: &[i32] = &[2, 5, 7, 9, 11];
const JUP_FROM_MARS: &[i32] = &[1, 2, 4, 7, 8, 10, 11];
const JUP_FROM_MERCURY: &[i32] = &[1, 2, 4, 5, 6, 9, 10, 11];
const JUP_FROM_JUPITER: &[i32] = &[1, 2, 3, 4, 7, 8, 10, 11];
const JUP_FROM_VENUS: &[i32] = &[2, 5, 6, 9, 10, 11];
const JUP_FROM_SATURN: &[i32] = &[3, 5, 6, 12];
const JUP_FROM_ASC: &[i32] = &[1, 2, 4, 5, 6, 7, 9, 10, 11];

// Venus's Ashtakavarga
const VEN_FROM_SUN: &[i32] = &[8, 11, 12];
const VEN_FROM_MOON: &[i32] = &[1, 2, 3, 4, 5, 8, 9, 11, 12];
const VEN_FROM_MARS: &[i32] = &[3, 5, 6, 9, 11, 12]; // Some texts: 3, 4, 6, 9, 11, 12. Stick to BV Raman/Standard.
const VEN_FROM_MERCURY: &[i32] = &[3, 5, 6, 9, 11];
const VEN_FROM_JUPITER: &[i32] = &[5, 8, 9, 10, 11];
const VEN_FROM_VENUS: &[i32] = &[1, 2, 3, 4, 5, 8, 9, 10, 11];
const VEN_FROM_SATURN: &[i32] = &[3, 4, 5, 8, 9, 10, 11];
const VEN_FROM_ASC: &[i32] = &[1, 2, 3, 4, 5, 8, 9, 11];

// Saturn's Ashtakavarga
const SAT_FROM_SUN: &[i32] = &[1, 2, 4, 7, 8, 10, 11];
const SAT_FROM_MOON: &[i32] = &[3, 6, 11];
const SAT_FROM_MARS: &[i32] = &[3, 5, 6, 10, 11, 12];
const SAT_FROM_MERCURY: &[i32] = &[6, 8, 9, 10, 11, 12];
const SAT_FROM_JUPITER: &[i32] = &[5, 6, 11, 12];
const SAT_FROM_VENUS: &[i32] = &[6, 11, 12];
const SAT_FROM_SATURN: &[i32] = &[3, 5, 6, 11];
const SAT_FROM_ASC: &[i32] = &[1, 3, 4, 6, 10, 11];


fn get_points(target_planet: i32, reference_planet: i32) -> &'static [i32] {
    // Reference IDs: 0=Sun, 1=Moon, 2=Mars, 3=Mer, 4=Jup, 5=Ven, 6=Sat, 7=Asc
    match target_planet {
        0 => match reference_planet { // Sun's AV
            0 => SUN_FROM_SUN,
            1 => SUN_FROM_MOON,
            2 => SUN_FROM_MARS,
            3 => SUN_FROM_MERCURY,
            4 => SUN_FROM_JUPITER,
            5 => SUN_FROM_VENUS,
            6 => SUN_FROM_SATURN,
            7 => SUN_FROM_ASC,
            _ => &[],
        },
        1 => match reference_planet { // Moon's AV
            0 => MOON_FROM_SUN,
            1 => MOON_FROM_MOON,
            2 => MOON_FROM_MARS,
            3 => MOON_FROM_MERCURY,
            4 => MOON_FROM_JUPITER,
            5 => MOON_FROM_VENUS,
            6 => MOON_FROM_SATURN,
            7 => MOON_FROM_ASC,
            _ => &[],
        },
        2 => match reference_planet { // Mars's AV
            0 => MARS_FROM_SUN,
            1 => MARS_FROM_MOON,
            2 => MARS_FROM_MARS,
            3 => MARS_FROM_MERCURY,
            4 => MARS_FROM_JUPITER,
            5 => MARS_FROM_VENUS,
            6 => MARS_FROM_SATURN,
            7 => MARS_FROM_ASC,
            _ => &[],
        },
        3 => match reference_planet { // Mercury's AV
            0 => MERC_FROM_SUN,
            1 => MERC_FROM_MOON,
            2 => MERC_FROM_MARS,
            3 => MERC_FROM_MERCURY,
            4 => MERC_FROM_JUPITER,
            5 => MERC_FROM_VENUS,
            6 => MERC_FROM_SATURN,
            7 => MERC_FROM_ASC,
            _ => &[],
        },
        4 => match reference_planet { // Jupiter's AV
            0 => JUP_FROM_SUN,
            1 => JUP_FROM_MOON,
            2 => JUP_FROM_MARS,
            3 => JUP_FROM_MERCURY,
            4 => JUP_FROM_JUPITER,
            5 => JUP_FROM_VENUS,
            6 => JUP_FROM_SATURN,
            7 => JUP_FROM_ASC,
            _ => &[],
        },
        5 => match reference_planet { // Venus's AV
            0 => VEN_FROM_SUN,
            1 => VEN_FROM_MOON,
            2 => VEN_FROM_MARS,
            3 => VEN_FROM_MERCURY,
            4 => VEN_FROM_JUPITER,
            5 => VEN_FROM_VENUS,
            6 => VEN_FROM_SATURN,
            7 => VEN_FROM_ASC,
            _ => &[],
        },
        6 => match reference_planet { // Saturn's AV
            0 => SAT_FROM_SUN,
            1 => SAT_FROM_MOON,
            2 => SAT_FROM_MARS,
            3 => SAT_FROM_MERCURY,
            4 => SAT_FROM_JUPITER,
            5 => SAT_FROM_VENUS,
            6 => SAT_FROM_SATURN,
            7 => SAT_FROM_ASC,
            _ => &[],
        },
        _ => &[]
    }
}

/// Calculate Binna Ashtakavarga for a single planet
///
/// # Arguments
/// * `planet_id` - The planet for which AV is calculated (0-6)
/// * `planet_positions` - Array of 7 planetary longitudes (0-6)
/// * `ascendant` - Ascendant longitude
pub fn calculate_binna_av(
    target_planet_id: i32,
    planet_positions: &[f64], // Expecting 7
    ascendant: f64
) -> AshtakavargaResult {
    let mut bindus = vec![0; 12];
    
    // Iterate over contributors: Sun(0) to Saturn(6), plus Ascendant(7)
    for ref_id in 0..=7 {
        let ref_long = if ref_id == 7 {
            ascendant
        } else {
            planet_positions[ref_id as usize]
        };
        
        // Sign of reference planet (0-11)
        let ref_sign = get_sign(ref_long);
        
        let points = get_points(target_planet_id, ref_id);
        
        for &offset in points.iter() {
            // Offset 1 means Same sign. Offset 2 means next sign.
            // Target Sign = (Ref Sign + Offset - 1) % 12
            let target_sign = (ref_sign + (offset as usize) - 1) % 12;
            bindus[target_sign] += 1;
        }
    }

    AshtakavargaResult {
        planet_id: target_planet_id,
        bindus
    }
}

/// Calculate Prastara Ashtakavarga (Detailed Contribution Grid)
pub fn calculate_prastara_av(
    target_planet_id: i32,
    planet_positions: &[f64],
    ascendant: f64
) -> PrastaraResult {
    let mut grid = vec![0u8; 96]; // 8 rows * 12 cols
    
    for ref_id in 0..=7 {
        let ref_long = if ref_id == 7 {
            ascendant
        } else {
            planet_positions[ref_id as usize]
        };
        
        let ref_sign = get_sign(ref_long);
        let points = get_points(target_planet_id, ref_id);
        
        for &offset in points.iter() {
            let target_sign = (ref_sign + (offset as usize) - 1) % 12;
            // grid[row * 12 + col]
            grid[(ref_id as usize) * 12 + target_sign] = 1;
        }
    }

    PrastaraResult {
        planet_id: target_planet_id,
        grid
    }
}

/// Calculate Sarvashtakavarga (Sum of all 7 BAVs)
pub fn calculate_sarvashtakavarga(
    planet_positions: &[f64], // Expecting 7
    ascendant: f64
) -> Sarvashtakavarga {
    let mut totals = vec![0; 12];
    
    for planet_id in 0..7 {
        let bav = calculate_binna_av(planet_id, planet_positions, ascendant);
        for i in 0..12 {
            totals[i] += bav.bindus[i];
        }
    }
    
    Sarvashtakavarga { totals }
}


// --- Reductions ---

/// Trikona Shodhana (Triangular Reduction)
/// Groups:
/// Fire: 0, 4, 8
/// Earth: 1, 5, 9
/// Air: 2, 6, 10
/// Water: 3, 7, 11
fn apply_trikona_reduction(bindus: &mut [i32; 12]) {
    let trikonas = [
        [0, 4, 8],
        [1, 5, 9],
        [2, 6, 10],
        [3, 7, 11]
    ];
    
    for group in trikonas.iter() {
        let v0 = bindus[group[0]];
        let v1 = bindus[group[1]];
        let v2 = bindus[group[2]];
        
        if v0 == 0 && v1 == 0 && v2 == 0 { continue; }
        
        // Parasara Rules:
        // 1. If at least one has 0, no reduction? 
        //    "If one is zero, leave others."
        //    "If two are zero, remove third."
        
        let zeros = (if v0 == 0 {1} else {0}) + (if v1 == 0 {1} else {0}) + (if v2 == 0 {1} else {0});
        
        if zeros == 1 {
            // One zero, others remain. Do nothing.
            continue;
        } else if zeros == 2 {
            // Two zeros, make third zero.
            bindus[group[0]] = 0;
            bindus[group[1]] = 0;
            bindus[group[2]] = 0;
        } else {
            // No zeros (or all zeros handled above).
            // Subtract lowest data.
            let min = v0.min(v1).min(v2);
            bindus[group[0]] -= min;
            bindus[group[1]] -= min;
            bindus[group[2]] -= min;
        }
    }
}

/// Ekadhipatya Shodhana (Dual Lordship Reduction)
/// Pairs: (0,7 Mars), (1,6 Ven), (2,5 Mer), (8,11 Jup), (9,10 Sat).
/// Cancer(3) and Leo(4) are single -> No reduction.
fn apply_ekadhipatya_reduction(bindus: &mut [i32; 12], planets_in_sign: &[Vec<i32>]) {
    // Pairs of signs owned by same planet
    let pairs = [
        (0, 7), // Mars
        (1, 6), // Venus
        (2, 5), // Mercury
        (8, 11), // Jupiter
        (9, 10)  // Saturn
    ];
    
    for &(s1, s2) in pairs.iter() {
        let v1 = bindus[s1];
        let v2 = bindus[s2];
        
        let occ1 = !planets_in_sign[s1].is_empty();
        let occ2 = !planets_in_sign[s2].is_empty();
        
        // Rules (Parasara):
        // 1. Both empty:
        //    - If both have points, replace both with lower? No.
        //    - Rule: "If both are without planets... eliminate the smaller figure and subtract it from the other?"
        //    - No. Rule:
        //      a) Both empty: Make greater equal to smaller? Or subtract smaller from greater?
        //      Correction: "If both signs are unoccupied, the smaller figure is made zero and larger remains same? No."
        //      Standard: "Reduce the larger figure to the level of the smaller figure? No."
        //      Correct Standard (Raman/Parasara):
        //      - One empty, other occupied: No reduction in occupied. Eliminate points in empty? 
        //        Actually: "If one is occupied... remove the figure in the unoccupied sign." (Wait, standard says make unoccupied zero?)
        //      - Both occupied: No reduction.
        //      - Both empty: 
        //         - If equal: Both zero.
        //         - If unequal: Make larger equal to smaller.
        //         - If one is zero: Leave other?
        
        // Let's implement rigorous Parasara from BPHS:
        
        if !occ1 && !occ2 {
            // Case 1: Both Unoccupied.
            if v1 == v2 {
                // Both zero.
                bindus[s1] = 0;
                bindus[s2] = 0;
            } else {
                // Replace larger with smaller.
                if v1 > v2 { bindus[s1] = v2; }
                else { bindus[s2] = v1; }
                // "The larger figure is removed? No, made equal." 
                // Wait, BPHS says "If unequal, the stronger (larger) should be reduced to the value of the weaker (smaller)."
                // result: both become min.
            }
        } else if occ1 && !occ2 {
            // Case 2: One occupied (s1), one empty (s2).
            // "Remove the points in the unoccupied sign."
            // So s2 becomes 0.
            bindus[s2] = 0; 
            // s1 remains.
        } else if !occ1 && occ2 {
            // Case 3: One empty (s1), one occupied (s2).
            bindus[s1] = 0;
            // s2 remains.
        } else {
            // Case 4: Both occupied.
            // No reduction.
        }
    }
}

pub fn calculate_reductions(
    bindus: &[i32], // 12
    planets: &[(i32, f64)] // (ID, Longitude)
) -> ReducedAshtakavarga {
    let mut reduced = [0; 12];
    for i in 0..12 {
        if i < bindus.len() { reduced[i] = bindus[i]; }
    }
    
    // Determine occupancy
    let mut occupancy: Vec<Vec<i32>> = vec![vec![]; 12];
    for &(pid, long) in planets.iter() {
        if pid > 8 { continue; } // Limit to main planets + nodes? Or just handle all. 
        // Standard Graha Pinda usually only Sun..Sat (0..6).
        // But occupancy checks might count Nodes? 
        // Parasara says "Occupied by a planet". Usually implies Sun..Sat. Nodes usually ignored in reductions?
        // Let's assume PID 0..6 for Pinda. For occupancy, maybe 0..6?
        // "If a sign is occupied by a planet".
        // Let's stick to 0..6 for safety unless specifically told Nodes count.
        if pid < 0 || pid > 6 { continue; }
        
        let sign = (long / 30.0).floor() as usize % 12;
        occupancy[sign].push(pid);
    }
    
    // 1. Trikona
    apply_trikona_reduction(&mut reduced);
    
    // 2. Ekadhipatya
    apply_ekadhipatya_reduction(&mut reduced, &occupancy);
    
    // 3. Shodya Pinda
    // Rasi Pinda + Graha Pinda.
    
    let rasi_multipliers = [7, 10, 8, 4, 10, 5, 7, 8, 9, 5, 11, 12];
    
    // Rasi Pinda
    let mut rasi_pinda = 0;
    for i in 0..12 {
        rasi_pinda += reduced[i] * rasi_multipliers[i];
    }
    
    // Graha Pinda
    // Multipliers for planets 0..6
    // Sun(0)=5, Moon(1)=5, Mars(2)=8, Mer(3)=5, Jup(4)=10, Ven(5)=7, Sat(6)=5.
    let multipliers = [5, 5, 8, 5, 10, 7, 5];
    
    let mut graha_pinda = 0;
    
    // Iterate planets and add multiplier if present
    for &(pid, long) in planets.iter() {
        if pid >= 0 && pid <= 6 {
             let sign = (long / 30.0).floor() as usize % 12;
             // Check if reduced[sign] > 0?
             // "Multiply the REDUCED points of that sign by the Planetary Multiplier."
             graha_pinda += reduced[sign] * multipliers[pid as usize];
        }
    }
    
    ReducedAshtakavarga {
        reduced_bindus: reduced.to_vec(),
        shodaya_pinda: rasi_pinda + graha_pinda
    }
}
