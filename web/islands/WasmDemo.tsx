import { useEffect, useState } from "preact/hooks";
import * as bindings from "../../lib/panchangam.internal.js";
import {
  __wbg_set_wasm,
  calculate_sunrise,
  get_version,
  Location,
} from "../../lib/panchangam.internal.js";

export default function WasmDemo() {
  const [version, setVersion] = useState<string>("Initializing...");
  const [sunrise, setSunrise] = useState<string>("");
  const [loaded, setLoaded] = useState(false);

  useEffect(() => {
    async function loadWasm() {
      try {
        const wasmUrl = new URL("/panchangam.wasm", globalThis.location.href);
        const imports = {
          "./panchangam.internal.js": { ...bindings },
        };

        const { instance } = await WebAssembly.instantiateStreaming(
          fetch(wasmUrl),
          // @ts-ignore: TS doesn't like extra bindings in imports
          imports,
        );

        __wbg_set_wasm(instance.exports);
        if (
          instance.exports.__wbindgen_start &&
          typeof instance.exports.__wbindgen_start === "function"
        ) {
          // @ts-ignore: generic wasm export
          instance.exports.__wbindgen_start();
        }

        setLoaded(true);
        const v = get_version();
        setVersion(v);

        // Example: Sunrise in New Delhi today
        // Lat: 28.6139, Lon: 77.2090
        const loc = new Location(28.6139, 77.2090, 200);
        const today = new Date();
        const s = calculate_sunrise(
          today.getFullYear(),
          today.getMonth() + 1,
          today.getDate(),
          loc,
        );
        setSunrise(new Date(s).toLocaleTimeString());
      } catch (e) {
        console.error("WASM Load Error:", e);
        setVersion("Error via Console");
      }
    }
    loadWasm();
  }, []);

  return (
    <div class="p-8 border border-white/10 bg-glass backdrop-blur-md rounded-2xl max-w-2xl mx-auto shadow-2xl relative overflow-hidden group hover:border-gold/30 transition-all duration-500">
      <div class="absolute top-0 right-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity text-gold">
        <svg width="64" height="64" viewBox="0 0 24 24" fill="currentColor">
          <path d="M12 2L2 7l10 5 10-5-10-5zm0 9l2.5-1.25L12 8.5l-2.5 1.25L12 11zm0 2.5l-5-2.5-5 2.5L12 22l10-8.5-5-2.5-5 2.5z" />
        </svg>
      </div>

      <h2 class="font-display text-2xl text-gold mb-6 flex items-center gap-3">
        <span class="w-2 h-2 bg-saffron rounded-full animate-pulse"></span>
        Live Engine Status
      </h2>

      <div class="space-y-4 font-mono text-main text-sm md:text-base">
        <div class="flex justify-between items-center border-b border-white/10 pb-3 border-dashed">
          <span class="text-paragraph uppercase tracking-widest text-xs">
            Core Library
          </span>
          <span class="text-main font-bold">Swiss Ephemeris {version}</span>
        </div>
        <div class="flex justify-between items-center border-b border-white/10 pb-3 border-dashed">
          <span class="text-paragraph uppercase tracking-widest text-xs">
            Sunrise (Delhi)
          </span>
          <span class="text-saffron font-bold">
            {sunrise || "Calculating..."}
          </span>
        </div>
        <div class="flex justify-between items-center pt-2">
          <span class="text-paragraph uppercase tracking-widest text-xs">
            System
          </span>
          <span
            class={`font-bold ${
              loaded ? "text-emerald-400" : "text-yellow-400"
            }`}
          >
            {loaded ? "READY" : "BOOTING"}
          </span>
        </div>
      </div>
    </div>
  );
}
