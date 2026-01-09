export default function RulerNav() {
  return (
    <nav class="fixed top-0 left-0 right-0 z-50 bg-void/90 backdrop-blur-md border-b border-white/10">
      {/* Ruler Ticks */}
      <div class="absolute bottom-0 left-0 right-0 h-2 w-full flex justify-between overflow-hidden opacity-30 pointer-events-none">
        {[...Array(100)].map((_, i) => (
          <div
            key={i}
            class={`h-full w-[1px] bg-white ${
              i % 10 === 0 ? "h-full bg-tech-blue" : "h-1/2 mt-auto"
            }`}
          >
          </div>
        ))}
      </div>

      <div class="container mx-auto px-6 h-16 flex items-center justify-between">
        {/* Logo / Brand */}
        <div class="flex items-center gap-3">
          <div class="w-6 h-6 border border-tech-blue flex items-center justify-center transform rotate-45">
            <div class="w-2 h-2 bg-tech-blue"></div>
          </div>
          <span class="font-display font-bold text-lg tracking-widest text-primary">
            PANCHANGAM
          </span>
        </div>

        {/* Links */}
        <div class="hidden md:flex gap-8 font-mono text-xs tracking-widest text-gray-400">
          <a href="#engine" class="hover:text-tech-blue transition-colors">
            ENGINE
          </a>
          <a href="#specs" class="hover:text-tech-blue transition-colors">
            SPECS
          </a>
          <a
            href="/docs/index.html"
            class="hover:text-tech-blue transition-colors"
          >
            DOCS
          </a>
        </div>

        {/* Status */}
        <div class="font-mono text-xs text-tech-blue border border-tech-blue/30 px-3 py-1 rounded-full bg-tech-blue/5">
          v2.10.03 :: STABLE
        </div>
      </div>
    </nav>
  );
}
