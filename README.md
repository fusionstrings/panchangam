# Panchangam (Wasm)

[![JSR](https://jsr.io/badges/@fusionstrings/panchangam)](https://jsr.io/@fusionstrings/panchangam)
[![NPM](https://img.shields.io/npm/v/@fusionstrings/panchangam)](https://www.npmjs.com/package/@fusionstrings/panchangam)
[![Crates.io](https://img.shields.io/crates/v/panchangam)](https://crates.io/crates/panchangam)

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

This project is currently distributed as a source crate. Verify and build the
Wasm bindings locally.

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

### Quick Start: Daily Panchang

Calculate comprehensive Panchang data including precise end times.

```typescript
import { calculate_daily_panchang, Location } from "./lib/panchangam.js";

// 1. Define Location: New Delhi (28.61 N, 77.20 E, 225m)
const delhi = new Location(28.6139, 77.2090, 225.0);

// 2. Calculate for January 1, 2024
// params: (year, month, day, location, ayanamsha_mode)
// mode 1 = Lahiri (Standard)
const result = calculate_daily_panchang(2024, 1, 1, delhi, 1);

// 3. Output Results
console.log(`Sunrise: ${new Date(result.sunrise).toLocaleTimeString()}`);
console.log(`Tithi: ${result.tithi_name}`);
console.log(`  - Ends at: ${new Date(result.tithi_end_time).toLocaleString()}`);
console.log(`Nakshatra: ${result.nakshatra_name}`);
console.log(
  `  - Ends at: ${new Date(result.nakshatra_end_time).toLocaleString()}`,
);
```

### Advanced: Planetary Positions & Dignity

Get precise sidereal positions and dignity status for all planets.

```typescript
import { calculate_planets, swe_julday } from "./lib/panchangam.js";

// Julian Day for calculation
const jd = swe_julday(2024, 1, 1, 12.0, 1); // Noon UT

// Calculate Sidereal positions (Mode 1 = Lahiri)
const planets = calculate_planets(jd, 1);

planets.forEach((p) => {
  console.log(`${p.name}: ${p.longitude.toFixed(2)}°`);
  console.log(`  Dignity: ${p.dignity}`); // Exalted, Own Sign, Friend, etc.
  console.log(`  Speed: ${p.speed.toFixed(4)}/day`);
  if (p.is_retrograde) console.log("  [Retrograde]");
});
```

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

## 🛠️ Development

### Project Structure

- **`src/lib.rs`**: Wasm entry point.
- **`src/vedic/`**: Core algorithms (Tithi, Nakshatra, Dignity, Muhurat).
- **`src/astronomy/`**: Swiss Ephemeris wrappers and solvers.
- **`scripts/build_npm.ts`**: Build script.

### Testing

Run the verification suite:

```bash
deno test --allow-read --allow-env
```

## License

MIT
