export type SchematicMode = "tithi" | "speed" | "accuracy";

interface SchematicProps {
  mode: SchematicMode;
  title: string;
  subtitle: string;
  value?: string;
}

export default function SchematicBlock(
  { mode, title, subtitle, value }: SchematicProps,
) {
  return (
    <div class="group relative p-6 bg-void/50 border border-blueprint technical-border h-full flex flex-col justify-between overflow-hidden">
      {/* Background Grid */}
      <div class="absolute inset-0 bg-[linear-gradient(rgba(0,240,255,0.03)_1px,transparent_1px),linear-gradient(90deg,rgba(0,240,255,0.03)_1px,transparent_1px)] bg-[size:20px_20px] pointer-events-none">
      </div>

      <div class="relative z-10">
        <div class="flex justify-between items-start mb-4">
          <div>
            <h3 class="text-white font-display text-lg tracking-widest uppercase">
              {title}
            </h3>
            <p class="text-xs text-saffron-dim font-mono mt-1">{subtitle}</p>
          </div>
          <div class="text-right">
            <div class="w-2 h-2 bg-terminal-green rounded-full animate-pulse ml-auto mb-1">
            </div>
            <span class="text-[10px] text-blueprint font-mono">
              SYS.NOMINAL
            </span>
          </div>
        </div>

        {/* Visualizations */}
        <div class="my-6 flex justify-center items-center h-32 w-full">
          {mode === "tithi" && (
            <svg viewBox="0 0 100 100" class="w-full h-full text-blueprint">
              {/* Orbits */}
              <circle
                cx="50"
                cy="50"
                r="40"
                fill="none"
                stroke="currentColor"
                stroke-width="0.5"
                stroke-dasharray="2 2"
                class="animate-[spin-slow_20s_linear_infinite]"
              />
              <circle
                cx="50"
                cy="50"
                r="25"
                fill="none"
                stroke="currentColor"
                stroke-width="0.5"
              />

              {/* Sun */}
              <circle cx="50" cy="50" r="5" fill="#F59E0B" />

              {/* Earth/Moon Angle Lines */}
              <line
                x1="50"
                y1="50"
                x2="90"
                y2="50"
                stroke="currentColor"
                stroke-width="0.5"
              />
              <line
                x1="50"
                y1="50"
                x2="75"
                y2="25"
                stroke="currentColor"
                stroke-width="0.5"
                class="opacity-50"
              />

              {/* Moon */}
              <circle
                cx="90"
                cy="50"
                r="3"
                fill="#fff"
                class="animate-[spin_4s_linear_infinite_reverse] origin-[50px_50px]"
              />

              {/* Arc */}
              <path
                d="M 80 50 A 30 30 0 0 0 71 29"
                fill="none"
                stroke="#F59E0B"
                stroke-width="1"
              />
              <text
                x="82"
                y="40"
                font-size="4"
                fill="currentColor"
                fontFamily="monospace"
              >
                12°
              </text>
            </svg>
          )}

          {mode === "speed" && (
            <div class="w-full h-full flex items-end gap-2 px-4 relative">
              {/* Grid lines */}
              <div class="absolute inset-0 border-l border-b border-blueprint/30">
              </div>
              {/* JS Bar */}
              <div class="w-1/3 bg-blueprint/20 h-[30%] relative group-hover:h-[30%] transition-all border-t border-r border-blueprint flex items-end justify-center pb-2">
                <span class="text-[8px] text-white/50 writing-mode-vertical rotate-180">
                  JS NATIVE
                </span>
              </div>
              {/* WASM Bar */}
              <div class="w-1/3 bg-saffron-tech h-[95%] relative animate-[pulse-slow_4s_ease-in-out_infinite] flex items-end justify-center pb-2 relative">
                <div class="absolute -top-4 text-xs font-mono text-saffron-tech">
                  20x
                </div>
                <span class="text-[8px] text-black font-bold writing-mode-vertical rotate-180">
                  WASM
                </span>
              </div>
            </div>
          )}

          {mode === "accuracy" && (
            <svg viewBox="0 0 100 50" class="w-full h-full text-blueprint">
              <line
                x1="0"
                y1="25"
                x2="100"
                y2="25"
                stroke="currentColor"
                stroke-width="0.5"
                stroke-dasharray="1 1"
              />
              <path
                d="M 0 25 Q 25 5 50 25 T 100 25"
                fill="none"
                stroke="#F59E0B"
                stroke-width="1"
              />
              <path
                d="M 0 25 Q 25 8 50 25 T 100 25"
                fill="none"
                stroke="currentColor"
                stroke-width="0.5"
                class="opacity-50"
              />
              <rect
                x="40"
                y="20"
                width="20"
                height="10"
                fill="none"
                stroke="#fff"
                stroke-width="0.5"
              />
              <line
                x1="50"
                y1="20"
                x2="50"
                y2="10"
                stroke="#fff"
                stroke-width="0.5"
              />
              <text
                x="45"
                y="8"
                font-size="3"
                fill="#fff"
                fontFamily="monospace"
              >
                Δ &lt; 0.0001&quot;
              </text>
            </svg>
          )}
        </div>
      </div>

      <div class="border-t border-blueprint/20 pt-2 flex justify-between items-center relative z-10">
        <span class="font-mono text-[10px] text-white/50">
          {value || "DATA_STREAM_ACTIVE"}
        </span>
        <div class="flex gap-1">
          <div class="w-1 h-1 bg-white/20"></div>
          <div class="w-1 h-1 bg-white/20"></div>
          <div class="w-1 h-1 bg-white/50"></div>
        </div>
      </div>
    </div>
  );
}
