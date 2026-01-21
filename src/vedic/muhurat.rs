use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use alloc::string::{String, ToString};

/// Represents a specific time interval (Muhurat)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Muhurat {
    /// Name of the Muhurat (e.g., "Rahu Kalam")
    #[wasm_bindgen(getter_with_clone)]
    pub name: String,
    /// Start time in Unix milliseconds
    pub start: f64,
    /// End time in Unix milliseconds
    pub end: f64,
}

/// Collection of daily Muhurats
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct DayMuhurats {
    /// Period of Raahu (Inauspicious for starting new ventures)
    #[wasm_bindgen(getter_with_clone)]
    pub rahu_kalam: Muhurat,
    /// Period of Yama (Inauspicious)
    #[wasm_bindgen(getter_with_clone)]
    pub yamaganda: Muhurat,
    /// Period of Gulika (Neutral/Inauspicious)
    #[wasm_bindgen(getter_with_clone)]
    pub gulika: Muhurat,
    /// Brahma Muhurta (Pre-dawn)
    #[wasm_bindgen(getter_with_clone)]
    pub brahma_muhurta: Muhurat,
    /// Abhijit Muhurta (Mid-day victory period)
    #[wasm_bindgen(getter_with_clone)]
    pub abhijit_muhurta: Muhurat,
}

/// Calculate Raahu, Yamaganda, and Gulika for a given day
/// 
/// # Arguments
/// * `sunrise_ms` - Unix timestamp of sunrise in ms
/// * `sunset_ms` - Unix timestamp of sunset in ms
/// * `weekday` - 0=Sunday, 1=Monday, ..., 6=Saturday
#[wasm_bindgen]
pub fn calculate_muhurats(sunrise_ms: f64, sunset_ms: f64, weekday: u8) -> DayMuhurats {
    let duration = sunset_ms - sunrise_ms;
    let octad = duration / 8.0;

    // 1-based octad indices for start times
    // (Rules: 0=Sun, 1=Mon, 2=Tue, 3=Wed, 4=Thu, 5=Fri, 6=Sat)
    
    // Rahu Kalam (Inauspicious)
    // Sun:8, Mon:2, Tue:7, Wed:5, Thu:6, Fri:4, Sat:3
    let rahu_octad = match weekday {
        0 => 8, 1 => 2, 2 => 7, 3 => 5, 4 => 6, 5 => 4, 6 => 3,
        _ => 8, // Fallback
    };
    
    // Yamaganda (Inauspicious)
    // Sun:5, Mon:4, Tue:3, Wed:2, Thu:1, Fri:7, Sat:6
    let yama_octad = match weekday {
        0 => 5, 1 => 4, 2 => 3, 3 => 2, 4 => 1, 5 => 7, 6 => 6,
        _ => 5,
    };
    
    // Gulika Kalam (Neutral/Inauspicious)
    // Sun:7, Mon:6, Tue:5, Wed:4, Thu:3, Fri:2, Sat:1
    let gulika_octad = match weekday {
        0 => 7, 1 => 6, 2 => 5, 3 => 4, 4 => 3, 5 => 2, 6 => 1,
        _ => 7,
    };
    
    // Helper to create Muhurat
    let make_muhurat = |name: &str, start_octad: u8| -> Muhurat {
        let start_idx = (start_octad - 1) as f64;
        let start_time = sunrise_ms + (start_idx * octad);
        let end_time = start_time + octad;
        Muhurat {
            name: name.to_string(),
            start: start_time,
            end: end_time,
        }
    };
    
    // Brahma Muhurta: 96 minutes before Sunrise -> Sunrise
    // 96 minutes = 5760000 ms
    let brahma_start = sunrise_ms - 5_760_000.0;
    let brahma = Muhurat {
        name: "Brahma Muhurta".to_string(),
        start: brahma_start,
        end: sunrise_ms,
    };

    // Abhijit Muhurta: 8th Muhurta of 15 day divisions
    // Day duration = sunset - sunrise
    let day_len = sunset_ms - sunrise_ms;
    let muhurta_len = day_len / 15.0;
    let abhijit_start = sunrise_ms + (7.0 * muhurta_len); // Start of 8th
    let abhijit_end = abhijit_start + muhurta_len;
    
    let abhijit = Muhurat {
        name: "Abhijit Muhurta".to_string(),
        start: abhijit_start,
        end: abhijit_end,
    };

    DayMuhurats {
        rahu_kalam: make_muhurat("Rahu Kalam", rahu_octad),
        yamaganda: make_muhurat("Yamaganda", yama_octad),
        gulika: make_muhurat("Gulika Kalam", gulika_octad),
        brahma_muhurta: brahma,
        abhijit_muhurta: abhijit,
    }
}
