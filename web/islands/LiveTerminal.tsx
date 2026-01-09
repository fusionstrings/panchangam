import { useEffect, useRef, useState } from "preact/hooks";
import * as bindings from "../../lib/panchangam.internal.js";
import {
  __wbg_set_wasm,
  calculate_sunrise,
  Location,
} from "../../lib/panchangam.internal.js";

export default function LiveTerminal() {
  const [lines, setLines] = useState<string[]>([
    "INITIALIZING KERNEL...",
    "LOADING SWISSEPH v2.10.03...",
    "Waiting for WASM Core...",
  ]);
  const waInstance = useRef<any>(null);

  const [userLoc, setUserLoc] = useState<{ lat: number; lon: number } | null>(
    null,
  );

  // Load WASM
  useEffect(() => {
    async function init() {
      try {
        const wasmUrl = new URL("/panchangam.wasm", globalThis.location.href);
        const imports = { "./panchangam.internal.js": { ...bindings } };
        const { instance } = await WebAssembly.instantiateStreaming(
          fetch(wasmUrl),
          imports as any,
        );
        __wbg_set_wasm(instance.exports);
        if (instance.exports.__wbindgen_start) {
          (instance.exports as any).__wbindgen_start();
        }
        waInstance.current = instance;
        setLines((prev) => [...prev, "CORE ONLINE: 64-BIT PRECISION ENABLED."]);

        // Request Geolocation
        if (navigator.geolocation) {
          setLines(
            (prev) => [...prev, "REQUESTING SATELLITE FIX (USER_LOC)..."],
          );
          navigator.geolocation.getCurrentPosition(
            (pos) => {
              setUserLoc({
                lat: pos.coords.latitude,
                lon: pos.coords.longitude,
              });
              setLines(
                (prev) => [
                  ...prev,
                  `LOCK ACQUIRED: ${pos.coords.latitude.toFixed(2)}N ${
                    pos.coords.longitude.toFixed(2)
                  }E`,
                ],
              );
            },
            (err) => {
              setLines((prev) => [...prev, `SAT FIX FAILED: ${err.message}`]);
            },
          );
        }
      } catch (e) {
        setLines((prev) => [...prev, `KERNEL ERROR: ${e}`]);
      }
    }
    init();
  }, []);

  useEffect(() => {
    const locations = [
      { name: "TOKYO", lat: 35.6, lon: 139.6, timezone: "Asia/Tokyo" },
      { name: "NEW_YORK", lat: 40.7, lon: -74.0, timezone: "America/New_York" },
      { name: "LONDON", lat: 51.5, lon: -0.1, timezone: "Europe/London" },
      { name: "PARIS", lat: 48.8, lon: 2.3, timezone: "Europe/Paris" },
      { name: "DUBAI", lat: 25.2, lon: 55.2, timezone: "Asia/Dubai" },
      { name: "SINGAPORE", lat: 1.3, lon: 103.8, timezone: "Asia/Singapore" },
      { name: "VATICAN", lat: 41.9, lon: 12.4, timezone: "Europe/Rome" },
      {
        name: "VARANASI",
        lat: 25.3,
        lon: 82.9,
        timezone: "Asia/Kolkata",
      }, /* Retaining spiritual core */
      { name: "UJJAIN", lat: 23.1, lon: 75.7, timezone: "Asia/Kolkata" },
    ];

    const interval = setInterval(() => {
      if (!waInstance.current) return;

      try {
        const now = new Date();
        // Prioritize User Location 50% of the time if available
        let locData;
        if (userLoc && Math.random() > 0.5) {
          locData = {
            name: "USER_LOC",
            lat: userLoc.lat,
            lon: userLoc.lon,
            timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
          };
        } else {
          locData = locations[Math.floor(Math.random() * locations.length)];
        }

        const loc = new Location(locData.lat, locData.lon, 0.0);

        const t0 = performance.now();
        // REAL CALCULATION: Actually compute sunrise for today
        const sunriseTs = calculate_sunrise(
          now.getFullYear(),
          now.getMonth() + 1,
          now.getDate(),
          loc,
        );
        const t1 = performance.now();

        const timeStr = new Date(sunriseTs).toLocaleTimeString("en-US", {
          hour12: false,
          hour: "2-digit",
          minute: "2-digit",
          second: "2-digit",
          timeZone: locData.timezone,
          timeZoneName: "short",
        });
        const latency = (t1 - t0).toFixed(3);
        const newLine = `[${
          now.toISOString().split("T")[1].slice(0, -1)
        }] CALC SUNRISE_${locData.name}... ${timeStr} (${latency}ms)`;

        setLines((prev) => {
          const newLines = [...prev, newLine];
          if (newLines.length > 8) return newLines.slice(1);
          return newLines;
        });
      } catch (e) {
        console.error(e);
      }
    }, 1200);
    return () => clearInterval(interval);
  }, [userLoc]);

  return (
    <div class="relative p-6 w-full max-w-lg bg-void border border-white/10 rounded-lg overflow-hidden group">
      {/* CRT Scanline Overlay */}
      <div class="absolute inset-0 bg-[linear-gradient(rgba(18,16,242,0.03)_50%,transparent_50%)] bg-[size:100%_4px] pointer-events-none z-20">
      </div>
      <div class="absolute inset-0 bg-gradient-to-b from-transparent to-tech-blue/5 pointer-events-none z-10 animate-scan">
      </div>

      {/* Header */}
      <div class="flex justify-between items-center mb-4 border-b border-white/10 pb-2 relative z-30">
        <span class="text-xs font-mono text-gray-500 uppercase tracking-widest">
          Live Execution Log (REAL-TIME)
        </span>
        <div class="flex gap-2">
          <div class="w-2 h-2 rounded-full bg-tech-blue animate-pulse"></div>
        </div>
      </div>

      {/* Content */}
      <div class="font-mono text-xs md:text-sm space-y-1 relative z-30 min-h-[150px]">
        {lines.map((line, i) => (
          <div key={i} class="text-tech-blue/80">
            <span class="mr-2 text-white/30">{">"}</span>
            {line}
          </div>
        ))}
        <div class="text-tech-blue animate-pulse mt-2">_</div>
      </div>

      {/* Glow Effect */}
      <div class="absolute -inset-1 bg-tech-blue/20 blur-xl opacity-0 group-hover:opacity-20 transition-opacity duration-500 z-0">
      </div>
    </div>
  );
}
