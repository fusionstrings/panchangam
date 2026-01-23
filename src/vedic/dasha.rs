//! Vimshottari Dasha calculations

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use alloc::string::{String, ToString};

/// Dasha period information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct DashaInfo {
    /// Current Mahadasha lord
    pub mahadasha: String,
    /// Current Antardasha lord
    pub antardasha: String,
    /// Current Pratyantardasha lord
    pub pratyantardasha: String,
    
    /// Date when the current Mahadasha ends (Unix ms)
    pub mahadasha_end_date: f64,
    /// Date when the current Antardasha ends (Unix ms)
    pub antardasha_end_date: f64,
    /// Date when the current Pratyantardasha ends (Unix ms)
    pub pratyantardasha_end_date: f64,
    
    /// Birth Nakshatra name
    pub nakshatra_name: String,
    /// Birth Nakshatra pada (1-4)
    pub nakshatra_pada: u8,
}

/// Yogini Dasha Information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct YoginiInfo {
    /// Current Mahadasha lord (Yogini)
    pub mahadasha: String,
    /// Current Antardasha lord (Yogini)
    pub antardasha: String,
    
    /// Date when the current Mahadasha ends (Unix ms)
    pub mahadasha_end_date: f64,
    /// Date when the current Antardasha ends (Unix ms)
    pub antardasha_end_date: f64,
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

/// Yogini Dasha Lords (Cycle 36 Years)
/// 1. Mangala (Moon) - 1 yr
/// 2. Pingala (Sun) - 2 yr
/// 3. Dhanya (Jupiter) - 3 yr
/// 4. Bhramari (Mars) - 4 yr
/// 5. Bhadrika (Mercury) - 5 yr
/// 6. Ulka (Saturn) - 6 yr
/// 7. Siddha (Venus) - 7 yr
/// 8. Sankata (Rahu) - 8 yr
const YOGINI_LORDS: [(&str, f64); 8] = [
    ("Mangala", 1.0),
    ("Pingala", 2.0),
    ("Dhanya", 3.0),
    ("Bhramari", 4.0),
    ("Bhadrika", 5.0),
    ("Ulka", 6.0),
    ("Siddha", 7.0),
    ("Sankata", 8.0),
];

// Total cycle = 120 years

/// Calculate Vimshottari Dasha details
/// 
/// # Arguments
/// * `moon_long` - Moon's sidereal longitude (degrees)
/// * `birth_time_ms` - Birth time (Unix ms)
/// * `current_time_ms` - Current time (Unix ms)
pub fn calculate_vimshottari(moon_long: f64, birth_time_ms: f64, current_time_ms: f64) -> DashaInfo {
    // Normalize moon_long to [0, 360)
    let mut normalized_moon = moon_long % 360.0;
    if normalized_moon < 0.0 {
        normalized_moon += 360.0;
    }

    // 1. Nakshatra calculation
    let nakshatra_val = normalized_moon / NAKSHATRA_SPAN;
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
    let mut time_in_dasha;
    
    if elapsed_years < balance_years {
        // Still in birth dasha
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
    let md_remaining = md_duration - time_in_dasha;
    let md_end_ms = current_time_ms + (md_remaining * ms_per_year);
    
    // 6. Antardasha (Sub-period)
    // Sub-periods are proportional: SubDuration = MainDuration * (SubLordDuration / 120)
    // Cycle starts from the Mahadasha lord itself
    let mut current_antardasha_idx = current_mahadasha_idx;
    let mut time_in_ad = time_in_dasha;
    let mut ad_duration;
    
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
    let ad_remaining = ad_duration - time_in_ad;
    let ad_end_ms = current_time_ms + (ad_remaining * ms_per_year);
    
    // 7. Pratyantardasha (Sub-sub-period)
    // PD = AD * (PD_Lord / 120)
    let mut current_pd_idx = current_antardasha_idx;
    let mut time_in_pd = time_in_ad;
    let mut pd_duration;
    
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
    let pd_remaining = pd_duration - time_in_pd;
    let pd_end_ms = current_time_ms + (pd_remaining * ms_per_year);
    
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
        mahadasha_end_date: md_end_ms,
        antardasha_end_date: ad_end_ms,
        pratyantardasha_end_date: pd_end_ms,
        nakshatra_name: nak_name,
        nakshatra_pada: pada,
    }
}

/// Calculate Yogini Dasha details
/// 
/// Cycle: 36 Years.
/// Start: (Nakshatra Index + 3) % 8.
/// Order: Mangala, Pingala, Dhanya, Bhramari, Bhadrika, Ulka, Siddha, Sankata.
pub fn calculate_yogini(moon_long: f64, birth_time_ms: f64, current_time_ms: f64) -> YoginiInfo {
    // Normalize moon_long
    let mut normalized_moon = moon_long % 360.0;
    if normalized_moon < 0.0 { normalized_moon += 360.0; }
    
    // Nakshatra Index (0-26)
    let nakshatra_val = normalized_moon / NAKSHATRA_SPAN;
    let nak_index = nakshatra_val.floor() as usize;
    let fraction = nakshatra_val - nakshatra_val.floor();
    
    // Starting Yogini
    let start_idx = (nak_index + 3) % 8;
    let (_start_lord, start_duration) = YOGINI_LORDS[start_idx];
    
    // Balance
    let balance_years = start_duration * (1.0 - fraction);
    
    let ms_per_year = 365.25 * 24.0 * 3600.0 * 1000.0;
    let elapsed_years = (current_time_ms - birth_time_ms) / ms_per_year;
    
    // Find Current MD
    let mut current_md_idx = start_idx;
    let mut time_in_dasha;
    
    if elapsed_years < balance_years {
        time_in_dasha = (start_duration - balance_years) + elapsed_years;
    } else {
        time_in_dasha = elapsed_years - balance_years;
        // Move to next dasha
        current_md_idx = (current_md_idx + 1) % 8;
        
        while time_in_dasha >= YOGINI_LORDS[current_md_idx].1 {
            time_in_dasha -= YOGINI_LORDS[current_md_idx].1;
            current_md_idx = (current_md_idx + 1) % 8;
        }
    }
    
    let (md_lord_name, md_duration) = YOGINI_LORDS[current_md_idx];
    let md_remaining = md_duration - time_in_dasha;
    let md_end_ms = current_time_ms + (md_remaining * ms_per_year);
    
    // Find Antardasha
    // AD Duration = MD_Duration * (AD_Lord_Duration / 36)
    // Order starts from MD Lord
    let mut current_ad_idx = current_md_idx;
    let mut time_in_ad = time_in_dasha;
    let mut ad_duration;
    
    loop {
        let (_, ad_lord_dur) = YOGINI_LORDS[current_ad_idx];
        ad_duration = md_duration * (ad_lord_dur / 36.0);
        
        if time_in_ad < ad_duration {
            break;
        }
        time_in_ad -= ad_duration;
        current_ad_idx = (current_ad_idx + 1) % 8;
    }
    
    let (ad_lord_name, _) = YOGINI_LORDS[current_ad_idx];
    let ad_remaining = ad_duration - time_in_ad;
    let ad_end_ms = current_time_ms + (ad_remaining * ms_per_year);
    
    YoginiInfo {
        mahadasha: md_lord_name.to_string(),
        antardasha: ad_lord_name.to_string(),
        mahadasha_end_date: md_end_ms,
        antardasha_end_date: ad_end_ms,
    }
}
