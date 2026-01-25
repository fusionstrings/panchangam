import { assertEquals } from "@std/assert";
import { calculate_reduced_ashtakavarga } from "../lib/panchangam.js";

Deno.test("Ashtakavarga: Reductions (Trikona & Ekadhipatya)", () => {
  // Synthetic Bindus for 12 Signs (Ari..Pis)
  // Trikona Logic Check:
  // Fire (0,4,8): 5, 4, 6. Min=4. -> Exp: 1, 0, 2.
  // Earth (1,5,9): 0, 5, 5. One zero -> Exp: 0, 5, 5. (No change)
  // Air (2,6,10): 0, 0, 5. Two zeros -> Exp: 0, 0, 0. (Eliminate third)
  // Water (3,7,11): 5, 8, 8. Min=5. -> Exp: 0, 3, 3.

  const bindus = new Int32Array([
    5,
    0,
    0,
    5,
    4,
    5,
    0,
    8,
    6,
    5,
    5,
    8,
  ]);

  // Planet Setup for Ekadhipatya Occupancy
  // Need to match signs 0..11.
  // Ari(0): Unoccupied (or assume empty)
  // Sco(7): Occupied by something (PID 2 Mars?)
  // This allows testing Ekadhipatya on Ari/Sco pair.

  // Pair 1: Mars (Ari 0, Sco 7).
  // Trikona Result: Ari(1), Sco(3).
  // Occupancy: P[0] empty, P[7] occupied.
  // Rule: "One occupied, remove unoccupied." -> Ari=0. Sco=3.

  // Pair 2: Venus (Tau 1, Lib 6).
  // Trikona Result: Tau(0), Lib(0). Both 0 -> 0.

  // Pair 3: Mercury (Gem 2, Vir 5).
  // Trikona Result: Gem(0), Vir(5).
  // Occupancy: Let's make Gem occupied, Vir empty?
  // P[2] occupied. P[5] empty.
  // Rule: Remove unoccupied -> Vir=0. Gem=0. (Both 0).

  // Pair 4: Jupiter (Sag 8, Pis 11).
  // Trikona Result: Sag(2), Pis(3).
  // Occupancy: Both Empty.
  // Rule: Unequal? Replace greater with smaller -> Sag(2), Pis(2).

  // Pair 5: Saturn (Cap 9, Aqu 10).
  // Trikona Result: Cap(5), Aqu(0).
  // Occupancy: No matter. One is zero already.

  const planets = [
    [2, 215.0], // Mars in Sco (7). 210-240.
    [3, 75.0], // Mercury in Gem (2). 60-90.
  ];
  // Need to pass as tuples. JS array of arrays [id, long] works via serde tuples.

  const result = calculate_reduced_ashtakavarga(bindus, planets);
  const reduced = result.reduced_bindus;
  console.log("Reduced Bindus:", reduced);

  // Verify Trikona + Ekadhipatya Combined Results

  // Group 1: Fire (Ari, Leo, Sag) -> Trikona: [1, 0, 2].
  // Ari(0) Mars pair with Sco(7). Ari empty, Sco occupied. Reduce Ari to 0. -> Ari: 0.
  // Leo(4) - Single. Remains 0 (from Trikona).
  // Sag(8) Jup pair with Pis(11). Both Empty. Trikona: Sag(2), Pis(3). Reduce Pis to 2. -> Sag: 2.

  assertEquals(reduced[0], 0, "Aries reduced to 0 (Ekadhipatya)");
  assertEquals(reduced[4], 0, "Leo Trikona 0");
  assertEquals(reduced[8], 2, "Sag Trikona 2, Ekadhipatya 2 (Equalized)");

  // Group 2: Earth (Tau, Vir, Cap) -> Trikona: [0, 5, 5].
  // Tau(1) Ven pair Lib(6). Trikona Tau(0), Lib(0). -> Tau: 0.
  // Vir(5) Mer pair Gem(2). Trikona Vir(5), Gem(0). Gem Occupied, Vir Empty. Reduce Vir to 0. -> Vir: 0.
  // Cap(9) Sat pair Aqu(10). Trikona Cap(5), Aqu(0).
  // Both empty (no planets in 9/10). Reduce larger (5) to smaller (0) -> Cap: 0.

  assertEquals(reduced[1], 0, "Tau 0");
  assertEquals(reduced[5], 0, "Vir reduced to 0 (Mer pair)");
  assertEquals(reduced[9], 0, "Cap 0");

  // Group 3: Air (Gem, Lib, Aqu) -> Trikona: [0, 0, 0].
  assertEquals(reduced[2], 0, "Gem 0");
  assertEquals(reduced[6], 0, "Lib 0");
  assertEquals(reduced[10], 0, "Aqu 0");

  // Group 4: Water (Can, Sco, Pis) -> Trikona: [0, 3, 3].
  // Can(3) Single. -> 0.
  // Sco(7) Mars pair Ari(0). Sco Occupied. -> Sco: 3.
  // Pis(11) Jup pair Sag(8). Both Empty. Trikona Pis(3), Sag(2). Equalize to 2. -> Pis: 2.

  assertEquals(reduced[3], 0, "Can 0");
  assertEquals(reduced[7], 3, "Sco 3");
  assertEquals(reduced[11], 2, "Pis reduced to 2 (Equalized)");

  // Check Shodya Pinda
  console.log("Shodya Pinda:", result.shodaya_pinda);
});
