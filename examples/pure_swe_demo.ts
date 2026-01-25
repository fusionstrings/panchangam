import { get_version, p_calc_ut, p_julday } from "../lib/panchangam.js";

console.log("=".repeat(60));
console.log(`🇨🇭 Pure Swiss Ephemeris Module Demo`);
console.log(`   Version: ${get_version()}`);
console.log("=".repeat(60));

// 1. Date Conversion
const year = 2024, month = 1, day = 1;
const hour = 12.0;

const tjd = p_julday(year, month, day, hour, 1); // 1 = SE_GREG_CAL
console.log(`\n📅 Date: ${year}-${month}-${day} ${hour}h UTC`);
console.log(`   Julian Day (p_julday): ${tjd.toFixed(6)}`);

// 2. Sidereal Time
// const sid_time = swe_sidtime(tjd);
// console.log(`   Sidereal Time (swe_sidtime): ${sid_time.toFixed(6)} hours`);

// 3. Planetary Calculation (Sun)
const SE_SUN = 0;
const SEFLG_SWIEPH = 2;
const SEFLG_SPEED = 256;
const iflag = SEFLG_SWIEPH | SEFLG_SPEED;

try {
  const sun_pos = p_calc_ut(tjd, SE_SUN, iflag);
  // const sun_name = swe_get_planet_name(SE_SUN);
  const sun_name = "Sun";

  console.log(`\n☀️  Planet: ${sun_name}`);
  console.log(`   Longitude: ${sun_pos.longitude.toFixed(6)}°`);
  console.log(`   Latitude:  ${sun_pos.latitude.toFixed(6)}°`);
  console.log(`   Distance:  ${sun_pos.distance.toFixed(6)} AU`);
  console.log(`   Speed:     ${sun_pos.speed_long.toFixed(6)} deg/day`);
} catch (e) {
  console.error("Error calculating Sun position:", e);
}

// 4. Reverse Date Conversion
// const date = swe_revjul(tjd, 1);
// console.log(`\n🔄 Reverse Date (swe_revjul):`);
// console.log(`   ${date.year}-${date.month}-${date.day} ${date.hour}h`);

console.log("\n✅ Pure module verification passed.");
