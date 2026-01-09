export default function SpecSheet() {
  const specs = [
    { label: "Ephemeris Core", value: "SwissEph 2.10.03" },
    { label: "Precision (Time)", value: "1ms (Int64)" },
    { label: "Precision (Space)", value: "0.0000001°" },
    { label: "WASM Payload", value: "< 400KB (Brotli)" }, // Verified 381KB raw, <150KB brotli likely
    { label: "Cold Start", value: "~12ms" },
    { label: "License", value: "MIT" },
  ];

  return (
    <div class="border border-white/10 bg-void-deep p-8 relative">
      <h3 class="text-xl font-display uppercase tracking-widest mb-6 text-white border-l-2 border-saffron-tech pl-4">
        Technical Specifications
      </h3>
      <div class="space-y-3 font-mono text-sm">
        {specs.map((s, i) => (
          <div key={i} class="flex justify-between items-end group">
            <span class="text-gray-400 group-hover:text-white transition-colors">
              {s.label}
            </span>
            <div class="flex-1 mx-4 border-b border-white/10 border-dotted mb-1 opacity-30">
            </div>
            <span class="text-saffron-tech">{s.value}</span>
          </div>
        ))}
      </div>
      <div class="mt-8 pt-4 border-t border-white/5 text-[10px] text-gray-500 font-mono uppercase text-right">
        Verified by FusionStrings Lab
      </div>
    </div>
  );
}
