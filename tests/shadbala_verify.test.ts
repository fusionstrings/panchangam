import { assertAlmostEquals, assertEquals } from "@std/assert";
import {
  calculate_jaimini_karakas,
  calculate_planet_strength,
  KarakaName,
} from "../lib/panchangam.js";

Deno.test("Shadbala: Basic Calculation Check", () => {
  // Test Stub output
  // Sun (0) at 10 deg Aries (Exalted)
  // Sthana: Uchcha(1.0/60) + Saptavargaja(30/60) = 0.516?
  // Dig: Sun at Ascendant (0deg) -> 4th from powerpoint(10th house).
  // Sun power point = 10th. (Asc + 270) = 270.
  // Long 10.
  // Arc = (10 - 270) = -260 -> 100 deg.
  // Bala = (180 - 100)/3 = 80/3 = 26.666

  // Result should contain consistent values
  const res = calculate_planet_strength(10.0, 0, 2450000.5, 0.0);
  console.log("Shadbala Sun:", res);

  // Check fields exist
  assertAlmostEquals(res.naisargika_bala, 60.0); // 60 Shashtiamsas (1 Rupa)
});

Deno.test("Jaimini: Karaka Calculation (7 Karakas)", () => {
  // 7 Planets longitudes
  // Sun: 29 deg
  // Moon: 28 deg
  // Mars: 10 deg
  // Mer: 15 deg
  // Jup: 5 deg
  // Ven: 2 deg
  // Sat: 1 deg

  // Expected order: Sun(AK), Moon(AmK), Mer(BK), Mar(MK), Jup(PK), Ven(GK), Sat(DK)

  const planets = [
    [0, 29.0], // Sun
    [1, 28.0], // Moon
    [2, 10.0], // Mars
    [3, 15.0], // Mer
    [4, 5.0], // Jup
    [5, 2.0], // Ven
    [6, 1.0], // Sat
  ];

  const karakas = calculate_jaimini_karakas(planets, false);
  console.log("Karakas:", karakas);

  // Verify AK (Atmakaraka) is Sun(0)
  const ak = karakas.find((k) => k.karaka_name === KarakaName.AtmaKaraka);
  if (!ak) throw new Error("Atmakaraka not found");
  assertEquals(ak.planet_id, 0);

  // Verify DK (Darakaraka) is Saturn(6)
  const dk = karakas.find((k) => k.karaka_name === KarakaName.DaraKaraka);
  if (!dk) throw new Error("Darakaraka not found");
  assertEquals(dk.planet_id, 6);
});

Deno.test("Jaimini: 8 Karakas (Include Rahu)", () => {
  // Sun: 20
  // Rahu: 25 -> Should be higher than Sun
  const planets = [
    [0, 20.0],
    [1, 10.0],
    [2, 5.0],
    [3, 4.0],
    [4, 3.0],
    [5, 2.0],
    [6, 1.0],
    [101, 25.0], // Rahu id 101? Using input directly.
  ];

  const karakas = calculate_jaimini_karakas(planets, true);
  console.log("8 Karakas:", karakas);

  const ak = karakas.find((k) => k.karaka_name === KarakaName.AtmaKaraka);
  if (!ak) throw new Error("Atmakaraka not found");
  assertEquals(ak.planet_id, 101); // Rahu is AK
});
