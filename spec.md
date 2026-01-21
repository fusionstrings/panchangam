# Panchangam Design Specification

High-performance, astronomical-grade Vedic Calendar library (`panchangam`). This
specification details the astronomical accuracy, Vedic algorithms, and data
modeling.

## 1. Core Principles

The library uses "Real-Time Vector Calculation" (Drik Ganita) to ensure
precision across all geolocations, rejecting static lookup tables in favor of
high-precision orbital physics.

### 1.1. Ephemeris Kernel (Rust/Wasm)

- **Standard**: **Drik Ganita** (Astronomical Precision).
- **Planetary Data**: Uses **Swiss Ephemeris** (v2.10) for NASA-grade planetary
  positions (Geocentric Longitude & Latitude).
- **Ayanamsha**: Supports a **Dynamic Ayanamsha Factory** to convert Tropical
  (Sayana) positions to Sidereal (Nirayana).
  - **Supported Modes**: Lahiri (Chitrapaksha), Raman, Krishnamurti, True
    Chitrapaksha, Fagan-Bradley.
  - **Formula**: $L_{nirayana} = L_{sayana} - Ayanamsha(t)$
- **Platform**: Compiled to **WebAssembly (Wasm)** for near-native speed in
  browsers, Node.js, and Edge environments.

### 1.2. Geo-Spatial Engine (Udaya Tithi)

- **Requirement**: Vedic days are defined by **Sunrise** (Udaya), not midnight.
- **Solution**: Iterative sunrise/sunset calculation accounting for
  **Atmospheric Refraction** and **Elevation**.
- **Timings**: Calculates exact start and end times for all Angas (Tithi,
  Nakshatra, Yoga, Karana) using binary search crossing detection against the
  ephemeris.

## 2. Architecture & Modules

### Module A: VedicChronos (5-Limb Calculator)

Calculates the instantaneous state of the _Pancha Anga_.

**1. Tithi (Lunar Day)**

- **Definition**: Angular distance between Moon ($L_m$) and Sun ($L_s$) in
  $12^{\circ}$ increments.
- **Precision**: 1-second accuracy for start/end times via iterative solver.

**2. Nakshatra (Lunar Mansion)**

- **Definition**: Moon's longitude divided into 27 segments of $13^{\circ}20'$.
- **Values**: Index, Name, Ruler, Quality.

**3. Yoga (Luni-Solar Sum)**

- **Definition**: Sum of longitudes ($L_m + L_s$) divided into 27 segments.

**4. Karana (Half-Tithi)**

- **Definition**: Half of a Tithi ($6^{\circ}$ increments).

**5. Vara (Solar Weekday)**

- **Logic**: Day begins at **Sunrise**.

### Module B: MuhuratMatrix (Time Quality)

Segments the day into auspicious/inauspicious windows using the breakdown of
"Dinamaan" (Day Duration).

**1. 8-Part Algorithm**

- **Calculation**: `(Sunset - Sunrise) / 8`
- **Segments**: Rahu Kalam, Yamaganda, Gulika.

**2. Special Muhurats**

- **Brahma Muhurta**: 96 minutes before Sunrise.
- **Abhijit Muhurta**: Mid-day victory period (8th part of 15 divisions).

### Module C: Astro-Logic (Dignity & Strength)

**1. Planetary Dignity**

- **Algorithm**: Parashara's Light logic.
- **States**: Exalted, Moolatrikona, Own Sign, Great Friend, Friend, Neutral,
  Enemy, Great Enemy, Debilitated.

## 3. Data Specifications

The library outputs a comprehensive structure:

```typescript
interface DailyPanchang {
  // Calculated Times
  sunrise: number; // Unix ms
  sunset: number;

  // The 5 Angas (With precise Start/End times)
  tithi: {
    index: number;
    name: string;
    startTime: number | null;
    endTime: number | null;
  };
  nakshatra: {
    index: number;
    name: string;
    startTime: number | null;
    endTime: number | null;
  };
  yoga: {
    index: number;
    name: string;
    startTime: number | null;
    endTime: number | null;
  };
  karana: {
    index: number;
    name: string;
    startTime: number | null;
    endTime: number | null;
  };
  vara: string;

  // Astronomy
  ayanamshaValue: number;
  planets: PlanetData[]; // Includes Dignity

  // Muhurats
  muhurats: {
    rahuKalam: { start: number; end: number };
    yamaganda: { start: number; end: number };
    gulika: { start: number; end: number };
    brahmaMuhurta: { start: number; end: number };
    abhijitMuhurta: { start: number; end: number };
  };
}
```

## 4. Verification Standard

Accuracy is validated against:

1. **Swiss Ephemeris**: For raw planetary positions (gold standard).
2. **Drik Ganita Principles**: For algorithmic correctness of Vedic timings.
