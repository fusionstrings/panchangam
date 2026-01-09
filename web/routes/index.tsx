import { Button } from "../components/Button.tsx";
import WasmDemo from "../islands/WasmDemo.tsx";

export default function Home() {
  return (
    <div class="min-h-screen relative overflow-hidden bg-void text-main selection:bg-saffron selection:text-void font-body">
      {/* Background Elements */}
      <div class="fixed inset-0 z-0">
        <div class="absolute inset-0 bg-hero-mandala bg-cover bg-center opacity-30 mix-blend-screen animate-pulse-slow">
        </div>
        <div class="absolute inset-0 bg-gradient-to-b from-void via-transparent to-void">
        </div>
      </div>

      <div class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 flex flex-col items-center justify-center min-h-screen text-center py-20">
        {/* Hero Section */}
        <div class="mb-16 space-y-6">
          <div class="inline-block mb-4 px-4 py-1.5 rounded-full border border-gold/30 bg-gold/5 backdrop-blur-md">
            <span class="text-gold text-xs font-mono uppercase tracking-widest">
              Vedic Astronomy Engine v1.0
            </span>
          </div>
          <h1 class="font-display text-6xl md:text-8xl lg:text-9xl font-black tracking-tight text-white drop-shadow-[0_0_15px_rgba(255,255,255,0.3)]">
            PANCHANGAM
          </h1>
          <p class="font-sans font-light text-2xl md:text-3xl text-saffron tracking-wide max-w-3xl mx-auto animate-fade-in-up">
            Cosmic Precision for the Modern Age
          </p>
          <p class="font-body text-paragraph text-lg md:text-xl max-w-2xl mx-auto leading-relaxed opacity-80">
            High-performance WebAssembly libraries for Tithi, Nakshatra, and
            Yoga calculations. Powered by Swiss Ephemeris accuracy.
          </p>
        </div>

        <div class="flex flex-col sm:flex-row gap-6 mb-24">
          <Button class="bg-saffron text-void hover:bg-gold transition-all duration-300 font-bold px-8 py-4 text-lg shadow-[0_0_20px_rgba(245,158,11,0.4)] hover:shadow-[0_0_30px_rgba(212,175,55,0.6)]">
            Explore Documentation
          </Button>
          <Button class="bg-glass backdrop-blur-md border border-stroke text-main hover:bg-white/10 hover:border-gold/50 transition-all duration-300 px-8 py-4 text-lg font-light tracking-wide">
            View Source
          </Button>
        </div>

        {/* Bento Grid Features */}
        <div class="w-full max-w-6xl grid grid-cols-1 md:grid-cols-12 gap-6 mb-32">
          {/* Main Feature - Drig Ganita */}
          <div class="md:col-span-8 p-8 border border-white/10 bg-glass backdrop-blur-md rounded-2xl hover:border-gold/30 transition-all duration-500 group text-left relative overflow-hidden">
            <div class="absolute top-0 right-0 -mt-20 -mr-20 w-64 h-64 bg-saffron/20 rounded-full blur-[80px] group-hover:bg-saffron/30 transition-all">
            </div>
            <h3 class="font-display text-3xl text-white mb-4 relative z-10">
              Drig Ganita
            </h3>
            <p class="text-paragraph text-lg relative z-10">
              Astronomical precision aligned with NASA Jet Propulsion Laboratory
              data. Calculations that respect the physical reality of celestial
              bodies.
            </p>
          </div>

          {/* Side Feature - WASM */}
          <div class="md:col-span-4 p-8 border border-white/10 bg-glass backdrop-blur-md rounded-2xl hover:border-tertiary/40 transition-all duration-500 text-left group">
            <h3 class="font-mono text-xl text-tertiary mb-3">#WASM_POWERED</h3>
            <p class="text-paragraph text-sm">
              Blazing fast calculations running anywhere: Browser, Edge, Deno,
              Node.
              <span class="block mt-4 text-xs text-white/50">
                0ms overhead.
              </span>
            </p>
          </div>

          {/* Bottom Feature - Rust Core */}
          <div class="md:col-span-4 p-8 border border-white/10 bg-glass backdrop-blur-md rounded-2xl hover:border-gold/30 transition-all duration-500 text-left">
            <h3 class="font-display text-2xl text-gold mb-2">Rust Core</h3>
            <p class="text-paragraph">
              Type-safe, correct-by-construction reliability.
            </p>
          </div>

          {/* Interactive Demo Area Placeholder */}
          <div class="md:col-span-8 p-1 border border-white/10 bg-glass backdrop-blur-md rounded-2xl overflow-hidden relative min-h-[300px] flex items-center justify-center">
            <div class="absolute inset-0 bg-void/50 z-0"></div>
            <div class="relative z-10 w-full h-full p-6">
              <WasmDemo />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
