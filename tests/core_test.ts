import { assertEquals } from "@std/assert";
import {
  calculate_sunrise,
  calculate_tithi,
  get_swisseph_version,
  get_version,
  Location,
} from "../lib/panchangam.js";

Deno.test("Swiss Ephemeris Version Check", () => {
  const version = get_swisseph_version();
  console.log("Swiss Ephemeris Version:", version);
  // Version should be 2.10.something
  assertEquals(version.startsWith("2.10"), true);
});

Deno.test("Tithi Calculation Check", () => {
  // Julian Day for some known date (e.g. 2024-01-01)
  // JD for 2024-01-01 12:00 UTC = 2460311.0
  const jd = 2460311.0;
  const tithi = calculate_tithi(jd);
  console.log("Tithi Info:", tithi);

  // Validate fields exist
  assertEquals(typeof tithi.index, "number");
  assertEquals(typeof tithi.name, "string");
  assertEquals(typeof tithi.completion, "number");
});

Deno.test("Sunrise Calculation Check (SPA)", () => {
  // Delhi coordinates: 28.6139° N, 77.2090° E
  const lat = 28.6139;
  const lon = 77.2090;
  const year = 2024;
  const month = 1;
  const day = 1;

  // Altitude 0 for simplicity
  const loc = new Location(lat, lon, 0.0);
  console.log("Location instance:", loc);
  console.log("Is instance of Location?", loc instanceof Location);

  // Check if we can access fields
  console.log("Lat:", loc.latitude);

  const sunrise = calculate_sunrise(year, month, day, loc);
  console.log("Sunrise Timestamp:", sunrise);

  // Should be a valid timestamp (greater than 0)
  assertEquals(sunrise > 0, true);

  const date = new Date(sunrise);
  console.log("Sunrise Date:", date.toISOString());
});
