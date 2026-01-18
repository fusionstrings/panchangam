/**
 * Cross-Library Verification Script
 *
 * Compares swisseph-wasm (via panchangam) against swisseph-wasi
 * to verify accuracy and precision.
 *
 * swisseph-wasm returns: { longitude, latitude, distance, ... }
 * swisseph-wasi returns: { xx: Float64Array, ... } where xx[0]=longitude
 */

import { assertAlmostEquals, assertEquals } from "@std/assert";
import {
  Constants,
  createSwissEph,
} from "@fusionstrings/swisseph-wasi/browser";
import { swe_calc_ut, swe_julday } from "../lib/panchangam.js";

const TOLERANCE = 0.01; // 0.01 degree tolerance

// Initialize swisseph-wasi
const swissephWasi = await createSwissEph();

console.log("=".repeat(60));
console.log("Cross-Library Verification: swisseph-wasm vs swisseph-wasi");
console.log("=".repeat(60));

// Test cases: planets at J2000
const TEST_CASES = [
  { name: "Sun at J2000", planet: Constants.SE_SUN },
  { name: "Moon at J2000", planet: Constants.SE_MOON },
  { name: "Mercury at J2000", planet: Constants.SE_MERCURY },
  { name: "Venus at J2000", planet: Constants.SE_VENUS },
  { name: "Mars at J2000", planet: Constants.SE_MARS },
  { name: "Jupiter at J2000", planet: Constants.SE_JUPITER },
  { name: "Saturn at J2000", planet: Constants.SE_SATURN },
];

const J2000 = 2451545.0;

// Test 1: Julian Day calculation
Deno.test("Julian Day calculation matches", () => {
  const wasmJd = swe_julday(2000, 1, 1, 12.0, 1); // SE_GREG_CAL = 1
  const wasiJd = swissephWasi.swe_julday(
    2000,
    1,
    1,
    12.0,
    Constants.SE_GREG_CAL,
  );

  console.log(`  swisseph-wasm JD: ${wasmJd}`);
  console.log(`  swisseph-wasi JD: ${wasiJd}`);

  assertEquals(wasmJd, wasiJd, "Julian Day should be identical");
});

// Test 2: Planetary positions comparison
for (const tc of TEST_CASES) {
  Deno.test(`${tc.name}: positions match`, () => {
    // swisseph-wasm (panchangam) - returns { longitude, latitude, ... }
    const wasmResult = swe_calc_ut(J2000, tc.planet, 2); // SEFLG_SWIEPH = 2
    const wasmLongitude = wasmResult.longitude;

    // swisseph-wasi - returns { xx: Float64Array } where xx[0] = longitude
    const wasiResult = swissephWasi.swe_calc_ut(
      J2000,
      tc.planet,
      Constants.SEFLG_SWIEPH,
    );
    const wasiLongitude = wasiResult.xx[0];

    console.log(`  ${tc.name}:`);
    console.log(`    swisseph-wasm: lon=${wasmLongitude.toFixed(6)}`);
    console.log(`    swisseph-wasi: lon=${wasiLongitude.toFixed(6)}`);
    const diff = Math.abs(wasmLongitude - wasiLongitude);
    console.log(
      `    difference: ${diff.toFixed(8)} deg (${
        (diff * 3600).toFixed(2)
      } arcsec)`,
    );

    assertAlmostEquals(
      wasmLongitude,
      wasiLongitude,
      TOLERANCE,
      `${tc.name} longitude should match within ${TOLERANCE} deg`,
    );
  });
}

// Test 3: Edge cases - ancient and future dates
Deno.test("Ancient date (1000 BCE) calculation", () => {
  const ancientJd = swe_julday(-1000, 1, 1, 12.0, 1);
  const wasmSun = swe_calc_ut(ancientJd, Constants.SE_SUN, 2);
  const wasiSun = swissephWasi.swe_calc_ut(
    ancientJd,
    Constants.SE_SUN,
    Constants.SEFLG_SWIEPH,
  );

  console.log(`  Ancient date Sun:`);
  console.log(`    swisseph-wasm: ${wasmSun.longitude.toFixed(6)}`);
  console.log(`    swisseph-wasi: ${wasiSun.xx[0].toFixed(6)}`);

  assertAlmostEquals(wasmSun.longitude, wasiSun.xx[0], TOLERANCE);
});

Deno.test("Future date (3000 CE) calculation", () => {
  const futureJd = swe_julday(3000, 1, 1, 12.0, 1);
  const wasmSun = swe_calc_ut(futureJd, Constants.SE_SUN, 2);
  const wasiSun = swissephWasi.swe_calc_ut(
    futureJd,
    Constants.SE_SUN,
    Constants.SEFLG_SWIEPH,
  );

  console.log(`  Future date Sun:`);
  console.log(`    swisseph-wasm: ${wasmSun.longitude.toFixed(6)}`);
  console.log(`    swisseph-wasi: ${wasiSun.xx[0].toFixed(6)}`);

  assertAlmostEquals(wasmSun.longitude, wasiSun.xx[0], TOLERANCE);
});

console.log("\n✅ All cross-library verifications complete!");
