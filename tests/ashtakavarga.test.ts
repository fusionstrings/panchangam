import { assertEquals } from "@std/assert";
import {
  calculate_ashtakavarga,
  type Sarvashtakavarga,
} from "../lib/panchangam.js";

Deno.test("Ashtakavarga: Sarvashtakavarga Calculation", () => {
  // Test Case:
  // Ascendant: Aries (0 deg)
  // Sun: Aries (0 deg)
  // Moon: Aries (0 deg)
  // Mars: Aries (0 deg)
  // Mercury: Aries (0 deg)
  // Jupiter: Aries (0 deg)
  // Venus: Aries (0 deg)
  // Saturn: Aries (0 deg)
  // All planets in Aries (1st House).
  // This is an extreme case but easy to verify summed points.

  // Check Sun's AV contributions from Sun (1st house): 1, 2, 4, 7, 8, 9, 10, 11 (Houses 1, 2, 4, 7, 8, 9, 10, 11 receive points)
  // Since all planets are in 1st house (Aries), all counts are from Aries.

  // Example: Sun in 1st from Sun -> Point in 1 (Aries).
  // Sun in 1st from Moon -> Point in 3 (Gemini).
  // ...

  const planets = [
    { id: 0, longitude: 0.0, speed: 1.0 },
    { id: 1, longitude: 0.0, speed: 1.0 },
    { id: 2, longitude: 0.0, speed: 1.0 },
    { id: 3, longitude: 0.0, speed: 1.0 },
    { id: 4, longitude: 0.0, speed: 1.0 },
    { id: 5, longitude: 0.0, speed: 1.0 },
    { id: 6, longitude: 0.0, speed: 1.0 },
  ];

  const asc = 0.0;

  const sav: Sarvashtakavarga = calculate_ashtakavarga(planets, asc);
  console.log("SAV Totals:", sav.totals);

  // Verify specific totals if known logic holds.
  // Total Bindus across all 12 signs in SAV is usually 337.
  let sum = 0;
  for (const b of sav.totals) {
    sum += b;
  }
  assertEquals(sum, 337);

  // Check 11th House (Aquarius). 11th is usually strong.
  // Aquarius is index 10.
  console.log("11th House Points:", sav.totals[10]);
});
