# Panchangam (Wasm)

[![JSR](https://jsr.io/badges/@fusionstrings/panchangam)](https://jsr.io/@fusionstrings/panchangam)
[![NPM](https://img.shields.io/npm/v/@fusionstrings/panchangam)](https://www.npmjs.com/package/@fusionstrings/panchangam)

> **High-Precision Vedic Astrology & Calendar Library**
>
> _Powered by Swiss Ephemeris | Compiled to WebAssembly | Built for the Edge_

## 🌟 Why Panchangam?

Most Vedic astrology libraries rely on static lookup tables or simplified
algorithms that trade accuracy for speed. **Panchangam** bridges the gap between
ancient Vedic requirements and modern astronomical precision.

1. **Astronomical Precision ("Drik Ganita")**: We wrap the industry-standard
   **Swiss Ephemeris** (used by NASA/JPL) to calculate planetary positions to
   millisecond precision. No approximations.
2. **The "Udaya Tithi" Standard**: In Vedic traditions, the day doesn't start at
   midnight; it starts at **Sunrise**. We calculate exact local sunrise times
   (taking atmospheric refraction and altitude into account) to determine the
   correct Tithi, Nakshatra, and Yoga for any location on Earth.
3. **Wasm-First Performance**: Written in **Rust** and compiled to
   **WebAssembly**, this library runs with near-native performance in
   **Node.js**, **Deno**, and **Cloudflare Workers**. It's designed for
   serverless scalability.

## ✨ Features

- **Swiss Ephemeris v2.10.03**: The gold standard for planetary calculations.
- **Vedic Calendar (Panchang)**:
  - **Tithi**: Lunar day (1-30) with precise **start and end times**.
  - **Nakshatra**: 27 Lunar mansions with **start and end times**.
  - **Yoga**: 27 Luni-solar combinations.
  - **Karana**: 11 Half-Tithis.
  - **Vara**: Weekday based on sunrise-to-sunrise logic.
- **Advanced Astronomy**:
  - **True Ayanamsa**: Support for **Lahiri (Chitrapaksha)**, Raman,
    Krishnamurti, True Chitrapaksha, and more.
  - **Planetary Dignity**: Automatic calculation of Exalted, Debilitated, Own
    Sign, Friend/Enemy status.
  - **Planetary War (Graha Yuddha)**: Detects when planets are dangerously close
    (< 1°).
  - **Extended Muhurat**: Real-time calculation of:
    - **Rahu Kalam, Yamaganda, Gulika** (8-part day division)
    - **Brahma Muhurta** (Pre-dawn spiritual window)
    - **Abhijit Muhurta** (Mid-day victory period)

## 🚀 Usage

### Installation

Panchangam is available on [JSR](https://jsr.io/@fusionstrings/panchangam) and
[NPM](https://www.npmjs.com/package/@fusionstrings/panchangam).

**Deno:**

```bash
deno add jsr:@fusionstrings/panchangam
```

**Node.js / Bun:**

```bash
npm install @fusionstrings/panchangam
```

### Quick Start: Daily Panchang

Calculate comprehensive Panchang data including precise end times.

```typescript
import { calculate_daily_panchang, Location } from "@fusionstrings/panchangam";

// 1. Define Location: New Delhi (28.61 N, 77.20 E, 225m)
const delhi = new Location(28.6139, 77.2090, 225.0);

// 2. Calculate for January 1, 2024
// params: (year, month, day, location, ayanamsha_mode)
// mode 1 = Lahiri (Standard)
const result = calculate_daily_panchang(2024, 1, 1, delhi, 1);

// 3. Output Results
console.log(`Sunrise: ${new Date(result.sunrise).toLocaleTimeString()}`);
console.log(`Tithi: ${result.tithi_name}`);
console.log(
  `  - Ends at: ${
    result.tithi_end_time
      ? new Date(result.tithi_end_time).toLocaleString()
      : "N/A"
  }`,
);
console.log(`Nakshatra: ${result.nakshatra_name}`);
console.log(
  `  - Ends at: ${
    result.nakshatra_end_time
      ? new Date(result.nakshatra_end_time).toLocaleString()
      : "N/A"
  }`,
);
```

### Advanced: Planetary Positions & Dignity

Get precise sidereal positions and dignity status for all planets.

```typescript
import { calculate_planets, p_julday } from "@fusionstrings/panchangam";

// Julian Day for calculation
const jd = p_julday(2024, 1, 1, 12.0, 1); // Noon UT

// Calculate Sidereal positions (Mode 1 = Lahiri)
const planets = calculate_planets(jd, 1);

planets.forEach((p) => {
  console.log(`${p.name}: ${p.longitude.toFixed(2)}°`);
  console.log(`  Dignity: ${p.dignity}`); // Exalted, Own Sign, Friend, etc.
  console.log(`  Speed: ${p.speed.toFixed(4)}/day`);
  if (p.is_retrograde) console.log("  [Retrograde]");
});
```

#### Note on ayanamsa application

If you're comparing `calculate_planets(jd, mode)` output to KP / Vedic stacks
like VedicRishi (`astrologyapi.com`), it's helpful to know how the ayanamsa
ends up in the returned longitudes:

**Ayanamsa appears to be applied as naive subtraction** — i.e. the returned
sidereal longitudes match `swe_calc(jd, planet, TROPICAL) − swe_get_ayanamsa(jd)`
in our testing — rather than via Swiss-Ephemeris's `SEFLG_SIDEREAL` flag,
which rotates ecliptic coordinates in the ayanamsa reference frame. The two
agree to first order but differ by ~3–6″ per planet. Naive subtraction matches
the convention most KP / legacy Vedic software uses, so it is often the
desired behaviour.

If you specifically want the `SEFLG_SIDEREAL` path, you can call
`calc_ut(jd, planet, SEFLG_SWIEPH | SEFLG_SPEED | SEFLG_SIDEREAL)` directly,
but the sidereal mode must be set first. In our testing, calling
`calculate_planets(jd, mode)` once appears to set the global sidereal mode as
a side effect, so a subsequent `calc_ut(..., SEFLG_SIDEREAL)` returns
positions in the expected ayanamsa frame:

```typescript
import { p_julday, calculate_planets, calc_ut, Constants } from "@fusionstrings/panchangam";

const jd = p_julday(2024, 1, 1, 12.0, 1);
calculate_planets(jd, 1); // prime sidereal mode (mode 1 = Lahiri)

const flags = Constants.SEFLG_SWIEPH | Constants.SEFLG_SPEED | Constants.SEFLG_SIDEREAL;
const sun = calc_ut(jd, Constants.SE_SUN, flags); // sidereal Lahiri Sun
```

These are observations from external testing rather than statements about the
library's internals; the maintainer is the authoritative source.

### Muhurat Calculation

Determine auspicious and inauspicious time windows.

```typescript
// Accessed via the daily panchang result
const muhurats = result.muhurats;

console.log("--- Inauspicious Periods ---");
console.log(
  `Rahu Kalam: ${new Date(muhurats.rahu_kalam.start).toLocaleTimeString()} - ${
    new Date(muhurats.rahu_kalam.end).toLocaleTimeString()
  }`,
);
console.log(
  `Yamaganda: ${new Date(muhurats.yamaganda.start).toLocaleTimeString()} - ${
    new Date(muhurats.yamaganda.end).toLocaleTimeString()
  }`,
);

console.log("--- Auspicious Periods ---");
console.log(
  `Brahma Muhurta: ${
    new Date(muhurats.brahma_muhurta.start).toLocaleTimeString()
  }`,
);
console.log(
  `Abhijit Muhurta: ${
    new Date(muhurats.abhijit_muhurta.start).toLocaleTimeString()
  }`,
);
```

### Planetary War (Graha Yuddha)

Detect planetary wars where planets are within 1° of each other.

```typescript
import { check_graha_yuddha, p_julday } from "@fusionstrings/panchangam";

const jd = p_julday(2024, 1, 1, 12.0, 1);
const wars = check_graha_yuddha(jd, 1) as any[]; // Mode 1 = Lahiri

if (wars.length > 0) {
  console.log("Planetary War Detected!");
  wars.forEach((war) => {
    console.log(
      `${war.planet1_name} vs ${war.planet2_name} (Diff: ${
        war.longitude_diff.toFixed(
          4,
        )
      }°)`,
    );
    console.log(`Winner: Planet ID ${war.winner_id} (Brighter)`);
  });
} else {
  console.log("No planetary wars currently.");
}
```

### Vimshottari Dasha

Calculate the current ruling planetary periods.

```typescript
import { calculate_vimshottari } from "@fusionstrings/panchangam";

// Birth details
const birth_moon_long = 45.5; // Example longitude
const birth_time_ms = new Date("1990-01-01").getTime();
const current_time_ms = Date.now();

const dasha = calculate_vimshottari(
  birth_moon_long,
  birth_time_ms,
  current_time_ms,
);

console.log(`Current Mahadasha: ${dasha.mahadasha}`);
console.log(`Current Antardasha: ${dasha.antardasha}`);
console.log(`Current Pratyantardasha: ${dasha.pratyantardasha}`);
console.log(
  `Ends: ${new Date(dasha.pratyantardasha_end_date).toLocaleDateString()}`,
);
```

### House Calculation

Calculate Ascendant and House Cusps for various systems (Placidus, Whole Sign,
etc.).

```typescript
import {
  calculate_houses,
  Location,
  p_julday,
} from "@fusionstrings/panchangam";

const jd = p_julday(2024, 1, 1, 12.0, 1);
const loc = new Location(28.6139, 77.2090, 0.0);

// 'P' = Placidus, 'W' = Whole Sign, 'E' = Equal
// Mode 1 = Lahiri Ayanamsha (Sidereal)
const houses = calculate_houses(jd, loc.latitude, loc.longitude, "P", 1);

console.log(`Ascendant: ${houses.ascendant.toFixed(2)}°`);
houses.cusps.forEach((cusp, i) => {
  console.log(`House ${i + 1}: ${cusp.toFixed(2)}°`);
});
```

## 🛠️ Development

### Project Structure

- **`src/lib.rs`**: Wasm entry point.
- **`src/vedic/`**: Core algorithms (Tithi, Nakshatra, Dignity, Muhurat).
- **`src/astronomy/`**: Swiss Ephemeris wrappers and solvers.
- **`scripts/build_npm.ts`**: Build script.

### Development / Building from Source

If you want to contribute or build the latest version from source:

**Prerequisites:**

- [Rust](https://www.rust-lang.org/) (stable)
- [Deno](https://deno.land/) (v1.37+)

**One-Step Build:**

```bash
deno task build
```

This generates:

- `./lib/panchangam.js`: The ESM entry point.
- `./lib/panchangam.wasm`: The compiled Wasm binary.
- `./lib/panchangam.d.ts`: Fully typed TypeScript definitions.

### Runnable Examples

You can run the full examples provided in the `examples/` directory:

```bash
# Basic Features (Daily Panchang, Planets, etc.)
deno run -A examples/demo.ts

# Advanced Features (Vargas, Shadbala, Jaimini)
deno run -A examples/demo_advanced.ts

# Ashtakavarga & Special Points
deno run -A examples/demo_ashtakavarga.ts
```

### Testing

Run the verification suite:

```bash
deno task test
```

## License

MIT
