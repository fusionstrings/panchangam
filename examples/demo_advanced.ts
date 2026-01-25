import {
  calculate_chara_dasha_periods,
  calculate_full_shadbala,
  calculate_houses,
  calculate_jaimini_karakas,
  calculate_planets,
  calculate_varga,
  Location,
  p_julday,
} from "../lib/panchangam.js";

// --- Setup: Birth Date & Location ---
const year = 1990, month = 1, day = 1, hour = 12.0; // Noon UT
const jd = p_julday(year, month, day, hour, 1);
// Bangalore coordinates
const loc = new Location(12.9716, 77.5946, 920.0);

console.log("--- Advanced Vedic Astrology Demo ---");
console.log(`Date: ${year}-${month}-${day} ${hour}:00 UT`);

// --- 1. Basic Calculations ---
// Mode 1 = Lahiri Ayanamsha
const planets = calculate_planets(jd, 1);
const house_info = calculate_houses(jd, loc.latitude, loc.longitude, "W", 1); // Whole Sign
const ascendant = house_info.ascendant;

console.log(`Ascendant: ${ascendant.toFixed(2)}°`);

// --- 2. Varga Charts (Divisional Charts) ---
console.log("\n--- Varga Positions (Navamsha D9) ---");
// Config: D9 method = 0 (Parashara)
const d9_config = { d9_method: 0 };

planets.forEach((p: { name: string; longitude: number; dignity: string }) => {
  // Calculate D9 position
  const varga_pos = calculate_varga(p.longitude, 9, d9_config);
  // Varga sign (1-12)
  console.log(
    `${p.name}: D1=${p.longitude.toFixed(2)}° -> D9 Sign=${varga_pos.sign}`,
  );
});

// --- 3. Shadbala (Six-Fold Strength) ---
console.log("\n--- Shadbala (Strength) ---");
const shadbala = calculate_full_shadbala(planets, jd, ascendant);

// Helper to print strength
const print_strength = (
  name: string,
  data: { total_rupas: number; ishta_phala: number; kashta_phala: number },
) => {
  console.log(
    `${name}: ${data.total_rupas.toFixed(2)} Rupas (Ishta: ${
      data.ishta_phala.toFixed(2)
    })`,
  );
};

print_strength("Sun", shadbala.sun);
print_strength("Moon", shadbala.moon);
print_strength("Mars", shadbala.mars);
print_strength("Mercury", shadbala.mercury);
print_strength("Jupiter", shadbala.jupiter);
print_strength("Venus", shadbala.venus);
print_strength("Saturn", shadbala.saturn);

// --- 4. Jaimini Karakas ---
console.log("\n--- Jaimini Charakarakas (7 Karaka Scheme) ---");
// Jaimini functions expect tuples of [id, longitude]
const planet_input = planets.map((
  p: { id: number; longitude: number },
) => [p.id, p.longitude]);
const karakas = calculate_jaimini_karakas(planet_input, false); // use_8_karakas = false

const karaka_names = [
  "Atma",
  "Amatya",
  "Bhatri",
  "Matri",
  "Putra",
  "Gnati",
  "Dara",
];
karakas.forEach((k) => {
  // Find planet name by ID
  const p_name = planets.find((p: { id: number; name: string }) =>
    p.id === k.planet_id
  )?.name ||
    "Unknown";
  console.log(`${karaka_names[k.karaka_name] || "Unknown"} Karaka: ${p_name}`);
});

// --- 5. Chara Dasha ---
console.log("\n--- Chara Dasha Periods ---");
// Ascendant sign (1-12)
const asc_sign = Math.floor(ascendant / 30) + 1;
const periods = calculate_chara_dasha_periods(planet_input, asc_sign, year);

periods.slice(0, 5).forEach((p) => {
  console.log(
    `Sign ${p.sign_id}: ${p.start_year.toFixed(1)} - ${
      p.end_year.toFixed(1)
    } (${p.duration_years} yrs)`,
  );
});
