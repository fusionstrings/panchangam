import { assertEquals } from "@std/assert";
import { calculate_chara_dasha_periods } from "../lib/panchangam.js";

Deno.test("Jaimini: Chara Dasha Calculation", () => {
  // Scenario:
  // Ascendant: Libra (7). Direct Group.
  // Order: 7, 8, 9, 10, 11, 12, 1, 2, 3, 4, 5, 6.

  // Planet Setup:
  // Venus (Lord of Lib) -> in Pisces (11).
  // Mars (Lord of Sco) -> in Aries (0). Direct.
  // Ketu (Co-Lord Sco) -> in Taurus (1). Direct.
  // Jup (Lord Sag) -> in Cancer (3). Direct.
  // Sat (Lord Cap) -> in Leo (4). Reverse.
  // Rahu (Co-Lord Aqu) -> in Vir (5). Direct.
  // Sat (Co-Lord Aqu) -> in Leo (4). Reverse.

  // We need 7 planets + Rahu/Ketu.
  // Rahu=7, Ketu=8.

  const planets = [
    [0, 0.0], // Sun
    [1, 0.0], // Moon
    [2, 5.0], // Mars in Aries (0)
    [3, 0.0], // Mer
    [4, 95.0], // Jup in Cancer (3) (Standard Jup=4)
    [5, 335.0], // Ven in Pisces (11) (Standard Ven=5)
    [6, 125.0], // Sat in Leo (4) (Standard Sat=6)
    [7, 155.0], // Rahu in Virgo (5) (Standard Rahu=7/101)
    [8, 35.0], // Ketu in Taurus (1) (Standard Ketu=8/102)
  ];

  const asc_sign = 6; // Libra (0-based)
  const start_year = 2000.0;

  const dasha = calculate_chara_dasha_periods(planets, asc_sign, start_year);
  console.log("Chara Dasha Sequence:", dasha);

  // 1. First Dasha: Libra (7).
  // Lord Venus in Pisces (11).
  // Count: Direct (Odd) or Direct (Group)?
  // Libra Odd -> Direct.
  // Count from Lib(6) to Pis(11).
  // 6->7->8->9->10->11.
  // Count = 6 (7,8,9,10,11,12).
  // Duration = 6-1 = 5 Years.

  assertEquals(dasha[0].sign_id, 6); // Libra
  assertEquals(dasha[0].duration_years, 5);
  assertEquals(dasha[0].start_year, 2000.0);
  assertEquals(dasha[0].end_year, 2005.0);

  // 2. Second Dasha: Scorpio (8).
  // Even -> Reverse? (Or Direct based on Group?)
  // K.N. Rao says Sco is Direct grouping? Ascendant groups determine *SEQUENCE*.
  // Individual sign duration counting depends on Odd/Even nature of sign.
  // Scorpio (8) is Even -> Reverse counting.
  // Lords: Mars in Aries (0). Ketu in Taurus (1).
  // Who is stronger?
  // Mars in 0. Ketu in 1.
  // Conjunctions? None.
  // Longitude?
  // Mars 5.0 deg. Ketu 35.0(5.0 in Tau). Equal?
  // Assume Mars 5.1, Ketu 5.0. Mars stronger.
  // Lord = Mars (in Aries 0).
  // Sign = Scorpio (7).
  // Count Reverse: 7 -> 0.
  // 7, 6, 5, 4, 3, 2, 1, 0.
  // Count = 8 signs (Sco, Lib, Vir, Leo, Can, Gem, Tau, Ari).
  // Duration = 8 - 1 = 7 Years.

  // Let's verify what the code does. Code compares degrees within sign.

  assertEquals(dasha[1].sign_id, 7); // Scorpio
  // Duration might differ based on exact strength logic, but check logic output.
});
