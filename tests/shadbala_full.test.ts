import { assertAlmostEquals } from "@std/assert";
import { calculate_full_shadbala } from "../lib/panchangam.js";

Deno.test("Shadbala: Full Profile & Drik Bala", () => {
  // Test Setup:
  // Ascendant: 0 deg (Aries)
  // Sun: 0 deg (Aries) -> Malefic
  // Moon: 180 deg (Libra) -> Benefic (Full Moon)
  // Mars: 90 deg (Cancer) -> Malefic
  // Mercury: 30 deg (Taurus) -> Benefic
  // Jupiter: 60 deg (Gemini) -> Benefic
  // Venus: 330 deg (Pisces) -> Benefic
  // Saturn: 270 deg (Capricorn) -> Malefic

  const planets = [
    { id: 0, longitude: 0.0, speed: 1.0 },
    { id: 1, longitude: 180.0, speed: 12.0 },
    { id: 2, longitude: 90.0, speed: -0.2 }, // Retrograde
    { id: 3, longitude: 30.0, speed: 1.5 },
    { id: 4, longitude: 60.0, speed: 0.1 },
    { id: 5, longitude: 330.0, speed: 1.2 },
    { id: 6, longitude: 270.0, speed: 0.05 },
  ];

  const jd = 2451545.0; // epoch J2000
  const asc = 0.0;

  const profile = calculate_full_shadbala(planets, jd, asc);
  console.log("Full Profile:", profile);

  // Verify Moon Drik Bala
  // Moon is at 180.
  // Aspects received:
  // 1. Sun (0) -> Angle 180. Value 60. Malefic -> -60.
  // 2. Mars (90) -> Angle 90. 4th Aspect. Value 60 (Special). Malefic -> -60.
  // 3. Mercury (30) -> Angle 150. Value (150-150)*2 = 0? Or (150-120)=30.
  //    Formula: 120-150 => 30 - (d-120).
  //    Angle 150 -> 30 - (30) = 0.
  // 4. Jupiter (60) -> Angle 120. 5th Aspect. Value 60 (Special). Benefic -> +60.
  // 5. Venus (330) -> Angle (180 - 330) mod 360 = 210.
  //    Angle from Venus to Moon: 180 - 330 = -150 -> 210.
  //    210 > 180. No standard aspect.
  //    Venus special? No. Value 0.
  // 6. Saturn (270) -> Angle (180 - 270) = -90 -> 270.
  //    Angle from Saturn to Moon: 270.
  //    Saturn 10th aspect (270). Value 60 (Special). Malefic -> -60.

  // Total Net: -60 (Sun) - 60 (Mars) + 0 (Mer) + 60 (Jup) + 0 (Ven) - 60 (Sat)
  // Net = -120.
  // Drik Bala = Net / 4 = -30.
  // Let's assert close to -30.

  assertAlmostEquals(profile.moon.drik_bala, -30.0, 1.0);

  // Verify Sun Drik Bala
  // Sun at 0.
  // Aspects from:
  // Moon (180) -> Angle 180. Full aspect (60). Benefic -> +60.
  // Mars (90) -> Angle 270. Mars 8th aspect? No. 8th is 210.
  //    Angle (0 - 90) = 270.
  //    Value 0.
  // Jupiter (60) -> Angle 300. Value 0.
  // Saturn (270) -> Angle (0 - 270) = 90.
  //    Saturn aspecting Sun?
  //    Angle from Sat to Sun: 90.
  //    Standard at 90: (90-60)+15 = 45.
  //    Saturn special? 3rd (60), 10th (270). No.
  //    Value 45. Malefic -> -45.

  // Net: +60 (Moon) - 45 (Sat) = +15.
  // Drik Bala = 15 / 4 = 3.75.

  assertAlmostEquals(profile.sun.drik_bala, 3.75, 1.0);

  // Verify Chesta Bala
  // Mars is Retrograde (speed -0.2) -> Should be 60.0
  assertAlmostEquals(profile.mars.chesta_bala, 60.0, 0.1);

  // Sun is Normal speed (1.0) -> Avg speed 0.98.
  // 1.0 < 1.2 * 0.98 (1.176). Not Sighra.
  // 1.0 > 0.5 * 0.98. Not Manda.
  // Should be 30.0.
  assertAlmostEquals(profile.sun.chesta_bala, 30.0, 0.1);
});
