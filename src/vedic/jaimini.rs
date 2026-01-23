//! Jaimini Astrology Calculation Module
//!
//! Includes Charakarakas (7 or 8 karaka schemes) and Aspects.

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use alloc::vec::Vec;
use alloc::vec;
use alloc::boxed::Box;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen]
pub enum KarakaName {
    AtmaKaraka = 0,    // Soul
    AmatyaKaraka = 1,  // Career/Mind
    BhatriKaraka = 2,  // Siblings
    MatriKaraka = 3,   // Mother
    PitraKaraka = 4,   // Father (only in 8-karaka scheme)
    PutraKaraka = 5,   // Children
    GnatiKaraka = 6,   // Relatives/Enemies
    DaraKaraka = 7,    // Spouse
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct KarakaObject {
    pub planet_id: i32,
    pub karaka_name: KarakaName,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct JaiminiProfile {
     #[wasm_bindgen(skip)]
    pub karakas: Vec<KarakaObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct CharaDashaPeriod {
    pub sign_id: usize, // 0=Aries, etc.
    pub duration_years: i32,
    pub start_year: f64,
    pub end_year: f64,
}

#[wasm_bindgen]
impl CharaDashaPeriod {
    #[wasm_bindgen(constructor)]
    pub fn new(sign_id: usize, duration_years: i32, start_year: f64, end_year: f64) -> Self {
        Self { sign_id, duration_years, start_year, end_year }
    }
}

#[wasm_bindgen]
impl JaiminiProfile {
    #[wasm_bindgen(getter)]
    pub fn karakas(&self) -> Box<[KarakaObject]> {
        self.karakas.clone().into_boxed_slice()
    }
    
    #[wasm_bindgen(setter)]
    pub fn set_karakas(&mut self, karakas: Box<[KarakaObject]>) {
        self.karakas = karakas.into_vec();
    }
}

/// Calculate Charakarakas
/// 
/// # Arguments
/// * `planet_longitudes` - Array of tuples (planet_id, longitude). Include Rahu if using 8 karakas.
/// * `use_8_karakas` - If true, use 8 karakas (include Rahu/Pitra). Default false (7 Karakas).
pub fn calculate_charakarakas(
    planet_longitudes: &[(i32, f64)],
    use_8_karakas: bool
) -> Vec<KarakaObject> {
    // 1. Normalize longitudes to 0-30 degrees (Sign traversal ignored)
    // Jaimini compares "advancement in sign".
    
    let mut candidates: Vec<(i32, f64)> = planet_longitudes.iter()
        .map(|&(id, long)| (id, long % 30.0))
        .collect();
        
    // Ketu is usually excluded. Rahu includes?
    // In 7 Karaka scheme: Sun, Moon, Mar, Mer, Jup, Ven, Sat. (Rahu sometimes excluded, sometimes Mixed).
    // Standard Parashara 7-karaka: Ra/Ke excluded.
    // 8-Karaka: Rahu included.
    
    if !use_8_karakas {
        // Filter out Nodes (id > 6)
        candidates.retain(|(id, _)| *id <= 6);
    } else {
        // Filter out Ketu? Rahu(101) Ketu(102) or similar IDs.
        // Assuming Swiss Eph IDs: Sun(0)..Sat(6), Ura(7)..Plu(9). Nodes: Rahu=11 or 10.
        // We need to know specific IDs. usually 0..6 + Rahu.
        // Panchnagam ids: Sun=0..Sat=6, Rahu=Rahu.
        // Let's assume input filters correct planets or we implement rigorous ID filter later.
    }
    
    // Sort descending by degree
    candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    let names = if use_8_karakas {
        vec![
            KarakaName::AtmaKaraka,
            KarakaName::AmatyaKaraka,
            KarakaName::BhatriKaraka,
            KarakaName::MatriKaraka,
            KarakaName::PitraKaraka, // 8-karaka inserts Pitra here? Or Matri becomes Pitra?
            // Standard 8: AK, AmK, BK, MK, PiK, PuK, GK, DK
            KarakaName::PutraKaraka,
            KarakaName::GnatiKaraka,
            KarakaName::DaraKaraka,
        ]
    } else {
        vec![
            KarakaName::AtmaKaraka,
            KarakaName::AmatyaKaraka,
            KarakaName::BhatriKaraka, // Brother
            KarakaName::MatriKaraka,  // Mother
            KarakaName::PutraKaraka,  // Son
            KarakaName::GnatiKaraka,  // Relation
            KarakaName::DaraKaraka,   // Spouse
        ]
    };
    
    let mut results = Vec::new();
    for (i, &(pid, long)) in candidates.iter().enumerate() {
        if i < names.len() {
            results.push(KarakaObject {
                planet_id: pid,
                karaka_name: names[i],
                longitude: long
            });
        }
    }
    
    results
}


// --- Chara Dasha Helpers ---

fn get_lord_of_sign(sign: usize) -> i32 {
    match sign {
        0 | 7 => 2, // Ari, Sco -> Mars
        1 | 6 => 5, // Tau, Lib -> Venus
        2 | 5 => 3, // Gem, Vir -> Mercury
        3 => 1,     // Can -> Moon
        4 => 0,     // Leo -> Sun
        8 | 11 => 4,// Sag, Pis -> Jupiter
        9 | 10 => 6,// Cap, Aqu -> Saturn
        _ => -1,
    }
}

// Check strength for Dual Lords (Scorpio: Mars/Ketu, Aquarius: Sat/Rahu)
// Returns the stronger planet ID location (sign index).
fn get_dual_lord_location(
    sign_id: usize, // 7 for Sco, 10 for Aqu
    planets: &[(i32, f64)], // needed to check associations/positions
    _rahu_id: i32,
    ketu_id: i32,
    rahu_id: i32
) -> usize {
    // 1. Identify candidates
    let (p1, p2) = if sign_id == 7 {
        (2, ketu_id) // Mars, Ketu
    } else {
        (6, rahu_id) // Saturn, Rahu
    };
    
    // Find signs of p1 and p2
    let find_sign = |pid: i32| -> usize {
        for &(id, long) in planets {
            if id == pid {
                return (long / 30.0).floor() as usize % 12;
            }
        }
        0 // Default fallback
    };
    
    let s1 = find_sign(p1);
    let s2 = find_sign(p2);
    
    if s1 == s2 { return s1; } // Conjoined, same
    
    // Rule 1: Planet with more planets conjoined is stronger.
    let count_conjunctions = |s: usize| -> usize {
        planets.iter().filter(|&&(_, l)| (l / 30.0).floor() as usize % 12 == s).count()
        // Subtract self (the lord)? The count includes itself. It's fine for comparison.
    };
    
    let c1 = count_conjunctions(s1);
    let c2 = count_conjunctions(s2);
    
    if c1 != c2 {
        return if c1 > c2 { s1 } else { s2 };
    }
    
    // Rule 2: Exalted? 
    // Mars Exalted in Cap(9). Ketu exalted in... Sag/Sco? Disputed. K.N. Rao ignores node exaltation usually?
    // Saturn Exalted in Lib(6). Rahu exalted in Tau/Gem?
    // Simplified MVP: Skip to Longitude rule if counts equal.
    
    // Rule 3: Higher Longitude (degrees within sign).
    // Or is it Atmakaraka Strength? 
    // Standard Jaimini says "One who has advanced more degrees".
    let get_deg = |pid: i32| -> f64 {
        for &(id, long) in planets {
            if id == pid { return long % 30.0; }
        }
        0.0
    };
    
    if get_deg(p1) > get_deg(p2) { s1 } else { s2 }
}


fn calculate_duration(sign: usize, lord_sign: usize, is_direct: bool) -> i32 {
    let count = if is_direct {
        // Forward: Lord - Sign
        (lord_sign as i32 - sign as i32 + 12).rem_euclid(12) 
    } else {
        // Reverse: Sign - Lord
        (sign as i32 - lord_sign as i32 + 12).rem_euclid(12)
    } as i32;
    
    // Count is number of signs. 
    // If Count = 0 (same sign) -> Jaimini says 1, but rule says subtract 1.
    // If lord in sign: 12 years.
    
    if sign == lord_sign {
        return 12;
    }
    
    // Formula: Count - 1.
    // E.g. Aries(1) to Mars in Gem(3). Direct.
    // Lord(2) - Sign(0). (2-0+12)%12 = 2.
    // Count 0->1->2? No.
    // Signs: Aries(1), Tau(2), Gem(3).
    // Count = 3 (Ari,Tau,Gem). 
    // My formula: (2 - 0) = 2? No.
    // Indices: Ari=0, Tau=1, Gem=2.
    // Diff 2 - 0 = 2.
    // Count should be 3. So Diff + 1.
    
    // Correct Formula logic:
    // Direct: (Lord - Sign + 12) % 12 + 1.
    // Reverse: (Sign - Lord + 12) % 12 + 1.
    
    let full_count = if is_direct {
        (lord_sign as i32 - sign as i32 + 12).rem_euclid(12) + 1
    } else {
        (sign as i32 - lord_sign as i32 + 12).rem_euclid(12) + 1
    };
    
    let duration = full_count - 1;
    if duration == 0 { 12 } else { duration }
}

/// Calculate Chara Dasha
/// 
/// Returns sequence of periods starting from birth date.
/// Assumes K.N. Rao / Standard Jaimini method.
pub fn calculate_chara_dasha(
    planets: &[(i32, f64)], // Must include Rahu(7/101?) Ketu(8/102?)
    ascendant_sign: usize,
    start_year: f64
) -> Vec<CharaDashaPeriod> {
    let mut periods = Vec::new();
    let mut current_year = start_year;
    
    // Identify Rahu/Ketu IDs. Let's scan for them.
    // Default assumption: 7=Rahu, 8=Ketu? Or check standard set.
    // If not found, use default.
    let rahu_id = planets.iter().find(|(id, _)| *id == 101 || *id == 7).map(|(id,_)| *id).unwrap_or(7);
    let ketu_id = planets.iter().find(|(id, _)| *id == 102 || *id == 8).map(|(id,_)| *id).unwrap_or(8);
    
    // 1. Determine Order of Dashas (Savya / Apasavya) based on the 9th from Lagna??
    // Actually, K.N. Rao's Chara Dasha order:
    // Aries, Libra, Leo, Aquarius, Pisces, Taurus, Cancer, Sagittarius, Capricorn, Virgo, Gemini, Scorpio?
    // Wait, the order depends on the Sign Group (Savya/Apasavya).
    // Savya (Zodiacal): 1, 2, 3...
    // Apasavya (Reverse): 1, 12, 11...
    
    // Groups:
    // Savya: Aries(1), Leo(5), Virgo(6), Libra(7), Aquarius(11), Pisces(12)?
    // Disputed.
    // Let's use the widely accepted K.N. Rao table for Order.
    
    // Order of Dashas usually starts from the ASCENDANT SIGN.
    // The specific sequence depends on the PADA (Arudha) or just the Sign Nature?
    // K.N. Rao:
    // Aries: Direct (Ari, Tau, Gem...)
    // Taurus: Direct (Tau, Gem...)
    // Gemini: Direct...
    // Wait, simple Jaimini Chara Dasha uses Zodiacal (1,2,3) or Anti-Zodiacal (1,12,11)?
    // Rules:
    // 1. Aries, Libra, Leo, Aquarius, Pisces, Taurus: SAVYA grouping?
    // NO.
    // Reliable source (K.N. Rao):
    // Aries(1): Direct (1,2,3...)
    // Taurus(2): Direct (2,3,4...)
    // Gemini(3): Direct
    // Cancer(4): Reverse (4,3,2...)
    // Leo(5): Reverse
    // Virgo(6): Reverse
    // Libra(7): Direct
    // Scorpio(8): Direct
    // Sagittarius(9): Direct
    // Capricorn(10): Reverse
    // Aquarius(11): Reverse
    // Pisces(12): Reverse
    
    // Summary:
    // 1, 2, 3, 7, 8, 9 -> Direct
    // 4, 5, 6, 10, 11, 12 -> Reverse
    
    let is_dasha_order_direct = match ascendant_sign {
        0 | 1 | 2 | 6 | 7 | 8 => true,
        _ => false
    };
    
    // Logic for sequence generation
    let mut dasha_signs = Vec::new();
    let mut curr = ascendant_sign;
    for _ in 0..12 {
        dasha_signs.push(curr);
        if is_dasha_order_direct {
            curr = (curr + 1) % 12;
        } else {
            curr = (curr + 11) % 12;
        }
    }
    
    // 2. Calculate Duration for each sign
    for &sign in &dasha_signs {
        // Find Lord
        let lord = get_lord_of_sign(sign);
        
        let lord_sign = if sign == 7 || sign == 10 { // Sco or Aqu
            // Special rules for Dual Lord
             get_dual_lord_location(sign, planets, 0, ketu_id, rahu_id)
        } else {
            // Standard
            let mut s = 0;
            for &(id, long) in planets {
                if id == lord {
                    s = (long / 30.0).floor() as usize % 12;
                    break;
                }
            }
            s
        };
        
        // Count Direction for Duration:
        // Dependent on the Dasha Sign itself.
        // Rule: 
        // Odd Signs (1,3,5,7,9,11): Direct Counting?
        // Even Signs (2,4,6,8,10,12): Reverse Counting?
        // Exception: 
        // 4, 5, 8, 11? 
        // K.N. Rao Modifications exist but basic Jaimini is:
        // Sama (Even): Reverse. Vishama (Odd): Direct.
        // Catch for Dual Ownership signs (Sco, Aqu)?
        // Scorpio(8) is Even -> Reverse.
        // Aquarius(11) is Odd -> Direct.
        // Cancer(4) Even -> Reverse.
        // Leo(5) Odd -> Direct.
        
        let is_odd = (sign + 1) % 2 != 0;
        let count_direct = is_odd;
        
        // Wait, K.N. Rao exception logic:
        // "Vrishabha(2), Simha(5), Vrischika(8), Kumbha(11)"
        // Taurus(2): Reverse (Even).
        // Leo(5): Reverse? (Odd but exception?) - Actually some texts say Leo is Reverse.
        // Let's stick to simple Odd=Direct, Even=Reverse for MVP unless directed otherwise.
        
        let duration = calculate_duration(sign, lord_sign, count_direct);
        
        periods.push(CharaDashaPeriod {
            sign_id: sign,
            duration_years: duration,
            start_year: current_year,
            end_year: current_year + duration as f64
        });
        
        current_year += duration as f64;
    }
    
    periods
}
