# Panchangam (Wasm)

Vedic Astrology and Calendar library backed by Swiss Ephemeris, compiled to
WebAssembly for edge/serverless environments.

## Features

- **Swiss Ephemeris v2.10.03**: Powered by the `swisseph-wasm` crate.
- **Wasm-First**: Built for Deno, Node.js, and Cloudflare Workers.
- **Vedic Calendar**:
  - Root-finding (binary search) for exact Tithi, Nakshatra, Yoga end times.
  - Sunrise/Sunset calculations via SPA (Solar Position Algorithm).
- **Astronomy**:
  - Sidereal/Tropical conversions.
  - High-precision planetary positions (vsop87/jpl).
  - Graha Yuddha (Planetary War) detection.
- **Muhurat**: Dynamic calculation of Rahu Kalam, Yamaganda, Gulika.

## 📦 Usage

### Installation

Currently set up as a local crate. Build it first:

```bash
deno task build
```

This produces `lib/panchangam.js` and `lib/panchangam.wasm`.

### Basic Example

```typescript
import {
  calculate_daily_panchang,
  Location,
  swe_calc_ut,
  swe_julday,
} from "./lib/panchangam.js";

// Location: Bangalore (12.97 N, 77.59 E)
const loc = new Location(12.9716, 77.5946, 920.0);

// Calculate for Jan 5, 2026, using Lahiri Ayanamsha (mode 1)
const result = calculate_daily_panchang(2026, 1, 5, loc, 1);

console.log("Tithi:", result.tithi_name);
// Output: "Dwitiya"

console.log(
  "Nakshatra Ends:",
  new Date(result.nakshatra_end_time).toISOString(),
);
// Output: "2026-01-05T07:54:53.000Z"
```

### Muhurats (Time Qualities)

```typescript
const m = result.muhurats;
console.log(`Rahu Kalam: ${new Date(m.rahu_kalam.start).toLocaleTimeString()}`);
```

### Planetary War

```typescript
import { check_graha_yuddha, swe_julday } from "./lib/panchangam.js";

const jd = swe_julday(2024, 2, 22, 12.0, 1);
const wars = check_graha_yuddha(jd, 1); // 1 = Lahiri

if (wars.length > 0) {
  console.log(`${wars[0].planet1_name} fights ${wars[0].planet2_name}!`);
  console.log(
    `Winner: ${
      wars[0].winner_id === wars[0].planet1_id
        ? wars[0].planet1_name
        : wars[0].planet2_name
    }`,
  );
}
```

## 🛠️ Build

Requirements:

- **Rust** (stable)
- **Deno**
- **Clang/LLVM** (for compiling Swiss Ephemeris C code)

```bash
deno task build
```

This command:

1. Compiles the Rust crate and links the `swisseph-wasm` dependency.
2. Generates the Wasm binary and JS bindings in `lib/`.

## 📂 Project Structure

- `src/lib.rs`: `panchangam` Wasm entry point.
- `src/vedic/`: Core Vedic logic.
- `examples/`: TypeScript verification scripts.

## License

MIT
