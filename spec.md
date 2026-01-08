# Panchangam Design Specification

High-performance, astronomical-grade Vedic Calendar library (`panchangam`). This
specification details the astronomical accuracy, Vedic algorithms, and data
modeling.

## 1. Core Principles

The library transitions from simple "Table Lookup" methods to "Real-Time Vector
Calculation" (Drik Ganita) to ensure precision across all geolocations.

### 1.1. Ephemeris Kernel (Rust/Wasm)

- **Standard**: **Drik Ganita** (Astronomical Precision).
- **Planetary Data**: Uses **Swiss Ephemeris** (via generic C bindings) for
  NASA-grade planetary positions (Geocentric Longitude & Latitude).
- **Ayanamsha**: Supports a **Dynamic Ayanamsha Factory** to convert Tropical
  (Sayana) positions to Sidereal (Nirayana).
  - **Supported Modes**: Lahiri (Chitrapaksha), Raman, Krishnamurti, True
    Chitrapaksha.
  - **Formula**: $L_{nirayana} = L_{sayana} - Ayanamsha(t)$
- **Platform**: Compiled to **WebAssembly (Wasm)** for near-native speed in
  browsers, Node.js, and Edge environments.

### 1.2. Geo-Spatial Engine (Udaya Tithi)

- **Requirement**: Vedic days are defined by **Sunrise** (Udaya), not midnight.
- **Solution**: Iterative sunrise/sunset calculation accounting for
  **Atmospheric Refraction** and **Elevation**.
- **Distinction**: Distinguishes between:
  - **Technical Event**: Exact astronomical end time (e.g., Tithi ends at
    14:03).
  - **Visual Event**: Observable phenomena (e.g., Moonrise).

## 2. Architecture & Modules

### Module A: VedicChronos (5-Limb Calculator)

Calculates the instantaneous state of the _Pancha Anga_.

**1. Tithi (Lunar Day)**

- **Definition**: Angular distance between Moon ($L_m$) and Sun ($L_s$) in
  $12^{\circ}$ increments.
- **Output**: Index (1-30), Name, Paksha, and Percentage Remaining.

**2. Nakshatra (Lunar Mansion)**

- **Definition**: Moon's longitude divided into 27 segments of $13^{\circ}20'$.
- **Precision**: Calculates exact boundary crossing times (e.g., Mrigashirsha to
  Ardra).

**3. Yoga (Luni-Solar Sum)**

- **Definition**: Sum of longitudes ($L_m + L_s$) divided into 27 segments.

**4. Karana (Half-Tithi)**

- **Definition**: Half of a Tithi ($6^{\circ}$ increments).

**5. Vara (Solar Weekday)**

- **Logic**: Day begins at **Sunrise**. If `CurrentTime < Sunrise`, it counts as
  the previous weekday.

### Module B: MuhuratMatrix (Time Quality)

Segments the day into auspicious/inauspicious windows based on the 8-part
division of the "Dinamaan" (Day Duration).

**1. 8-Part Algorithm**

- **Calculation**: `(Sunset - Sunrise) / 8`
- **Segments**: Rahu Kalam (Inauspicious), Yamaganda, Gulika.
- **Logic**: Variable start times based on Weekday.

## 3. Implementation Plan

### Phase 1: Rust Core (Astronomy)

- Wrap `libswe` (Swiss Ephemeris).
- Implement `Ayanamsha` trait.
- Validate against NASA JPL Horizons.

### Phase 2: Vedic Math (Panchang)

- Implement Tithi, Nakshatra, Yoga calculations.
- **Root Finding**: Use iterative solvers (Brent's method) to find exact end
  times for Tithi/Nakshatra.

### Phase 3: Geo-Spatial

- Precise `Sunrise` algorithm.
- `DailyPanchang` struct anchored to local Sunrise.
- **Planetary War**: Detection logic for planets within $< 1^{\circ}$.

### Phase 4: Wasm/TS Bindings

- Expose Rust structs via `wasm-bindgen`.
- Generate strict TypeScript definitions (`.d.ts`).

## 4. Data Specifications

The library outputs a comprehensive structure:

```typescript
interface DailyPanchang {
  // Calculated Times
  sunrise: number; // Unix ms
  sunset: number;

  // The 5 Angas
  tithi: {
    index: number;
    name: string;
    endTime: number | null;
  };
  nakshatra: {
    index: number;
    name: string;
    endTime: number | null;
  };
  yoga: {
    index: number;
    name: string;
    endTime: number | null;
  };
  vara: string;

  // Astronomy
  ayanamshaValue: number;

  // Muhurats
  muhurats: {
    rahuKalam: { start: number; end: number };
    yamaganda: { start: number; end: number };
    gulika: { start: number; end: number };
  };
}
```

## 5. Verification

Accuracy is verified against:

1. **DrikPanchang.com**: For Tithi end times.
2. **Swiss Ephemeris**: For raw planetary positions.
3. **Physical Observation**: Sunrise times adjusted for refraction.
