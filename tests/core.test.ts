import { assertAlmostEquals, assertEquals } from "@std/assert";
import {
  calculate_daily_panchang,
  get_swisseph_version,
  get_version,
  Location,
  p_calc_ut,
} from "../lib/panchangam.js";

Deno.test("Swiss Ephemeris Version", () => {
  const ver = get_swisseph_version();
  console.log(`Swiss Eph Version: ${ver}`);
  assertEquals(ver, "2.10.03");
});

Deno.test("Library Version", () => {
  const ver = get_version();
  console.log(`Panchangam Version: ${ver}`);

  // Dynamic check against deno.json
  const denoConfig = JSON.parse(
    Deno.readTextFileSync(new URL("../deno.json", import.meta.url)),
  );
  assertEquals(ver, denoConfig.version);
});

Deno.test("Sunrise/Sunset Calculation", () => {
  // Bangalore
  const loc = new Location(12.9716, 77.5946, 920.0);
  // Jan 5, 2026
  const sunrise_ms = calculate_daily_panchang(2026, 1, 5, loc, 1).sunrise;
  const date = new Date(sunrise_ms);

  // Expected around 6:40 AM IST (UTC+5:30)
  // 6:40 AM IST = 1:10 AM UTC
  assertEquals(date.getUTCHours(), 1);
  assertAlmostEquals(date.getUTCMinutes(), 11, 2); // Allow +/- 2 mins
});

Deno.test("Planetary Position (Sun)", () => {
  // J2000
  const jd = 2451545.0;
  const res = p_calc_ut(jd, 0, 2); // SE_SUN, SEFLG_SWIEPH

  // Sun at J2000 should be near 280 deg longitude (Capricorn)
  // Actually J2000 epoch is defined by standard positions.
  // Mean longitude approx 280.46
  assertAlmostEquals(res.longitude, 280.46, 1.0);
});
