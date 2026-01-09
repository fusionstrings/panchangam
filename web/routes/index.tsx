import { Head } from "$fresh/runtime.ts";
import HeroEngine from "../islands/HeroEngine.tsx";
import LiveTerminal from "../islands/LiveTerminal.tsx";
import RulerNav from "../components/RulerNav.tsx";
import SpecSheet from "../components/SpecSheet.tsx";
import SchematicBlock from "../components/SchematicBlock.tsx";
import InstallCmd from "../islands/InstallCmd.tsx";

export default function Home() {
  return (
    <>
      <Head>
        <title>Panchangam | The Engine of Time</title>
      </Head>

      <div class="min-h-screen bg-void text-primary selection:bg-tech-blue selection:text-white font-sans overflow-x-hidden">
        <RulerNav />

        {/* --- HERO SECTION: The Engine of Time --- */}
        <section
          class="relative min-h-screen flex items-center justify-center pt-20 overflow-hidden"
          id="engine"
        >
          {/* Background Engine (Parallax Layer) */}
          <div class="absolute inset-0 flex items-center justify-center z-0 scale-125 md:scale-100 opacity-60">
            <HeroEngine />
          </div>

          {/* Content Overlay */}
          <div class="relative z-10 text-center container mx-auto px-4">
            <div class="inline-block px-4 py-1 border border-tech-blue/30 rounded-full bg-void/80 backdrop-blur-sm mb-6">
              <span class="text-tech-blue font-mono text-xs tracking-[0.3em] uppercase">
                Computational Panchangam
              </span>
            </div>

            <h1 class="text-6xl md:text-8xl lg:text-9xl font-display font-black leading-none tracking-tighter mb-8 mix-blend-screen text-transparent bg-clip-text bg-gradient-to-b from-white to-gray-500">
              THE ENGINE
              <br />
              OF TIME
            </h1>

            <p class="text-lg md:text-xl text-gray-400 font-mono max-w-2xl mx-auto mb-12">
              Precision time-keeping for the modern stack. <br />
              <span class="text-tech-blue">Swiss Ephemeris accuracy.</span>{" "}
              WASM velocity.
            </p>

            <div class="flex flex-col md:flex-row items-center justify-center gap-6">
              {/* Terminal-Style CTA */}
              <InstallCmd />
            </div>
          </div>

          {/* Scroll Indicator */}
          <div class="absolute bottom-10 left-1/2 -translate-x-1/2 flex flex-col items-center gap-2 opacity-50">
            <div class="w-[1px] h-12 bg-gradient-to-b from-transparent via-white to-transparent">
            </div>
            <span class="text-[10px] uppercase tracking-widest font-mono">
              SCROLL TO SCAN
            </span>
          </div>
        </section>

        {/* --- SECTION 2: Developer Experience --- */}
        <section
          class="py-24 border-t border-white/10 bg-void/50 backdrop-blur-sm relative z-10"
          id="specs"
        >
          <div class="container mx-auto px-4">
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-16 items-center">
              {/* Left: Terminal & Specs */}
              <div class="space-y-12">
                <div>
                  <h2 class="text-4xl md:text-5xl font-display font-bold mb-6">
                    CELESTIAL LOGIC. <br />{" "}
                    <span class="text-tech-blue">EXECUTED.</span>
                  </h2>
                  <p class="text-gray-400 font-sans text-lg leading-relaxed mb-8">
                    Bypass the mysticism. Access raw, high-precision
                    astronomical data directly in your TypeScript workflow. No
                    lookup tables. No approximate algorithms.
                  </p>
                  <LiveTerminal />
                </div>
                <SpecSheet />
              </div>

              {/* Right: Technical Visuals (Using Legacy Schematics but styled) */}
              <div class="grid gap-6">
                <SchematicBlock
                  mode="accuracy"
                  title="ORBITAL PRECISION"
                  subtitle="DELTA VARIANCE < 0.001"
                  value="LOCKED"
                />
                <SchematicBlock
                  mode="tithi"
                  title="LUNAR MECHANICS"
                  subtitle="PHASE CALCULATION"
                  value="WAXING_GIBBOUS"
                />
                <div class="p-8 border border-white/10 bg-gradient-to-br from-void to-tech-blue/10 flex flex-col justify-end min-h-[200px] group hover:border-tech-blue/50 transition-colors cursor-pointer relative overflow-hidden">
                  <div class="absolute top-0 right-0 p-4 text-tech-blue font-mono text-xs">
                    DOCS_V2
                  </div>
                  <h3 class="text-2xl font-display font-bold mb-2 group-hover:translate-x-2 transition-transform">
                    READ THE DOCS &rarr;
                  </h3>
                </div>
              </div>
            </div>
          </div>
        </section>

        {/* --- FOOTER --- */}
        <footer class="border-t border-white/10 py-12 bg-black">
          <div class="container mx-auto px-4 flex flex-col md:flex-row justify-between items-center text-xs font-mono text-gray-600">
            <div class="uppercase tracking-widest mb-4 md:mb-0">
              System Nominal // {new Date().getFullYear()} FusionStrings
            </div>
            <div class="flex gap-8">
              <a href="#" class="hover:text-tech-blue transition-colors">
                GITHUB
              </a>
              <a href="#" class="hover:text-tech-blue transition-colors">NPM</a>
              <a href="#" class="hover:text-tech-blue transition-colors">
                LICENSE_MIT
              </a>
            </div>
          </div>
        </footer>
      </div>
    </>
  );
}
