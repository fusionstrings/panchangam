import { check_graha_yuddha, p_julday } from "../lib/panchangam.js";

console.log("=".repeat(60));
console.log(`⚔️  Graha Yuddha (Planetary War) Demo`);
console.log("=".repeat(60));

// Test Case: February 22, 2024
// Venus and Mars were very close.
const year = 2024, month = 2, day = 22;
const jd = p_julday(year, month, day, 12.0, 1);

console.log(`Checking for date: ${year}-${month}-${day} (JD: ${jd})`);
const wars = check_graha_yuddha(jd, 1); // Lahiri

if (Array.isArray(wars) && wars.length > 0) {
  console.log(`\n⚠️  Planetary War Detected! (${wars.length} pairs)`);
  for (const w of wars) {
    console.log(`\n   ⚔️  ${w.planet1_name} vs ${w.planet2_name}`);
    console.log(`       Difference: ${w.longitude_diff.toFixed(4)}°`);
    console.log(`       ${w.planet1_name} Mag: ${w.planet1_mag.toFixed(2)}`);
    console.log(`       ${w.planet2_name} Mag: ${w.planet2_mag.toFixed(2)}`);

    const winnerName = w.winner_id === w.planet1_id
      ? w.planet1_name
      : w.planet2_name;
    console.log(`       🏆 Victor: ${winnerName} (Brighter)`);
  }
} else {
  console.log("\n✅ No Planetary War detected.");
}

// Second Test: A date with no war (today)
const jd2 = p_julday(2025, 1, 5, 12.0, 1);
console.log(`\nChecking for date: 2025-01-05`);
const wars2 = check_graha_yuddha(jd2, 1);
if (!wars2 || wars2.length === 0) {
  console.log("✅ No Planetary War detected (Expected).");
} else {
  console.log(
    `⚠️  Unexpected war detected: ${
      // deno-lint-ignore no-explicit-any
      wars2.map((w: any) => w.planet1_name + "-" + w.planet2_name).join(", ")}`,
  );
}
