import { assertEquals } from "@std/assert";
import {
  calculate_vimshottari,
  calculate_yogini,
  // YoginiInfo,
} from "../lib/panchangam.js";

Deno.test("Dasha: Yogini Dasha Calculation", () => {
  // Test Case:
  // Moon Longitude: 0 deg (Aries - Ashwini Nakshatra)
  // Birth Time: 0
  // Current Time: 0

  // Ashwini (0):
  // Yogini Start = (Nak Index 0 + 3) % 8 = 3.
  // Index 3 -> 0=Mangala, 1=Pingala, 2=Dhanya, 3=Bhramari (Mars).
  // So starting Dasha should be Bhramari.

  const start_yogini = calculate_yogini(0.0, 0, 0);
  console.log("Start Yogini (Ashwini):", start_yogini);

  assertEquals(start_yogini.mahadasha, "Bhramari");

  // Check Elapsed Time logic
  // Add 5 years (Bhramari duration is 4 years).
  // Should move to next: Bhadrika (Mercury).
  const ms_5_years = 5 * 365.25 * 24 * 3600 * 1000;
  const next_yogini = calculate_yogini(0.0, 0, ms_5_years);
  console.log("Yogini after 5 years:", next_yogini);

  assertEquals(next_yogini.mahadasha, "Bhadrika");
});

Deno.test("Dasha: Vimshottari Basic verify", () => {
  // Ashwini (0-13.33 deg). Lord Ketu.
  // 0 deg -> Start of Ashwini. Full Ketu balance.
  const vim = calculate_vimshottari(0.0, 0, 0);
  assertEquals(vim.mahadasha, "Ketu");
});
