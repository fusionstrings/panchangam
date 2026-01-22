import { assertAlmostEquals, assertEquals } from "@std/assert";
import {
  calculate_daily_panchang,
  calculate_houses,
  calculate_vimshottari,
  Constants,
  julday,
  Location,
  tithi_end_time,
  tithi_start_time,
} from "../lib/panchangam.js";

// Basic config
const date = { year: 2026, month: 1, day: 18, hour: 12.0 };
const loc = new Location(28.6139, 77.2090, 0); // New Delhi
const jd = julday(
  date.year,
  date.month,
  date.day,
  date.hour,
  Constants.SE_GREG_CAL,
);

Deno.test("Constants Export", () => {
  assertEquals(Constants.SE_SUN, 0);
  assertEquals(Constants.SE_MOON, 1);
  assertEquals(Constants.SEFLG_SWIEPH, 2);
  assertEquals(Constants.SEFLG_SPEED, 256);
});

Deno.test("Ascendant Calculation", () => {
  // 2026-01-18 12:00 UT at New Delhi
  // Just verify it returns numbers and structure
  const houses = calculate_houses(jd, loc.latitude, loc.longitude, "P", 1);
  console.log("Ascendant:", houses.ascendant);
  console.log("MC:", houses.mc);

  // Valid range check (0-360)
  assertEquals(houses.ascendant >= 0 && houses.ascendant < 360, true);
  assertEquals(houses.cusps.length, 12);
});

Deno.test("Vimshottari Dasha", () => {
  // Moon longitude test: 0 degrees (Ashwini) -> Ketu Dasha
  const moonLong = 0.5; // Ashwini 1st pada
  const birthMs = Date.now() - (1000 * 3600 * 24 * 365 * 20); // 20 years ago
  const currentMs = Date.now();

  const dasha = calculate_vimshottari(moonLong, birthMs, currentMs);
  console.log(
    "Dasha:",
    dasha.mahadasha,
    dasha.antardasha,
    dasha.nakshatra_name,
  );

  assertEquals(dasha.nakshatra_name, "Ashwini");
  // Ketu (7y) -> Venus (20y). 20 years later should be Venus mahadasha?
  // Ketu 7 years. Venus 20 years. Total 27 years.
  // If born 20 years ago, should be in Venus dasha.
  assertEquals(dasha.mahadasha, "Venus");
});

Deno.test("Tithi Boundaries", () => {
  const start = tithi_start_time(jd);
  const end = tithi_end_time(jd);

  console.log(`Tithi Start JD: ${start}`);
  console.log(`Current JD:     ${jd}`);
  console.log(`Tithi End JD:   ${end}`);

  assertEquals(start < jd, true);
  assertEquals(end > jd, true);
  // Tithi length approx 0.9 to 1.0 day usually
  assertEquals(end - start > 0.8, true);
  assertEquals(end - start < 1.1, true);
});

Deno.test("Karana in Panchang", () => {
  const panchang = calculate_daily_panchang(2026, 1, 18, loc, 1);
  console.log("Karana:", panchang.karana_name, panchang.karana_index);
  assertEquals(typeof panchang.karana_name, "string");
  assertEquals(panchang.karana_index > 0, true);
});
