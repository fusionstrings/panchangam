import {
  calculate_ashtakavarga,
  calculate_houses,
  calculate_planets,
  calculate_prastara_ashtakavarga,
  calculate_reduced_ashtakavarga,
  calculate_special_lagnas,
  calculate_sunrise,
  Location,
  p_julday,
} from "../lib/panchangam.js";

// --- Setup ---
const year = 2024, month = 1, day = 15;
const lat = 28.6139, lon = 77.2090; // New Delhi
const loc = new Location(lat, lon, 225.0);

// Calculate Sunrise for Special Lagnas
const sunrise_ms = calculate_sunrise(year, month, day, loc);
const sunrise_jd = (sunrise_ms / 86400000.0) + 2440587.5;

// Birth Time: 10:30 AM Local
// Sunrise is approx 7:15 AM.
const birth_hour_local = 10.5;
const ist_offset = 5.5;
const birth_hour_ut = birth_hour_local - ist_offset;

const birth_jd = p_julday(year, month, day, birth_hour_ut, 1);

// Get Planets
const planets = calculate_planets(birth_jd, 1); // Lahiri
const house_info = calculate_houses(birth_jd, lat, lon, "W", 1);
const ascendant = house_info.ascendant;

console.log("--- Ashtakavarga & Special Points Demo ---");

// --- 1. Ashtakavarga ---
console.log("\n1. Sarvashtakavarga (SAV) Totals:");
const sav = calculate_ashtakavarga(planets, ascendant);
// Array of 12 numbers (Aries to Pisces)
console.log("SAV:", sav.totals);

// --- 2. Reductions & Shodya Pinda ---
console.log("\n2. Reductions & Shodya Pinda (from SAV):");
// We can run reduction on SAV totals often used for transits,
// OR run on individual BAVs. Here we show SAV reduction for demo.
// calculate_reduced_ashtakavarga expects tuples [id, long]
const planet_tuples = planets.map((p: any) => [p.id, p.longitude]);
const reduced = calculate_reduced_ashtakavarga(sav.totals, planet_tuples);
console.log("Reduced Bindus:", reduced.reduced_bindus);
console.log("Shodya Pinda:", reduced.shodaya_pinda);

// --- 3. Prastara Ashtakavarga ---
console.log("\n3. Prastara Ashtakavarga (Sun):");
// Target Planet ID: 0 (Sun)
const prastara = calculate_prastara_ashtakavarga(0, planets, ascendant);
// 8 rows x 12 signs = 96 elements.
const grid = prastara.grid;
console.log("Prastara Grid Length:", grid.length);
// Show Row 0 (Sun's contribution to Sun's AV)
console.log("Row 0 (Sun->Sun):", grid.slice(0, 12));

// --- 4. Special Lagnas ---
console.log("\n4. Special Lagnas:");
// Need Sun at Sunrise
const planets_sunrise = calculate_planets(sunrise_jd, 1);
// Manually cast or find
// In JS from WASM, calculate_planets returns array of objects.
const sun_obj = (planets_sunrise as any[]).find((p: any) => p.id === 0);
const sun_sunrise = sun_obj ? sun_obj.longitude : 0;

// Need Moon at Birth
const moon_obj = (planets as any[]).find((p: any) => p.id === 1);
const moon_birth = moon_obj ? moon_obj.longitude : 0;

const special_lagnas = calculate_special_lagnas(
  birth_jd,
  sunrise_jd,
  sun_sunrise,
  ascendant,
  moon_birth,
);

const to_deg = (v: number) => v.toFixed(2);

console.log(`Hora Lagna (HL): ${to_deg(special_lagnas.hora_lagna)}°`);
console.log(`Ghati Lagna (GL): ${to_deg(special_lagnas.ghati_lagna)}°`);
console.log(`Sree Lagna (SL): ${to_deg(special_lagnas.sree_lagna)}°`);
