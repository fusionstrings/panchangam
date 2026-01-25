import { assertAlmostEquals, assertEquals } from "@std/assert";
import {
  calculate_houses,
  calculate_vimshottari,
  Constants,
  // Location,
  p_julday,
  tithi_end_time,
  tithi_start_time,
} from "../lib/panchangam.js";

// --- DASHA EDGE CASES ---

Deno.test("Dasha: Cycle Wrap-around", () => {
  // Mercury (last dasha, 17y) -> Should wrap to Ketu (first dasha)
  // Revati (last nakshatra) maps to Mercury dasha.
  // Moon at 360.0 (or 0.0) is transition.

  // Revati 4th pada end (360 deg) -> End of Mercury Dasha.
  const _moonLong = 359.99; // End of Revati
  const _birthMs = 0; // Epoch
  // 17 years into Mercury dasha means start of Ketu?
  // Let's test basic planet cycle

  // Calculate for today
  // If born with Moon at 0.0 (Ashwini start), starts Ketu 0.0.
  // 120 years later, should be Ketu again.

  const cycleMs = 120 * 365.25 * 24 * 3600 * 1000;
  const res = calculate_vimshottari(0.0, 0, cycleMs + 1000);

  // Should be back to Ketu
  assertEquals(res.mahadasha, "Ketu");
  console.log("120 year cycle check passed");
});

Deno.test("Dasha: Fractional Balance Precision", () => {
  // Moon exactly in middle of Ashwini (Ketu 7y)
  // Ashwini span 13.33 deg. Middle ~6.66 deg.
  // Should have 3.5 years remaining.
  const nakSpan = 360 / 27;
  const moonLong = nakSpan / 2.0;

  const res = calculate_vimshottari(moonLong, 0, 0);
  // Total 7 years. 50% passed. Remaining should be ~3.5
  // Current time is 0. So end date is duration from now.
  const msPerYear = 365.25 * 24 * 3600 * 1000;
  const remainingYears = res.mahadasha_end_date / msPerYear;
  assertAlmostEquals(remainingYears, 3.5, 0.1);
});

// --- TITHI EDGE CASES ---

Deno.test("Tithi: 30th Tithi (Amavasya) Boundary", () => {
  // Amavasya is when Moon-Sun ~ 348-360 deg (idx 30)
  // Next is Shukla Pratipada (idx 1).
  // Test transition handling (0/360 wrap)

  // Find a known Amavasya
  // 2026-01-18 is Amavasya (idx 30)
  const jd = p_julday(2026, 1, 19, 0, Constants.SE_GREG_CAL);
  const start = tithi_start_time(jd);

  // Ensure start calculation didn't loop infinitely or fail
  assertEquals(start < jd, true);

  // Also check end time robustness
  const end = tithi_end_time(jd); // Should be next Pratipada start
  assertEquals(end > jd, true);
  // Length check (ensure no huge jump)
  const length = end - start;
  assertEquals(length > 0.8 && length < 1.1, true);

  console.log("Tithi wrap-around safe");
});

// --- ASCENDANT EDGE CASES ---

Deno.test("Ascendant: Polar Latitude", () => {
  // High latitude (e.g., 80N) where houses can be weird
  const jd = 2461058.5;
  const houses = calculate_houses(jd, 80.0, 0.0, "P", 1);

  // Check for NaN
  if (isNaN(houses.ascendant)) {
    throw new Error("Ascendant is NaN at high latitude");
  }
  console.log("Polar Ascendant:", houses.ascendant);
});
