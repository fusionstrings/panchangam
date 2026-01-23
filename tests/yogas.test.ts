import { assertEquals } from "@std/assert";
import { find_active_yogas, YogaResult } from "../lib/panchangam.js";

Deno.test("Yoga Detection: Hamsa & Budhaditya", () => {
  // Scenario:
  // Ascendant: Cancer (90 deg)
  // Jupiter: Cancer (90 deg) -> In 1st House (Kendra) + Exalted -> Hamsa Yoga
  // Sun: Leo (120 deg) -> In 2nd House
  // Mercury: Leo (120 deg) -> In 2nd House -> Conjunction -> Budhaditya Yoga
  // Mars: Capricorn (270 deg) -> Exalted in 7th House (Kendra) -> Ruchaka Yoga

  const planets = [
    { id: 0, longitude: 125.0, speed: 1.0 }, // Sun in Leo
    { id: 1, longitude: 0.0, speed: 1.0 }, // Moon
    { id: 2, longitude: 275.0, speed: 1.0 }, // Mars in Capricorn (Exalted)
    { id: 3, longitude: 122.0, speed: 1.0 }, // Mer in Leo
    { id: 4, longitude: 95.0, speed: 0.1 }, // Jup in Cancer (Exalted)
    { id: 5, longitude: 0.0, speed: 1.0 }, // Ven
    { id: 6, longitude: 0.0, speed: 1.0 }, // Sat
  ];

  // Ascendant Cancer -> 90.0 ~ 120.0
  const asc = 100.0; // Cancer

  const yogas = find_active_yogas(planets, asc);
  console.log("Active Yogas:", yogas);

  // Check Hamsa Yoga
  const hamsa = yogas.find((y: any) => y.name === "Hamsa Yoga");
  assertEquals(
    !!hamsa,
    true,
    "Hamsa Yoga should be active (Jup Exalted in Kendra)",
  );

  // Check Ruchaka Yoga
  const ruchaka = yogas.find((y: any) => y.name === "Ruchaka Yoga");
  assertEquals(
    !!ruchaka,
    true,
    "Ruchaka Yoga should be active (Mars Exalted in Kendra)",
  );

  // Check Budhaditya Yoga
  const budha = yogas.find((y: any) => y.name === "Budhaditya Yoga");
  assertEquals(
    !!budha,
    true,
    "Budhaditya Yoga should be active (Sun+Mer conjunction)",
  );
});
