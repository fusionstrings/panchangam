//! Swiss Ephemeris Constants
//!
//! Re-exports common constants for planetary calculations.
//! Eliminates need for clients to hardcode these values.
//!
//! Usage in TypeScript:
//! ```typescript
//! import { Constants } from "@fusionstrings/panchangam";
//! const sunId = Constants.SE_SUN;
//! ```

use wasm_bindgen::prelude::*;

/// Container for Swiss Ephemeris constants.
/// Access via static getters, e.g., `Constants.SE_SUN`
#[wasm_bindgen]
pub struct Constants;

#[wasm_bindgen]
impl Constants {
    // ========================================================================
    // Planet IDs (from Swiss Ephemeris swephexp.h)
    // ========================================================================
    
    /// Sun (0)
    #[wasm_bindgen(getter)]
    pub fn SE_SUN() -> i32 { 0 }
    
    /// Moon (1)
    #[wasm_bindgen(getter)]
    pub fn SE_MOON() -> i32 { 1 }
    
    /// Mercury (2)
    #[wasm_bindgen(getter)]
    pub fn SE_MERCURY() -> i32 { 2 }
    
    /// Venus (3)
    #[wasm_bindgen(getter)]
    pub fn SE_VENUS() -> i32 { 3 }
    
    /// Mars (4)
    #[wasm_bindgen(getter)]
    pub fn SE_MARS() -> i32 { 4 }
    
    /// Jupiter (5)
    #[wasm_bindgen(getter)]
    pub fn SE_JUPITER() -> i32 { 5 }
    
    /// Saturn (6)
    #[wasm_bindgen(getter)]
    pub fn SE_SATURN() -> i32 { 6 }
    
    /// Uranus (7)
    #[wasm_bindgen(getter)]
    pub fn SE_URANUS() -> i32 { 7 }
    
    /// Neptune (8)
    #[wasm_bindgen(getter)]
    pub fn SE_NEPTUNE() -> i32 { 8 }
    
    /// Pluto (9)
    #[wasm_bindgen(getter)]
    pub fn SE_PLUTO() -> i32 { 9 }
    
    /// Mean Lunar Node / Rahu (10)
    #[wasm_bindgen(getter)]
    pub fn SE_MEAN_NODE() -> i32 { 10 }
    
    /// True Lunar Node / Rahu (11)
    #[wasm_bindgen(getter)]
    pub fn SE_TRUE_NODE() -> i32 { 11 }

    // ========================================================================
    // Calculation Flags (from Swiss Ephemeris swephexp.h)
    // ========================================================================
    
    /// Use Swiss Ephemeris (2)
    #[wasm_bindgen(getter)]
    pub fn SEFLG_SWIEPH() -> i32 { 2 }
    
    /// Use Moshier Ephemeris (4)
    #[wasm_bindgen(getter)]
    pub fn SEFLG_MOSEPH() -> i32 { 4 }
    
    /// Include speed in output (256)
    #[wasm_bindgen(getter)]
    pub fn SEFLG_SPEED() -> i32 { 256 }
    
    /// Sidereal positions (65536)
    #[wasm_bindgen(getter)]
    pub fn SEFLG_SIDEREAL() -> i32 { 65536 }
    
    /// Heliocentric positions (8)
    #[wasm_bindgen(getter)]
    pub fn SEFLG_HELCTR() -> i32 { 8 }
    
    /// True positions - no aberration (16)
    #[wasm_bindgen(getter)]
    pub fn SEFLG_TRUEPOS() -> i32 { 16 }
    
    /// Equatorial positions (2048)
    #[wasm_bindgen(getter)]
    pub fn SEFLG_EQUATORIAL() -> i32 { 2048 }
    
    /// Topocentric positions (32768)
    #[wasm_bindgen(getter)]
    pub fn SEFLG_TOPOCTR() -> i32 { 32768 }

    // ========================================================================
    // Calendar Types
    // ========================================================================
    
    /// Gregorian calendar (1)
    #[wasm_bindgen(getter)]
    pub fn SE_GREG_CAL() -> i32 { 1 }
    
    /// Julian calendar (0)
    #[wasm_bindgen(getter)]
    pub fn SE_JUL_CAL() -> i32 { 0 }
}
