//! Vimshottari Dasha calculations

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Dasha period information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct DashaInfo {
    /// Current Mahadasha lord (e.g., "Ketu")
    pub mahadasha: String,
    /// Current Antardasha lord (e.g., "Venus")
    pub antardasha: String,
    /// Current Pratyantardasha lord (e.g., "Sun")
    pub pratyantardasha: String,
    /// Completion date of the current sub-period (Unix ms)
    pub end_date: f64,
    /// Time remaining in current sub-period (years)
    pub years_remaining: f64,
    /// Birth Nakshatra name
    pub nakshatra_name: String,
    /// Birth Nakshatra pada (1-4)
    pub nakshatra_pada: u8,
}

const NAKSHATRA_SPAN: f64 = 360.0 / 27.0; // 13.3333...

/// Dasha lords and durations
const DASHA_LORDS: [(&str, f64); 9] = [
    ("Ketu", 7.0),
    ("Venus", 20.0),
    ("Sun", 6.0),
    ("Moon", 10.0),
    ("Mars", 7.0),
    ("Rahu", 18.0),
    ("Jupiter", 16.0),
    ("Saturn", 19.0),
    ("Mercury", 17.0),
];

// Total cycle = 120 years

/// Calculate Vimshottari Dasha for a given birth and current date.
/// 
/// # Arguments
/// * `moon_long` - Moon's sidereal longitude at birth (degrees)
/// * `birth_time_ms` - Birth time in Unix milliseconds
/// * `current_time_ms` - Current time in Unix milliseconds
pub fn calculate_vimshottari(moon_long: f64, birth_time_ms: f64, current_time_ms: f64) -> DashaInfo {
    // 1. Nakshatra calculation
    let nakshatra_val = moon_long / NAKSHATRA_SPAN;
    let nak_index = nakshatra_val.floor() as usize; // 0-26
    let fraction = nakshatra_val - nakshatra_val.floor();
    
    // Pada
    let pada = (fraction * 4.0).floor() as u8 + 1;
    
    // 2. Starting Dasha
    // Cycle determines starting lord. nak_index % 9 maps to DASHA_LORDS
    let start_dasha_idx = nak_index % 9;
    let (_start_lord, start_duration) = DASHA_LORDS[start_dasha_idx];
    
    // 3. Balance at birth
    let balance_years = start_duration * (1.0 - fraction);
    
    // 4. Elapsed time in years
    // 365.25 days per year average for dasha calculations usually
    let ms_per_year = 365.25 * 24.0 * 3600.0 * 1000.0;
    let elapsed_years = (current_time_ms - birth_time_ms) / ms_per_year;
    
    // 5. Find current Mahadasha
    let mut current_mahadasha_idx = start_dasha_idx;
    let mut time_in_dasha = elapsed_years;
    
    // Adjust for initial balance
    // If elapsed time < balance, we are still in first dasha
    let mut mahadasha_start_offset = 0.0;
    
    if elapsed_years < balance_years {
        // Still in birth dasha
        // Effective time passed within this dasha is (duration - balance) + elapsed
        // But simpler: calculate sub-periods based on remaining balance?
        // Let's standardise: treat birth moment as (duration - balance) into the dasha.
        time_in_dasha = (DASHA_LORDS[start_dasha_idx].1 - balance_years) + elapsed_years;
    } else {
        // Passed the first dasha balance
        time_in_dasha = elapsed_years - balance_years;
        // Move to next dasha
        current_mahadasha_idx = (current_mahadasha_idx + 1) % 9;
        
        while time_in_dasha >= DASHA_LORDS[current_mahadasha_idx].1 {
            time_in_dasha -= DASHA_LORDS[current_mahadasha_idx].1;
            current_mahadasha_idx = (current_mahadasha_idx + 1) % 9;
        }
    }
    
    let (md_lord, md_duration) = DASHA_LORDS[current_mahadasha_idx];
    
    // 6. Antardasha (Sub-period)
    // Sub-periods are proportional: SubDuration = MainDuration * (SubLordDuration / 120)
    // Cycle starts from the Mahadasha lord itself
    let mut current_antardasha_idx = current_mahadasha_idx;
    let mut time_in_ad = time_in_dasha;
    let mut ad_duration = 0.0;
    
    loop {
        let (_ad_lord_name, ad_lord_dur) = DASHA_LORDS[current_antardasha_idx];
        ad_duration = md_duration * (ad_lord_dur / 120.0);
        
        if time_in_ad < ad_duration {
            break;
        }
        time_in_ad -= ad_duration;
        current_antardasha_idx = (current_antardasha_idx + 1) % 9;
    }
    
    let (ad_lord, _) = DASHA_LORDS[current_antardasha_idx];
    
    // 7. Pratyantardasha (Sub-sub-period)
    // PD = AD * (PD_Lord / 120)
    let mut current_pd_idx = current_antardasha_idx;
    let mut time_in_pd = time_in_ad;
    let mut pd_duration = 0.0;
    
    loop {
        let (_pd_lord, pd_lord_dur) = DASHA_LORDS[current_pd_idx];
        pd_duration = ad_duration * (pd_lord_dur / 120.0);
        
        if time_in_pd < pd_duration {
            break;
        }
        time_in_pd -= pd_duration;
        current_pd_idx = (current_pd_idx + 1) % 9;
    }
    
    let (pd_lord, _) = DASHA_LORDS[current_pd_idx];
    
    // Calculate end date of current PD
    let remaining_years_in_pd = pd_duration - time_in_pd;
    let end_ms = current_time_ms + (remaining_years_in_pd * ms_per_year);
    
    use crate::vedic::nakshatra::NAKSHATRA_NAMES;
    let nak_name = if nak_index < 27 { 
        NAKSHATRA_NAMES[nak_index].to_string() 
    } else { 
        "Unknown".to_string() 
    };

    DashaInfo {
        mahadasha: md_lord.to_string(),
        antardasha: ad_lord.to_string(),
        pratyantardasha: pd_lord.to_string(),
        end_date: end_ms,
        years_remaining: remaining_years_in_pd,
        nakshatra_name: nak_name,
        nakshatra_pada: pada,
    }
}
