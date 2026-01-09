export default function SpecSheet() {
  const specs = [
    { label: "ENGINE KERNEL", value: "SwissEph 2.10" },
    { label: "EXECUTION RUNTIME", value: "WebAssembly (WASM)" },
    { label: "PRECISION VARIANCE", value: "< 0.001 arcseconds" },
    { label: "MEMORY FOOTPRINT", value: "Ultra-Low (< 400KB)" },
    { label: "COLD START LATENCY", value: "~12ms" },
    { label: "LICENSE MODEL", value: "MIT Open Source" },
  ];

  return (
    <div
      id="specs"
      class="border border-white/10 bg-void p-0 relative group overflow-hidden"
    >
      {/* Decorative Corner */}
      <div class="absolute top-0 right-0 w-8 h-8 border-t-2 border-r-2 border-tech-blue opacity-50">
      </div>

      <div class="bg-white/5 p-4 border-b border-white/10 flex justify-between items-center">
        <h3 class="text-sm font-mono text-primary tracking-widest uppercase">
          Hardware Specification
        </h3>
        <div class="w-2 h-2 bg-burnt-saffron animate-pulse"></div>
      </div>

      <div class="divide-y divide-white/5">
        {specs.map((s, i) => (
          <div
            key={i}
            class="flex justify-between items-center p-4 hover:bg-tech-blue/5 transition-colors font-mono text-xs md:text-sm"
          >
            <span class="text-gray-500 uppercase tracking-wider w-1/2">
              {s.label}
            </span>
            <span class="text-tech-blue font-bold w-1/2 text-right">
              {s.value}
            </span>
          </div>
        ))}
      </div>

      <div class="p-4 bg-white/5 border-t border-white/10 text-[10px] text-gray-600 font-mono text-right uppercase">
        Certified for High-Frequency Trading & Astronomy
      </div>
    </div>
  );
}
