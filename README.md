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
