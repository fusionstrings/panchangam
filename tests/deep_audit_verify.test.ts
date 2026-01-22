import {
  assertArrayIncludes,
  assertEquals,
  assertNotEquals,
} from "@std/assert";
import {
  calculate_daily_panchang,
  calculate_houses,
  calculate_planets,
  Constants,
  Location,
} from "../lib/panchangam.js";

const loc = new Location(28.6139, 77.2090, 0); // New Delhi
const jd = 2461059.0; // 2026-01-18 12:00 UT

Deno.test("Deep Audit: Ayanamsha Modes & IDs", () => {
  // Lahiri should be 1
  assertEquals(Constants.SE_SIDM_LAHIRI, 1);
  assertEquals(Constants.SE_SIDM_RAMAN, 3);
  assertEquals(Constants.SE_SIDM_KRISHNAMURTI, 5);
  assertEquals(Constants.SE_SIDM_YUKTESHWAR, 7);
  assertEquals(Constants.SE_SIDM_JN_BHASIN, 8);
  assertEquals(Constants.SE_SIDM_FAGAN_BRADLEY, 0);

  // Test that different modes produce different planetary longitudes
  const pLahiri = calculate_planets(jd, Constants.SE_SIDM_LAHIRI);
  const pRaman = calculate_planets(jd, Constants.SE_SIDM_RAMAN);

  const sunLahiri = pLahiri.find((p: any) => p.name === "Sun").longitude;
  const sunRaman = pRaman.find((p: any) => p.name === "Sun").longitude;

  assertNotEquals(sunLahiri, sunRaman);
});

Deno.test("Deep Audit: Whole Sign Houses", () => {
  // Test Equal House vs Whole Sign
  const hEqual = calculate_houses(
    jd,
    loc.latitude,
    loc.longitude,
    "P",
    Constants.SE_SIDM_LAHIRI,
  );
  const hWhole = calculate_houses(
    jd,
    loc.latitude,
    loc.longitude,
    "W",
    Constants.SE_SIDM_LAHIRI,
  );

  const asc = hEqual.ascendant;
  const signStart = Math.floor(asc / 30) * 30;

  assertEquals(hEqual.cusps[0], asc);
  assertEquals(hWhole.cusps[0], signStart);
  assertEquals(hWhole.cusps[1], (signStart + 30) % 360);
});

Deno.test("Deep Audit: Planet Status (Retrograde/Combust)", () => {
  const planets = calculate_planets(jd, Constants.SE_SIDM_LAHIRI);

  planets.forEach((p: any) => {
    console.log(
      `${p.name}: Lon=${
        p.longitude.toFixed(2)
      }, Retro=${p.is_retrograde}, Combust=${p.is_combust}`,
    );
  });

  const sun = planets.find((p: any) => p.name === "Sun");
  const moon = planets.find((p: any) => p.name === "Moon");

  // Sun is never combust or retrograde
  assertEquals(sun.is_retrograde, false);
  assertEquals(sun.is_combust, false);

  // Rahu/Ketu should have same retrograde status
  const rahu = planets.find((p: any) => p.name === "Rahu");
  const ketu = planets.find((p: any) => p.name === "Ketu");
  assertEquals(rahu.is_retrograde, ketu.is_retrograde);
  assertEquals(
    Math.abs((rahu.longitude + 180) % 360 - ketu.longitude) < 0.001,
    true,
  );
});

Deno.test("Deep Audit: Daily Panchang Planetary Snapshot", () => {
  const p = calculate_daily_panchang(
    2026,
    1,
    18,
    loc,
    Constants.SE_SIDM_LAHIRI,
  );

  assertNotEquals((p as any).planets, undefined);
  assertEquals(Array.isArray((p as any).planets), true);
  assertEquals((p as any).planets.length, 9); // 7 planets + Rahu + Ketu

  const sun = (p as any).planets.find((pl: any) => pl.name === "Sun");
  assertEquals(sun !== undefined, true);
  console.log("Sun at Sunrise:", sun.longitude);
});
