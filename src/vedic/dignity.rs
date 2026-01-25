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
/// * `longitude` - Sidereal longitude in degrees (0-360)
pub fn calculate_dignity(planet_name: &str, longitude: f64) -> Dignity {
    let sign_index = ((longitude / 30.0).floor() as u8 % 12) + 1; // 1-12
    let deg_in_sign = longitude % 30.0;
    
    // Basic Vedic table (Parashara)
    // 1-Aries, 2-Taurus, 3-Gemini, 4-Cancer, 5-Leo, 6-Virgo,
    // 7-Libra, 8-Scorpio, 9-Sagittarius, 10-Capricorn, 11-Aquarius, 12-Pisces
    
    match planet_name {
        "Sun" => match sign_index {
            1 => Dignity::Exalted,      // Aries
            7 => Dignity::Debilitated,  // Libra
            5 => {
                if deg_in_sign <= 20.0 { Dignity::Moolatrikona }
                else { Dignity::OwnSign }
            },
            4 | 8 | 9 | 12 => Dignity::Friend,
            2 | 10 | 11 => Dignity::Enemy,
            3 | 6 => Dignity::Neutral,
            _ => Dignity::Neutral,
        },
        "Moon" => match sign_index {
            2 => {
                if deg_in_sign <= 3.0 { Dignity::Exalted }
                else { Dignity::Moolatrikona }
            },
            8 => Dignity::Debilitated,
            4 => Dignity::OwnSign,
            5 | 3 | 6 => Dignity::Friend,
            1 | 9 | 12 | 7 | 10 | 11 => Dignity::Neutral, 
            _ => Dignity::Friend,
        },
        "Mars" => match sign_index {
            10 => Dignity::Exalted,
            4 => Dignity::Debilitated,
            1 => {
                if deg_in_sign <= 12.0 { Dignity::Moolatrikona }
                else { Dignity::OwnSign }
            },
            8 => Dignity::OwnSign,
            5 | 9 | 12 => Dignity::Friend,
            3 | 6 => Dignity::Enemy,
            2 | 7 | 11 => Dignity::Neutral,
            _ => Dignity::Neutral,
        },
        "Mercury" => match sign_index {
            6 => {
                if deg_in_sign <= 15.0 { Dignity::Exalted }
                else if deg_in_sign <= 20.0 { Dignity::Moolatrikona }
                else { Dignity::OwnSign }
            },
            12 => Dignity::Debilitated,
            3 => Dignity::OwnSign,
            2 | 5 | 7 => Dignity::Friend,
            4 => Dignity::Enemy,
            1 | 8 | 9 | 10 | 11 => Dignity::Neutral,
            _ => Dignity::Neutral,     
        },
        "Jupiter" => match sign_index {
            4 => Dignity::Exalted,
            10 => Dignity::Debilitated,
            9 => {
                if deg_in_sign <= 10.0 { Dignity::Moolatrikona }
                else { Dignity::OwnSign }
            },
            12 => Dignity::OwnSign,
            1 | 5 | 8 => Dignity::Friend,
            2 | 3 | 6 | 7 => Dignity::Enemy,
            11 => Dignity::Neutral,
            _ => Dignity::Neutral,
        },
        "Venus" => match sign_index {
            12 => Dignity::Exalted,
            6 => Dignity::Debilitated,
            7 => {
                if deg_in_sign <= 15.0 { Dignity::Moolatrikona }
                else { Dignity::OwnSign }
            },
            2 => Dignity::OwnSign,
            3 | 10 | 11 => Dignity::Friend,
            4 | 5 => Dignity::Enemy,
            1 | 8 | 9 => Dignity::Neutral,
            _ => Dignity::Neutral,
        },
        "Saturn" => match sign_index {
            7 => Dignity::Exalted,
            1 => Dignity::Debilitated,
            11 => {
                if deg_in_sign <= 20.0 { Dignity::Moolatrikona }
                else { Dignity::OwnSign }
            },
            10 => Dignity::OwnSign,
            2 | 3 | 6 => Dignity::Friend,
            4 | 5 | 8 => Dignity::Enemy,
            9 | 12 => Dignity::Neutral,
            _ => Dignity::Neutral,
        },
        _ => Dignity::Neutral,
    }
}

/// Planetary Relationship Level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Relationship {
    PermanentFriend = 1,
    Neutral = 0,
    PermanentEnemy = -1,
}

/// Naisargika Maitri (Permanent Relationship)
pub fn get_naisargika_relationship(p1: i32, p2: i32) -> Relationship {
    // Standard table (Parashara)
    // Planets: 0=Sun, 1=Moon, 2=Mars, 3=Mer, 4=Jup, 5=Ven, 6=Sat
    if p1 == p2 { return Relationship::Neutral; }
    
    match p1 {
        0 => match p2 { // Sun
            1 | 2 | 4 => Relationship::PermanentFriend,
            5 | 6 => Relationship::PermanentEnemy,
            _ => Relationship::Neutral, // Mer
        },
        1 => match p2 { // Moon
            0 | 3 => Relationship::PermanentFriend,
            _ => Relationship::Neutral,
        },
        2 => match p2 { // Mars
            0 | 1 | 4 => Relationship::PermanentFriend,
            3 => Relationship::PermanentEnemy,
            _ => Relationship::Neutral,
        },
        3 => match p2 { // Mercury
            0 | 5 => Relationship::PermanentFriend,
            1 => Relationship::PermanentEnemy,
            _ => Relationship::Neutral,
        },
        4 => match p2 { // Jupiter
            0 | 1 | 2 => Relationship::PermanentFriend,
            3 | 5 => Relationship::PermanentEnemy,
            _ => Relationship::Neutral,
        },
        5 => match p2 { // Venus
            3 | 6 => Relationship::PermanentFriend,
            0 | 1 => Relationship::PermanentEnemy,
            _ => Relationship::Neutral,
        },
        6 => match p2 { // Saturn
            3 | 5 => Relationship::PermanentFriend,
            0..=2 => Relationship::PermanentEnemy,
            _ => Relationship::Neutral,
        },
        _ => Relationship::Neutral,
    }
}

/// Tatkalika Maitri (Temporary Relationship)
pub fn get_tatkalika_relationship(s1: u8, s2: u8) -> Relationship {
    if s1 == s2 { return Relationship::PermanentEnemy; } // Self is enemy? No, usually not defined for self.
    
    // 1-based signs.
    let diff = (s2 as i32 - s1 as i32).rem_euclid(12);
    // Friends: 2, 3, 4, 10, 11, 12 from self.
    // In index terms (0-11): 1, 2, 3, 9, 10, 11.
    match diff {
        1 | 2 | 3 | 9 | 10 | 11 => Relationship::PermanentFriend,
        _ => Relationship::PermanentEnemy,
    }
}

/// Panchadha Maitri (Five-Fold Relationship)
pub fn get_panchadha_relationship(p1_id: i32, p1_sign: u8, p2_id: i32, p2_sign: u8) -> Dignity {
    let perm = get_naisargika_relationship(p1_id, p2_id);
    let temp = get_tatkalika_relationship(p1_sign, p2_sign);
    
    let score = perm as i32 + temp as i32;
    // Score map:
    // 1 + 1 = 2 (Great Friend)
    // 1 + 0 = 1 (Friend)
    // 0 + 1 = 1 (Friend) -- Wait, Neutral + Friend = Friend.
    // 0 + 0 = 0 (Neutral) -- Actually, perm is never 0 in some tables? No, Mer is neutral to Sun.
    // 0 - 1 = -1 (Enemy)
    // -1 + 0 = -1 (Enemy)
    // -1 - 1 = -2 (Great Enemy)
    
    // Result mapping to Dignity levels:
    match score {
        2 => Dignity::GreatFriend,
        1 => Dignity::Friend,
        0 => Dignity::Neutral,
        -1 => Dignity::Enemy,
        -2 => Dignity::GreatEnemy,
        _ => Dignity::Neutral,
    }
}


