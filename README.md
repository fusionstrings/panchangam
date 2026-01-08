# Panchangam (Wasm)

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
  - **Tithi**: Lunar day (1-30) with end times calculated via iterative binary
    search.
  - **Nakshatra**: 27 Lunar mansions.
  - **Yoga**: 27 Luni-solar combinations.
  - **Karana**: 11 Half-Tithis.
  - **Vara**: Weekday based on sunrise-to-sunrise logic.
- **Advanced Astronomy**:
  - **True Ayanamsa**: Support for **Lahiri (Chitrapaksha)**, Raman,
    Krishnamurti, and more.
  - **Planetary War (Graha Yuddha)**: Detects when planets are dangerously close
    (< 1°).
  - **Muhurat**: Real-time calculation of Rahu Kalam, Yamaganda, and Gulika
    (8-part day division).

## 🚀 Usage

### Installation

This project is currently distributed as a source crate. You verify and build
the Wasm bindings locally.

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

### Quick Start

```typescript
import { calculate_daily_panchang, Location } from "./lib/panchangam.js";

// 1. Define Location: Bangalore (12.97 N, 77.59 E, 920m altitude)
const bangalore = new Location(12.9716, 77.5946, 920.0);

// 2. Calculate for January 5, 2026
// params: (year, month, day, location, ayanamsha_mode)
// mode 1 = Lahiri (Chitrapaksha)
const result = calculate_daily_panchang(2026, 1, 5, bangalore, 1);

// 3. Output Results
console.log(`Date: ${result.date}`);
console.log(`Sunrise: ${new Date(result.sunrise).toLocaleTimeString()}`);
console.log(
  `Tithi: ${result.tithi_name} (Ends at ${
    new Date(result.tithi_end_time).toLocaleTimeString()
  })`,
);
console.log(`Nakshatra: ${result.nakshatra_name}`);
```

### Advanced: Planetary War

```typescript
import { check_graha_yuddha, swe_julday } from "./lib/panchangam.js";

const jd = swe_julday(2026, 1, 6, 12.0, 1); // Jan 6, 2026
const conflicts = check_graha_yuddha(jd, 1);

if (conflicts.length > 0) {
  conflicts.forEach((war) => {
    console.warn(
      `⚔️ PLANETARY WAR: ${war.planet1_name} vs ${war.planet2_name}`,
    );
    console.log(
      `Winner: ${
        war.winner_id === war.planet1_id ? war.planet1_name : war.planet2_name
      }`,
    );
  });
}
```

## 🛠️ Development

### Project Structure

- **`src/lib.rs`**: Wasm entry point (exposes functions to JS).
- **`src/vedic/`**: Core Vedic algorithms (Tithi, Nakshatra, etc.).
- **`src/astronomy/`**: Swiss Ephemeris wrappers and solvers.
- **`scripts/build_npm.ts`**: Build script to generate NPM/Deno package.

### Testing

Run the Deno-based verification suite:

```bash
deno test --allow-read --allow-env
```

## License

MIT
