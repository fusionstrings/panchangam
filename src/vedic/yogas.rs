//! Yoga Detection Engine
//!
//! Identifies standard planetary combinations (Yogas).

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use alloc::vec::Vec;
use alloc::vec;
use alloc::string::String;


use crate::vedic::shadbala::PlanetInput;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct YogaResult {
    #[wasm_bindgen(skip)]
    pub name: String,
    #[wasm_bindgen(skip)]
    pub description: String,
}

#[wasm_bindgen]
impl YogaResult {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, description: String) -> Self {
        Self { name, description }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn description(&self) -> String {
        self.description.clone()
    }
}

// Internal Chart Representation for efficient checking


fn get_sign(long: f64) -> usize {
    (long / 30.0).floor() as usize % 12
}

fn get_house(asc_sign: usize, planet_sign: usize) -> usize {
    // 1-based house number relative to Ascendant sign
    // P = 0, Asc = 0 -> 1
    // P = 1, Asc = 0 -> 2
    let diff = (planet_sign as i32 - asc_sign as i32).rem_euclid(12);
    (diff + 1) as usize
}

fn get_lord(sign: usize) -> i32 {
    match sign {
        0 | 7 => 2, // Aries, Scorpio -> Mars
        1 | 6 => 5, // Taurus, Libra -> Venus
        2 | 5 => 3, // Gemini, Virgo -> Mercury
        3 => 1,     // Cancer -> Moon
        4 => 0,     // Leo -> Sun
        8 | 11 => 4,// Sagittarius, Pisces -> Jupiter
        9 | 10 => 6,// Capricorn, Aquarius -> Saturn
        _ => -1,
    }
}

// Exaltation helper (simplified)
fn is_exalted(planet_id: i32, sign: usize) -> bool {
    match planet_id {
        0 => sign == 0, // Sun in Aries
        1 => sign == 1, // Moon in Taurus
        2 => sign == 9, // Mars in Capricorn
        3 => sign == 5, // Mercury in Virgo
        4 => sign == 3, // Jupiter in Cancer
        5 => sign == 11, // Venus in Pisces
        6 => sign == 6, // Saturn in Libra
        _ => false,
    }
}

fn is_own_sign(planet_id: i32, sign: usize) -> bool {
    get_lord(sign) == planet_id
}

pub fn check_yogas(
    planets: &[PlanetInput], // data
    ascendant: f64
) -> Vec<YogaResult> {
    let mut yogas = Vec::new();
    
    // Build context
    let mut p_longs = [0.0; 7];
    let mut p_signs = [0; 7];
    let mut p_houses = [0; 7];
    
    // Map existing inputs
    for p in planets {
        if p.id >= 0 && p.id <= 6 {
            p_longs[p.id as usize] = p.longitude;
        }
    }
    
    let asc_sign = get_sign(ascendant);
    
    for i in 0..7 {
        p_signs[i] = get_sign(p_longs[i]);
        p_houses[i] = get_house(asc_sign, p_signs[i]);
    }
    
    // ---------------------------------------------------------
    // 1. Pancha Mahapurusha Yogas (Mars, Mer, Jup, Ven, Sat)
    // Rule: Planet in Own/Exalt sign AND in Kendra (1, 4, 7, 10) from Ascendant.
    // ---------------------------------------------------------
    
    // Ruchaka (Mars)
    if (is_exalted(2, p_signs[2]) || is_own_sign(2, p_signs[2])) && [1, 4, 7, 10].contains(&p_houses[2]) {
        yogas.push(YogaResult::new("Ruchaka Yoga".into(), "Mars in Own/Exalted sign in Kendra.".into()));
    }
    
    // Bhadra (Mercury)
    if (is_exalted(3, p_signs[3]) || is_own_sign(3, p_signs[3])) && [1, 4, 7, 10].contains(&p_houses[3]) {
         yogas.push(YogaResult::new("Bhadra Yoga".into(), "Mercury in Own/Exalted sign in Kendra.".into()));
    }
    
    // Hamsa (Jupiter)
    if (is_exalted(4, p_signs[4]) || is_own_sign(4, p_signs[4])) && [1, 4, 7, 10].contains(&p_houses[4]) {
         yogas.push(YogaResult::new("Hamsa Yoga".into(), "Jupiter in Own/Exalted sign in Kendra.".into()));
    }
    
    // Malavya (Venus)
    if (is_exalted(5, p_signs[5]) || is_own_sign(5, p_signs[5])) && [1, 4, 7, 10].contains(&p_houses[5]) {
         yogas.push(YogaResult::new("Malavya Yoga".into(), "Venus in Own/Exalted sign in Kendra.".into()));
    }
    
    // Sasa (Saturn)
    if (is_exalted(6, p_signs[6]) || is_own_sign(6, p_signs[6])) && [1, 4, 7, 10].contains(&p_houses[6]) {
         yogas.push(YogaResult::new("Sasa Yoga".into(), "Saturn in Own/Exalted sign in Kendra.".into()));
    }
    
    // ---------------------------------------------------------
    // 2. Gaja Kesari Yoga (Jupiter and Moon)
    // Rule: Jupiter in Kendra from Moon.
    // ---------------------------------------------------------
    let jup_from_moon = get_house(p_signs[1], p_signs[4]); // Moon sign as Asc
    if [1, 4, 7, 10].contains(&jup_from_moon) {
        yogas.push(YogaResult::new("Gaja Kesari Yoga".into(), "Jupiter in Kendra from Moon.".into()));
    }
    
    // ---------------------------------------------------------
    // 3. Budhaditya Yoga (Sun + Mercury)
    // Rule: Sun and Mercury in same house.
    // ---------------------------------------------------------
    if p_signs[0] == p_signs[3] {
        yogas.push(YogaResult::new("Budhaditya Yoga".into(), "Sun and Mercury conjunction.".into()));
    }
    
    // ---------------------------------------------------------
    // 4. Chandra Mangala Yoga (Moon + Mars)
    // Rule: Moon and Mars conjunct or opposite? Standard is conjunct.
    // ---------------------------------------------------------
    if p_signs[1] == p_signs[2] {
         yogas.push(YogaResult::new("Chandra Mangala Yoga".into(), "Moon and Mars conjunction.".into()));
    }
    
    // ---------------------------------------------------------
    // 5. Amala Yoga
    // Rule: Benefic (Jup, Ven, Mer) in 10th from Asc or Moon.
    // ---------------------------------------------------------
    let benefics = [3, 4, 5];
    for &b in &benefics {
        if p_houses[b] == 10 {
            yogas.push(YogaResult::new("Amala Yoga".into(), "Benefic in 10th house from Ascendant.".into()));
            break; // Defined once
        }
        let b_from_moon = get_house(p_signs[1], p_signs[b]);
        if b_from_moon == 10 {
             yogas.push(YogaResult::new("Amala Yoga".into(), "Benefic in 10th house from Moon.".into()));
             break;
        }
    }
    
    // ---------------------------------------------------------
    // 6. Vipareeta Raja Yoga (Simplified)
    // Rule: Lords of Trik (6, 8, 12) in Trik.
    // ---------------------------------------------------------
    let trik_houses = [6, 8, 12];
    
    // Find lords of 6, 8, 12 relative to Asc
    let sign_6 = (asc_sign + 5) % 12;
    let sign_8 = (asc_sign + 7) % 12;
    let sign_12 = (asc_sign + 11) % 12;
    
    let lord_6 = get_lord(sign_6);
    let lord_8 = get_lord(sign_8);
    let lord_12 = get_lord(sign_12);
    
    let check_placement = |planet_id: i32| -> bool {
        if planet_id < 0 { return false; }
        let h = p_houses[planet_id as usize];
        trik_houses.contains(&(h as i32))
    };
    
    if check_placement(lord_6) && check_placement(lord_8) && check_placement(lord_12) {
         // This is a check if ALL lords are in trik? No, usually if ANY lord is in Trik it's partial Vipareeta.
         // "If the lords of 6, 8, 12 houses occupy self or other trika houses."
         // If Lord 6 is in 6, 8, or 12.
         
         let l6_ok = check_placement(lord_6);
         let l8_ok = check_placement(lord_8);
         let l12_ok = check_placement(lord_12);
         
         if l6_ok || l8_ok || l12_ok {
              yogas.push(YogaResult::new("Vipareeta Raja Yoga".into(), "Lord of 6/8/12 in 6/8/12.".into()));
         }
    }
    
    yogas
}
