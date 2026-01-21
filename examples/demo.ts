import {
  calculate_daily_panchang,
  calculate_houses,
  calculate_planets,
  calculate_vimshottari,
  check_graha_yuddha,
  Location,
  swe_julday,
} from "../lib/panchangam.js";

// 1. Daily Panchang
const loc = new Location(28.6139, 77.2090, 225.0); // New Delhi
const date = new Date();
console.log(`Calculating for: ${date.toISOString()}`);

const result = calculate_daily_panchang(
  date.getFullYear(),
  date.getMonth() + 1,
  date.getDate(),
  loc,
  1, // Lahiri
);

console.log("\n--- Daily Panchang ---");
console.log(`Sunrise: ${new Date(result.sunrise).toLocaleTimeString()}`);
console.log(`Tithi: ${result.tithi_name} (${(result.tithi_index)}/30)`);
if (result.tithi_end_time) {
  console.log(`  Ends: ${new Date(result.tithi_end_time).toLocaleString()}`);
}

console.log(
  `Nakshatra: ${result.nakshatra_name} (${result.nakshatra_index}/27)`,
);
if (result.nakshatra_end_time) {
  console.log(
    `  Ends: ${new Date(result.nakshatra_end_time).toLocaleString()}`,
  );
}

console.log("\n--- Muhurats ---");
console.log(
  `Brahma Muhurta Start: ${
    new Date(result.muhurats.brahma_muhurta.start).toLocaleTimeString()
  }`,
);
console.log(
  `Rahu Kalam Start: ${
    new Date(result.muhurats.rahu_kalam.start).toLocaleTimeString()
  }`,
);

// 2. Planets & Dignity
console.log("\n--- Planetary Positions & Dignity ---");
const jd = swe_julday(
  date.getFullYear(),
  date.getMonth() + 1,
  date.getDate(),
  12.0,
  1,
);
interface PlanetData {
  name: string;
  longitude: number;
  dignity: string;
}

const planets = calculate_planets(jd, 1) as unknown as PlanetData[];

planets.forEach((p) => {
  console.log(
    `${p.name.padEnd(8)}: ${p.longitude.toFixed(2)}° | Dignity: ${p.dignity}`,
  );
});

// 3. Planetary War
console.log("\n--- Planetary War Check ---");
interface WarDetails {
  planet1_name: string;
  planet2_name: string;
  longitude_diff: number;
  winner_id: number;
}

const wars = check_graha_yuddha(jd, 1) as unknown as WarDetails[];
if (Array.isArray(wars) && wars.length > 0) {
  wars.forEach((w) => {
    console.log(
      `WAR: ${w.planet1_name} vs ${w.planet2_name} (Diff: ${
        w.longitude_diff.toFixed(4)
      }°) -> Winner: ID ${w.winner_id}`,
    );
  });
} else {
  console.log("No planetary wars detected.");
}

// 4. Houses
console.log("\n--- House Cusps (Placidus) ---");
const houses = calculate_houses(jd, loc.latitude, loc.longitude, "P", 1);
console.log(`Ascendant: ${houses.ascendant.toFixed(2)}°`);
houses.cusps.forEach((c: number, i: number) => {
  console.log(`House ${i + 1}: ${c.toFixed(2)}°`);
});

// 5. Vimshottari Dasha
console.log("\n--- Vimshottari Dasha (Example) ---");
const birth_moon = 45.5; // Rohini
const birth_time = date.getTime() - (30 * 365.25 * 24 * 3600 * 1000); // 30 years ago
const dasha = calculate_vimshottari(birth_moon, birth_time, date.getTime());
console.log(
  `Current Dasha: ${dasha.mahadasha} - ${dasha.antardasha} - ${dasha.pratyantardasha}`,
);
console.log(
  `Ends: ${new Date(dasha.pratyantardasha_end_date).toLocaleDateString()}`,
);
