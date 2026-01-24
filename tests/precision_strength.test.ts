import { assertEquals } from "@std/assert";
import {
  calculate_full_shadbala,
  calculate_planets,
} from "../lib/panchangam.js";

Deno.test("Strength Precision: Moolatrikona Degrees", () => {
  // Sun in Leo (120-150 deg)
  // 0-20 deg Leo (120-140) is Moolatrikona
  // 20-30 deg Leo (140-150) is Own Sign

  // We'll compare Saptavargaja Bala (part of Sthana Bala)
  // Saptavargaja in D1: Moolatrikona = 45, Own = 30.

  const planets_mool = [
    { id: 0, longitude: 125.0, speed: 0.98 }, // Sun in Leo (Moolatrikona range)
    { id: 1, longitude: 0.0, speed: 1.0 },
    { id: 2, longitude: 0.0, speed: 1.0 },
    { id: 3, longitude: 0.0, speed: 1.0 },
    { id: 4, longitude: 0.0, speed: 1.0 },
    { id: 5, longitude: 0.0, speed: 1.0 },
    { id: 6, longitude: 0.0, speed: 1.0 },
  ];

  const planets_own = [
    { id: 0, longitude: 145.0, speed: 0.98 }, // Sun in Leo (Own Sign range)
    { id: 1, longitude: 0.0, speed: 1.0 },
    { id: 2, longitude: 0.0, speed: 1.0 },
    { id: 3, longitude: 0.0, speed: 1.0 },
    { id: 4, longitude: 0.0, speed: 1.0 },
    { id: 5, longitude: 0.0, speed: 1.0 },
    { id: 6, longitude: 0.0, speed: 1.0 },
  ];

  const shad_mool = calculate_full_shadbala(planets_mool, 2451545.0, 0.0);
  const shad_own = calculate_full_shadbala(planets_own, 2451545.0, 0.0);

  console.log("Sun Sthana (Moolatrikona 125 deg):", shad_mool.sun.sthana_bala);
  console.log("Sun Sthana (Own Sign 145 deg):", shad_own.sun.sthana_bala);

  // Difference should be approx 15 shashtiamsas (45 - 30) for D1.
  // However, Saptavargaja sums all 7 Vargas.
  // We expect shad_mool.sun.sthana_bala > shad_own.sun.sthana_bala
  const diff = shad_mool.sun.sthana_bala - shad_own.sun.sthana_bala;
  assertEquals(
    diff > 5.0,
    true,
    "Moolatrikona degree should give higher strength than Own Sign degree",
  );
});

Deno.test("Strength Precision: Five-Fold Friendship", () => {
  // Sun and Saturn relative positions
  // Saturn Permanent Enemy of Sun.
  // If Saturn in 11th from Sun (Temporary Friend) -> Combined: Neutral.
  // If Saturn in 1st from Sun (Temporary Enemy) -> Combined: Great Enemy.

  const planets_neutral = [
    { id: 0, longitude: 0.0, speed: 1.0 }, // Sun in Aries
    { id: 6, longitude: 300.0, speed: 1.0 }, // Sat in Aquarius (11th from Sun)
    { id: 1, longitude: 0.0, speed: 1.0 },
    { id: 2, longitude: 0.0, speed: 1.0 },
    { id: 3, longitude: 0.0, speed: 1.0 },
    { id: 4, longitude: 0.0, speed: 1.0 },
    { id: 5, longitude: 0.0, speed: 1.0 },
  ];

  const planets_great_enemy = [
    { id: 0, longitude: 0.0, speed: 1.0 }, // Sun in Aries
    { id: 6, longitude: 5.0, speed: 1.0 }, // Sat in Aries (1st house - Enemy)
    { id: 1, longitude: 0.0, speed: 1.0 },
    { id: 2, longitude: 0.0, speed: 1.0 },
    { id: 3, longitude: 0.0, speed: 1.0 },
    { id: 4, longitude: 0.0, speed: 1.0 },
    { id: 5, longitude: 0.0, speed: 1.0 },
  ];

  const shad_neutral = calculate_full_shadbala(planets_neutral, 2451545.0, 0.0);
  const shad_worst = calculate_full_shadbala(
    planets_great_enemy,
    2451545.0,
    0.0,
  );

  console.log(
    "Saturn Sthana (Neutral context):",
    shad_neutral.saturn.sthana_bala,
  );
  console.log(
    "Saturn Sthana (Great Enemy context):",
    shad_worst.saturn.sthana_bala,
  );

  // In D1, Saturn in Aries is Debilitated anyway, but saptavargaja checks sign lord.
  // Lord of Aries is Mars.
  // In both cases, Sat is in Aries (Lord Mars).
  // Relationship Sat-Mars:
  // Perm: Mars is Enemy of Sat.
  // Temp:
  // Case 1: Sat(Aries), Sun(Aries), others(Aries).
  // Case 2: Sat(Aries), Sun(Aquarius).

  // Wait, the relationship is between the PLANET and the sign LORD.
  // If Sat in Aries, Lord is Mars.
  // Relationship is Sat-Mars.
  // To test 5-fold, we need to move MARS.

  const planets_friend = [
    { id: 6, longitude: 10.0, speed: 0.03 }, // Sat in Aries
    { id: 2, longitude: 310.0, speed: 0.52 }, // Mars in Aquarius (11th house from Sat -> Friend)
    { id: 0, longitude: 0.0, speed: 1.0 },
    { id: 1, longitude: 0.0, speed: 1.0 },
    { id: 3, longitude: 0.0, speed: 1.0 },
    { id: 4, longitude: 0.0, speed: 1.0 },
    { id: 5, longitude: 0.0, speed: 1.0 },
  ];

  const planets_enemy = [
    { id: 6, longitude: 10.0, speed: 0.03 }, // Sat in Aries
    { id: 2, longitude: 15.0, speed: 0.52 }, // Mars in Aries (1st house from Sat -> Enemy)
    { id: 0, longitude: 0.0, speed: 1.0 },
    { id: 1, longitude: 0.0, speed: 1.0 },
    { id: 3, longitude: 0.0, speed: 1.0 },
    { id: 4, longitude: 0.0, speed: 1.0 },
    { id: 5, longitude: 0.0, speed: 1.0 },
  ];

  const shad_f = calculate_full_shadbala(planets_friend, 2451545.0, 0.0);
  const shad_e = calculate_full_shadbala(planets_enemy, 2451545.0, 0.0);

  console.log("Saturn Sthana (Neutral):", shad_f.saturn.sthana_bala);
  console.log("Saturn Sthana (Great Enemy):", shad_e.saturn.sthana_bala);

  assertEquals(
    shad_f.saturn.sthana_bala > shad_e.saturn.sthana_bala,
    true,
    "Temporary friendship should increase Shadbala",
  );
});
