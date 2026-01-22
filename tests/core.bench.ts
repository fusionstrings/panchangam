import { p_calc_ut } from "../lib/panchangam.js";

// Basic benchmark to measure planetary calculation performance
Deno.bench("Swiss Ephemeris Calculation (Sun)", () => {
  const jd = 2451545.0;
  p_calc_ut(jd, 0, 2); // SE_SUN, SEFLG_SWIEPH
});

Deno.bench("Swiss Ephemeris Calculation (Moon)", () => {
  const jd = 2451545.0;
  p_calc_ut(jd, 1, 2); // SE_MOON, SEFLG_SWIEPH
});
