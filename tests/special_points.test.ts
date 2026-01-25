import { assertAlmostEquals, assertEquals } from "@std/assert";
import {
  calculate_houses,
  calculate_planets,
  calculate_prastara_ashtakavarga,
  calculate_special_lagnas,
  calculate_sunrise,
  Location as LibLocation,
  p_julday,
} from "../lib/panchangam.js";

interface TestPlanet {
  id: number;
  longitude: number;
}

Deno.test("Special Lagnas: Hora, Ghati, Sree Lagna", () => {
  // Sample Birth: 1985-04-24 12:00:00 (Local Time)
  // Location: 13.0827 N, 80.2707 E (Chennai)
  // Lahiri Ayanamsha (mode 1)

  const lat = 13.0827;
  const lon = 80.2707;
  const year = 1985, month = 4, day = 24, _hour = 12.0;

  // JD for 12:00 UT
  // Birth at 12:00 Local (IST = UT+5.5) -> UT = 6.5
  const ut = 6.5;
  const birth_jd = p_julday(year, month, day, ut, 1);

  // Sunrise at 05:54 IST -> 00.4 UT
  // We use the library's calculate_sunrise for consistency
  const loc = new LibLocation(lat, lon, 0);
  const sunrise_ms = calculate_sunrise(year, month, day, loc);
  // Convert ms to JD: (ms / 86400000) + 2440587.5
  const sunrise_jd = (sunrise_ms / 86400000) + 2440587.5;

  // Calculate Sun at Sunrise
  // We need planets at sunrise_jd
  const plants_at_sunrise = calculate_planets(
    sunrise_jd,
    1,
  ) as unknown as TestPlanet[];
  const sun_sunrise = plants_at_sunrise.find((p) => p.id === 0)?.longitude || 0;

  // Calculate Lagna and Moon at birth
  const house_info = calculate_houses(birth_jd, lat, lon, "W", 1);
  const lagna = house_info.ascendant;

  const planets_at_birth = calculate_planets(
    birth_jd,
    1,
  ) as unknown as TestPlanet[];
  const moon_birth = planets_at_birth.find((p) => p.id === 1)?.longitude || 0;

  const result = calculate_special_lagnas(
    birth_jd,
    sunrise_jd,
    sun_sunrise,
    lagna,
    moon_birth,
  );

  console.log("Special Lagnas result:", result);

  // Verification logic
  // Diff in hours
  const diff_hours = (birth_jd - sunrise_jd) * 24;

  // HL should be Sun + hours * 30
  const expected_hl = (sun_sunrise + diff_hours * 30.0) % 360;
  assertAlmostEquals(result.hora_lagna, expected_hl, 0.001);

  // GL should be Sun + hours * 60
  const expected_gl = (sun_sunrise + diff_hours * 60.0) % 360;
  assertAlmostEquals(result.ghati_lagna, expected_gl, 0.001);

  // SL should be Lagna + (Moon portion * 360)
  const nk_span = 360 / 27;
  const expected_sl = (lagna + (moon_birth % nk_span) / nk_span * 360) % 360;
  assertAlmostEquals(result.sree_lagna, expected_sl, 0.001);
});

Deno.test("Prastara Ashtakavarga: Sun", () => {
  const planet_longs = [
    { id: 0, longitude: 10.0, speed: 1.0 }, // Sun
    { id: 1, longitude: 40.0, speed: 12.0 }, // Moon
    { id: 2, longitude: 70.0, speed: 0.5 }, // Mars
    { id: 3, longitude: 100.0, speed: 1.5 }, // Merc
    { id: 4, longitude: 130.0, speed: 0.2 }, // Jup
    { id: 5, longitude: 160.0, speed: 1.2 }, // Ven
    { id: 6, longitude: 190.0, speed: 0.1 }, // Sat
  ];
  const ascendant = 5.0; // Aries

  const result = calculate_prastara_ashtakavarga(0, planet_longs, ascendant);

  console.log("Sun Prastara grid (first sign):", result.grid.slice(0, 12));

  // Row 0 is Sun from Sun: [1, 2, 4, 7, 8, 9, 10, 11]
  // Sun sign is (10/30) = 0 (Aries)
  // Expected grid for Row 0:
  // Sign 0 (1): 1
  // Sign 1 (2): 1
  // Sign 2 (3): 0
  // Sign 3 (4): 1
  // Sign 4 (5): 0
  // Sign 5 (6): 0
  // Sign 6 (7): 1
  // Sign 7 (8): 1
  // Sign 8 (9): 1
  // Sign 9 (10): 1
  // Sign 10 (11): 1
  // Sign 11 (12): 0

  const row0 = result.grid.slice(0, 12);
  const expected_row0 = [1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0];

  for (let i = 0; i < 12; i++) {
    assertEquals(row0[i], expected_row0[i], `Row 0 Sign ${i} mismatch`);
  }
});
