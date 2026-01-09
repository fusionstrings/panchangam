import { Head } from "$fresh/runtime.ts";
import InteractiveConsole from "../islands/InteractiveConsole.tsx";
import SchematicBlock from "../components/SchematicBlock.tsx";
import SpecSheet from "../components/SpecSheet.tsx";

export default function Home() {
  return (
    <>
      <Head>
        <title>Panchangam | High-Precision Vedic Astronomy</title>
      </Head>

      {/* Main Container */}
      <div class="min-h-screen relative overflow-x-hidden selection:bg-saffron-tech selection:text-black">
        {/* Background Decorative Elements */}
        <div class="fixed inset-0 pointer-events-none z-0">
          <div class="absolute inset-0 bg-[radial-gradient(circle_at_50%_50%,rgba(5,10,20,0)_0%,rgba(5,10,20,1)_100%)]">
          </div>
          {/* Center Mandala / Gear Ghost */}
          <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[800px] h-[800px] border border-white/5 rounded-full opacity-20 animate-[spin-slow_120s_linear_infinite]">
          </div>
          <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] border border-white/5 rounded-full opacity-30 animate-[spin-slow_80s_linear_infinite_reverse]">
          </div>
        </div>

        {/* Content Layer */}
        <div class="relative z-10 container mx-auto px-4 py-12 md:py-24 max-w-7xl">
          {/* Header */}
          <header class="mb-20 text-center relative">
            <div class="inline-block border border-saffron-tech/30 px-3 py-1 mb-6 rounded-full bg-saffron-dim/10 backdrop-blur-sm">
              <span class="text-saffron-tech font-mono text-xs tracking-[0.2em] uppercase">
                Vedic Futurism // v2.0.0
              </span>
            </div>
            <h1 class="text-6xl md:text-8xl lg:text-9xl font-display font-medium leading-none tracking-tighter text-transparent bg-clip-text bg-gradient-to-b from-white to-white/40 mb-6">
              PANCHANGAM
            </h1>
            <p class="text-xl md:text-2xl text-gray-400 font-sans max-w-2xl mx-auto font-light leading-relaxed">
              High-precision astronomical engine. <br />
              <span class="text-white">Swiss Ephemeris core</span>{" "}
              verified for 5000+ years.
            </p>

            {/* Visual Anchor Line */}
            <div class="h-16 w-[1px] bg-gradient-to-b from-saffron-tech to-transparent mx-auto mt-12">
            </div>
          </header>

          {/* Main Grid Layout */}
          <div class="grid grid-cols-1 lg:grid-cols-12 gap-8 mb-24 items-start">
            {/* Left Column: Spec & Details */}
            <div class="lg:col-span-4 space-y-8">
              <div class="lg:sticky lg:top-8">
                <SpecSheet />

                <div class="mt-8 text-sm text-gray-500 font-mono">
                  <p className="mb-4">ENGINEERING NOTES:</p>
                  <p>
                    Compiled to WebAssembly for near-native performance.
                    Eliminates network latency for high-frequency calculations.
                    Ideal for serverless edge workers.
                  </p>
                </div>
              </div>
            </div>

            {/* Center/Right Column: Interactive & Visuals */}
            <div class="lg:col-span-8 space-y-8">
              {/* Console Island */}
              <div class="flex justify-center lg:justify-end mb-12 relative z-50 min-h-[300px]">
                <InteractiveConsole />
              </div>

              {/* Schematics Grid */}
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <SchematicBlock
                  mode="accuracy"
                  title="Precision"
                  subtitle="ARC-SECOND ACCURACY"
                  value="DELTA_V < 1e-7"
                />
                <SchematicBlock
                  mode="speed"
                  title="Velocity"
                  subtitle="WASM COMPILATION"
                  value="20ms / YEAR"
                />
                <SchematicBlock
                  mode="tithi"
                  title="Dynamics"
                  subtitle="ORBITAL MECHANICS"
                  value="MOON_PHASE: WAXING"
                />
                <div class="group relative p-6 bg-void/50 border border-white/5 h-full flex flex-col justify-center items-center text-center hover:bg-white/5 transition-colors cursor-pointer">
                  <a href="/docs" class="absolute inset-0"></a>
                  <h3 class="text-2xl font-display text-white mb-2">
                    Documentation
                  </h3>
                  <p class="text-sm text-gray-400 font-mono">
                    View full API Reference &rarr;
                  </p>
                </div>
              </div>
            </div>
          </div>

          {/* Footer */}
          <footer class="border-t border-white/10 pt-12 flex flex-col md:flex-row justify-between items-center text-gray-500 font-mono text-xs">
            <div class="mb-4 md:mb-0">
              &copy; {new Date().getFullYear()}{" "}
              FusionStrings. All systems nominal.
            </div>
            <div class="flex gap-6 uppercase tracking-widest">
              <a href="#" class="hover:text-saffron-tech transition-colors">
                GitHub
              </a>
              <a href="#" class="hover:text-saffron-tech transition-colors">
                NPM
              </a>
              <a href="#" class="hover:text-saffron-tech transition-colors">
                Crates.io
              </a>
            </div>
          </footer>
        </div>
      </div>
    </>
  );
}
