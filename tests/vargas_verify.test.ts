import { assertAlmostEquals, assertEquals } from "@std/assert";
import {
  calculate_varga,
  D10Variation,
  D2Variation,
  D3Variation,
  D9Variation,
  VargaType,
} from "../lib/panchangam.js";

// Helper to convert deg to Sign/Deg
function toSignDeg(fullDeg: number) {
  const s = Math.floor(fullDeg / 30) + 1;
  const d = fullDeg % 30;
  return { sign: s, deg: d };
}

Deno.test("Varga: D1 (Rashi) check", () => {
  // 45 degrees = 15 deg Taurus (Sign 2)
  const res = calculate_varga(45.0, 1, undefined);
  assertEquals(res.sign, 2);
  assertAlmostEquals(res.longitude, 15.0);
});

Deno.test("Varga: D9 (Navamsha) Parashara Standard", () => {
  // Example: Sun at 10 deg Aries (Aries is Movable).
  // 10 deg is in the 4th Navamsa (0-3.20, 3.20-6.40, 6.40-10.00).
  // Wait, 10.0 is exactly the boundary? let's use 10.1 (4th part).
  // Parts: 1 (0-3.33), 2 (3.33-6.66), 3 (6.66-10), 4 (10-13.33).
  // Note: Rust code uses (deg / (30/9)).

  // Aries Movable: Starts from Aries(1).
  // 4th Navamsa of Aries = Cancer(4).

  const res = calculate_varga(11.0, 9, undefined);
  // 11 deg is in 4th Navamsa (10deg to 13deg 20m).
  // Sign should be Cancer(4).
  console.log("11 deg Aries D9:", res);
  assertEquals(res.sign, 4);

  // Example: 1 deg Taurus (Fixed).
  // Fixed starts from 9th from Sign => 9th from Taurus(2) is Capricorn(10).
  // 1st Navamsa of Taurus => Capricorn(10).
  const res2 = calculate_varga(31.0, 9, undefined); // 30 + 1
  console.log("1 deg Taurus D9:", res2);
  assertEquals(res2.sign, 10);
});

Deno.test("Varga: D3 (Drekkana) Variations", () => {
  // Sun at 15 deg Aries. (2nd Drekkana).

  // Config as plain object or undefined
  let config: any = { d3_method: D3Variation.Parashara };

  const pRes = calculate_varga(15.0, 3, config);
  assertEquals(pRes.sign, 5); // Leo

  // Jagannatha
  config.d3_method = D3Variation.Jagannatha;
  const jRes = calculate_varga(45.0, 3, config); // 15 deg Taurus
  console.log("Jagannatha D3 Taurus 2nd part:", jRes.sign);
  assertEquals(jRes.sign, 2);

  // Parashara again
  config.d3_method = D3Variation.Parashara;
  const pRes2 = calculate_varga(45.0, 3, config);
  assertEquals(pRes2.sign, 6);
});

Deno.test("Varga: D60 (Shashtiamsha)", () => {
  // 10.05 degrees Aries.
  // Part = floor(10.05 / 0.5) = 20.
  // 0-based index 20 is the 21st Shashtiamsha.
  // Count from Sign(1). 1 + 20 = 21. 21 % 12 = 9 (Sagittarius).

  const res = calculate_varga(10.05, 60, undefined);
  console.log("10.05 Aries D60:", res.sign);

  // (0 idx + 20) % 12 = 8 -> Sign index 8 is Sagittarius(9).
  assertEquals(res.sign, 9);
});
