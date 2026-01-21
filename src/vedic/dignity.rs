//! Planetary Dignity calculations
//! Based on Vedic Astrology rules (Exalted, Moolatrikona, Own Sign, Debilitated, etc.)

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use alloc::string::{String, ToString};

/// Planetary Dignity status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[wasm_bindgen]
pub enum Dignity {
    Exalted = 0,
    Moolatrikona = 1,
    OwnSign = 2,
    GreatFriend = 3,
    Friend = 4,
    Neutral = 5,
    Enemy = 6,
    GreatEnemy = 7,
    Debilitated = 8,
}

impl Dignity {
    pub fn to_string(&self) -> String {
        match self {
            Dignity::Exalted => "Exalted",
            Dignity::Moolatrikona => "Moolatrikona",
            Dignity::OwnSign => "Own Sign",
            Dignity::GreatFriend => "Great Friend",
            Dignity::Friend => "Friend",
            Dignity::Neutral => "Neutral",
            Dignity::Enemy => "Enemy",
            Dignity::GreatEnemy => "Great Enemy",
            Dignity::Debilitated => "Debilitated",
        }.to_string()
    }
}

/// Calculate dignity for a planet in a specific zodiac sign (1-12)
/// 
/// # Arguments
/// * `planet_name` - Name of the planet (e.g., "Sun", "Moon")
/// * `sign_index` - Zodiac sign index (1 = Aries, 12 = Pisces)
pub fn calculate_dignity(planet_name: &str, sign_index: u8) -> Dignity {
    // Basic Vedic table (Parashara)
    // 1-Aries, 2-Taurus, 3-Gemini, 4-Cancer, 5-Leo, 6-Virgo,
    // 7-Libra, 8-Scorpio, 9-Sagittarius, 10-Capricorn, 11-Aquarius, 12-Pisces
    
    match planet_name {
        "Sun" => match sign_index {
            1 => Dignity::Exalted,      // Aries
            7 => Dignity::Debilitated,  // Libra
            5 => Dignity::OwnSign,      // Leo
            // Friend: Moon (4), Mars (1,8), Jupiter (9,12) -> 4, 8, 9, 12 (1 is Exalted)
            4 | 8 | 9 | 12 => Dignity::Friend,
            // Enemy: Venus (2,7), Saturn (10,11) -> 2, 10, 11 (7 is Debilitated)
            2 | 10 | 11 => Dignity::Enemy,
            // Neutral: Mercury (3,6)
            3 | 6 => Dignity::Neutral,
            _ => Dignity::Neutral,
        },
        "Moon" => match sign_index {
            2 => Dignity::Exalted,      // Taurus (technically first 3 deg, rest Moolatrikona, here simplified)
            8 => Dignity::Debilitated,  // Scorpio
            4 => Dignity::OwnSign,      // Cancer
            // Friend: Sun (5), Mercury (3,6) -> 5, 3, 6 (2 is Exalted)
            5 | 3 | 6 => Dignity::Friend,
            // No Neutrals/Enemies usually defined strictly for Moon like others? 
            // Standard: Friends: Sun, Mercury. Neutrals: Mars, Jup, Ven, Sat. No Enemies.
            // Wait, standard dict:
            // Friend: Sun, Mercury
            // Neutral: Mars, Jupiter, Venus, Saturn
            // Enemy: None
            // But let's map signs:
            // Sun(5), Merc(3,6) -> Friend
            // Mars(1,8), Jup(9,12), Ven(2,7), Sat(10,11) -> Neutral
            1 | 9 | 12 | 7 | 10 | 11 => Dignity::Neutral, 
            _ => Dignity::Friend, // 3, 5, 6
        },
        "Mars" => match sign_index {
            10 => Dignity::Exalted,     // Capricorn
            4 => Dignity::Debilitated,  // Cancer
            1 | 8 => Dignity::OwnSign,  // Aries, Scorpio
            // Friend: Sun (5), Moon (4-Deb), Jup (9,12) -> 5, 9, 12
            5 | 9 | 12 => Dignity::Friend,
            // Enemy: Mercury (3,6)
            3 | 6 => Dignity::Enemy,
            // Neutral: Venus (2,7), Saturn (11) (10 is Exalted) -> 2, 7, 11
            2 | 7 | 11 => Dignity::Neutral,
            _ => Dignity::Neutral,
        },
        "Mercury" => match sign_index {
            6 => Dignity::Exalted,      // Virgo (0-15 Exalted, 16-20 Mul, 21-30 Own) - Simplified to Exalted/Own
            12 => Dignity::Debilitated, // Pisces
            3 => Dignity::OwnSign,      // Gemini
            // Friend: Sun (5), Venus (2,7) -> 2, 5, 7
            2 | 5 | 7 => Dignity::Friend,
            // Enemy: Moon (4)
            4 => Dignity::Enemy,
            // Neutral: Mars (1,8), Jup (9,12), Sat (10,11)
            1 | 8 | 9 | 10 | 11 => Dignity::Neutral,
            _ => Dignity::Neutral,     
        },
        "Jupiter" => match sign_index {
            4 => Dignity::Exalted,      // Cancer
            10 => Dignity::Debilitated, // Capricorn
            9 | 12 => Dignity::OwnSign, // Sagittarius, Pisces
            // Friend: Sun (5), Moon (4-Ex), Mars (1,8) -> 1, 5, 8
            1 | 5 | 8 => Dignity::Friend,
            // Enemy: Merc (3,6), Ven (2,7)
            2 | 3 | 6 | 7 => Dignity::Enemy,
            // Neutral: Sat (11) (10 is Deb) -> 11
            11 => Dignity::Neutral,
            _ => Dignity::Neutral,
        },
        "Venus" => match sign_index {
            12 => Dignity::Exalted,     // Pisces
            6 => Dignity::Debilitated,  // Virgo
            2 | 7 => Dignity::OwnSign,  // Taurus, Libra
            // Friend: Merc (3,6-Deb), Sat (10,11) -> 3, 10, 11
            3 | 10 | 11 => Dignity::Friend,
            // Enemy: Sun (5), Moon (4)
            4 | 5 => Dignity::Enemy,
            // Neutral: Mars (1,8), Jup (9,12-Ex) -> 1, 8, 9
            1 | 8 | 9 => Dignity::Neutral,
            _ => Dignity::Neutral,
        },
        "Saturn" => match sign_index {
            7 => Dignity::Exalted,      // Libra
            1 => Dignity::Debilitated,  // Aries
            10 | 11 => Dignity::OwnSign,// Capricorn, Aquarius
            // Friend: Merc (3,6), Ven (2,7-Ex) -> 2, 3, 6
            2 | 3 | 6 => Dignity::Friend,
            // Enemy: Sun (5), Moon (4), Mars (1-Deb,8) -> 4, 5, 8
            4 | 5 | 8 => Dignity::Enemy,
            // Neutral: Jup (9,12)
            9 | 12 => Dignity::Neutral,
            _ => Dignity::Neutral,
        },
        "Rahu" | "Ketu" => Dignity::Neutral, // Simplified
        "Ascendant" => Dignity::Neutral,
        _ => Dignity::Neutral,
    }
}
