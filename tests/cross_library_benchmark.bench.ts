/**
 * Performance Benchmark: swisseph-wasm vs swisseph-wasi
 *
 * Compares execution speed of planetary calculations.
 */

import {
  Constants,
  createSwissEph,
} from "@fusionstrings/swisseph-wasi/browser";
import { swe_calc_ut, swe_julday } from "../lib/panchangam.js";

const ITERATIONS = 1000;
const J2000 = 2451545.0;

// Initialize swisseph-wasi
const swissephWasi = await createSwissEph();

console.log("=".repeat(60));
console.log("Performance Benchmark: swisseph-wasm vs swisseph-wasi");
console.log(`Iterations: ${ITERATIONS}`);
console.log("=".repeat(60));

// Benchmark 1: Julian Day calculation
Deno.bench("swisseph-wasm: swe_julday", () => {
  swe_julday(2000, 1, 1, 12.0, 1);
});

Deno.bench("swisseph-wasi: swe_julday", () => {
  swissephWasi.swe_julday(2000, 1, 1, 12.0, Constants.SE_GREG_CAL);
});

// Benchmark 2: Sun position
Deno.bench("swisseph-wasm: Sun position", () => {
  swe_calc_ut(J2000, 0, 2);
});

Deno.bench("swisseph-wasi: Sun position", () => {
  swissephWasi.swe_calc_ut(J2000, Constants.SE_SUN, Constants.SEFLG_SWIEPH);
});

// Benchmark 3: Moon position (more complex)
Deno.bench("swisseph-wasm: Moon position", () => {
  swe_calc_ut(J2000, 1, 2);
});

Deno.bench("swisseph-wasi: Moon position", () => {
  swissephWasi.swe_calc_ut(J2000, Constants.SE_MOON, Constants.SEFLG_SWIEPH);
});

// Benchmark 4: All planets in single call
Deno.bench("swisseph-wasm: All 7 planets", () => {
  for (let planet = 0; planet <= 6; planet++) {
    swe_calc_ut(J2000, planet, 2);
  }
});

Deno.bench("swisseph-wasi: All 7 planets", () => {
  for (let planet = 0; planet <= 6; planet++) {
    swissephWasi.swe_calc_ut(J2000, planet, Constants.SEFLG_SWIEPH);
  }
});
