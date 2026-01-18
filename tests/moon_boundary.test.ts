import { assertAlmostEquals, assertEquals } from "@std/assert";
import { calculate_vimshottari } from "../lib/panchangam.js";

Deno.test("Dasha: Moon Longitude 360.0 Boundary", () => {
  // 360.0 should be treated as 0.0 (Ashwini start)
  const res = calculate_vimshottari(360.0, 0, 0);
  assertEquals(res.nakshatra_name, "Ashwini");
  assertEquals(res.mahadasha, "Ketu");
});

Deno.test("Dasha: Negative Longitude Handling", () => {
  // -10 degrees -> 350 degrees (Revati)
  const res = calculate_vimshottari(-10.0, 0, 0);
  assertEquals(res.nakshatra_name, "Revati");
  assertEquals(res.mahadasha, "Mercury");
});
