export default function HeroEngine() {
  return (
    <div class="relative w-full h-[600px] md:h-[800px] flex items-center justify-center overflow-hidden pointer-events-none select-none">
      {/* Background Grid (Static) */}
      <div class="absolute inset-0 bg-[radial-gradient(circle_at_center,rgba(24,6,242,0.03)_0%,transparent_70%)]">
      </div>

      {/* Engine Container (SVG) */}
      <svg
        viewBox="0 0 1000 1000"
        class="w-full h-full max-w-[1200px] opacity-90"
      >
        {/* DEFS for Glows */}
        <defs>
          <filter id="glow-blue" x="-20%" y="-20%" width="140%" height="140%">
            <feGaussianBlur stdDeviation="5" result="blur" />
            <feComposite in="SourceGraphic" in2="blur" operator="over" />
          </filter>
          <filter id="glow-gold" x="-20%" y="-20%" width="140%" height="140%">
            <feGaussianBlur stdDeviation="3" result="blur" />
            <feComposite in="SourceGraphic" in2="blur" operator="over" />
          </filter>
        </defs>

        {/* --- LAYER 1: OUTER STATIC RINGS --- */}
        <circle
          cx="500"
          cy="500"
          r="450"
          fill="none"
          stroke="rgba(255,255,255,0.05)"
          stroke-width="1"
          stroke-dasharray="10 5"
        />
        <circle
          cx="500"
          cy="500"
          r="480"
          fill="none"
          stroke="rgba(24,6,242,0.1)"
          stroke-width="1"
        />

        {/* --- LAYER 2: THE NAKSHATRA GEAR (Slow Counter-Clockwise) --- */}
        <g class="animate-[spin-slow_120s_linear_infinite_reverse] origin-center">
          <circle
            cx="500"
            cy="500"
            r="350"
            fill="none"
            stroke="rgba(255,255,255,0.1)"
            stroke-width="2"
          />
          {/* Gear Teeth */}
          {[...Array(27)].map((_, i) => (
            <line
              key={i}
              x1="500"
              y1="150"
              x2="500"
              y2="130"
              stroke="white"
              stroke-width="2"
              transform={`rotate(${i * (360 / 27)} 500 500)`}
              class="opacity-30"
            />
          ))}
          {/* Nakshatra Glyphs (Simplified Dots/Lines for abstract tech feel) */}
          {[...Array(27)].map((_, i) => (
            <g key={i} transform={`rotate(${i * (360 / 27)} 500 500)`}>
              <text
                x="500"
                y="180"
                fill="rgba(255,255,255,0.4)"
                font-size="10"
                font-family="monospace"
                text-anchor="middle"
                transform={`rotate(${-i * (360 / 27)} 500 180)`}
              >
                {[
                  "ASH",
                  "BHA",
                  "KRI",
                  "ROH",
                  "MRI",
                  "ARD",
                  "PUN",
                  "PUS",
                  "ASL",
                  "MAG",
                  "PPH",
                  "UPH",
                  "HAS",
                  "CHI",
                  "SWA",
                  "VIS",
                  "ANU",
                  "JYE",
                  "MUL",
                  "PAS",
                  "UAS",
                  "SRA",
                  "DHA",
                  "SAT",
                  "PBH",
                  "UBH",
                  "REV",
                ][i]}
              </text>
            </g>
          ))}
        </g>

        {/* --- LAYER 3: THE RASHI/ZODIAC GEAR (Medium Clockwise) --- */}
        <g class="animate-[spin-slow_60s_linear_infinite] origin-center">
          <circle
            cx="500"
            cy="500"
            r="250"
            fill="none"
            stroke="#EFBF04"
            stroke-width="1"
            class="opacity-30"
          />
          <path
            id="zodiac-path"
            d="M 500 250 A 250 250 0 1 1 499 250"
            fill="none"
          />
          {[...Array(12)].map((_, i) => (
            <g key={i} transform={`rotate(${i * 30} 500 500)`}>
              <path
                d="M 500 250 L 500 280"
                stroke="#EFBF04"
                stroke-width="2"
                class="opacity-50"
              />
              {/* Placeholder Geometric Glyphs for Zodiac */}
              <rect
                x="495"
                y="220"
                width="10"
                height="10"
                fill="none"
                stroke="white"
                stroke-width="1"
                class="opacity-20"
              />
            </g>
          ))}
        </g>

        {/* --- LAYER 4: THE CORE MECHANISM (Fast) --- */}
        <g class="animate-[spin-slow_20s_linear_infinite] origin-center">
          <circle
            cx="500"
            cy="500"
            r="100"
            fill="none"
            stroke="#EFBF04"
            stroke-width="1"
            stroke-dasharray="2 10"
            class="opacity-40"
          />
          <rect
            x="450"
            y="450"
            width="100"
            height="100"
            fill="none"
            stroke="#EFBF04"
            stroke-width="1"
            transform="rotate(45 500 500)"
            class="opacity-30"
          />
        </g>

        {/* --- LAYER 5: SCANNING BEAM --- */}
        <g>
          <line
            x1="500"
            y1="500"
            x2="500"
            y2="50"
            stroke="#EFBF04"
            stroke-width="2"
            filter="url(#glow-blue)"
            class="animate-[spin_4s_linear_infinite] origin-center opacity-80"
          >
          </line>
          <circle
            cx="500"
            cy="500"
            r="5"
            fill="#EFBF04"
            filter="url(#glow-gold)"
          />
        </g>

        {/* --- LAYER 6: HUD OVERLAY (Static) --- */}
        <line
          x1="100"
          y1="500"
          x2="900"
          y2="500"
          stroke="rgba(255,255,255,0.1)"
          stroke-width="1"
        />
        <line
          x1="500"
          y1="100"
          x2="500"
          y2="900"
          stroke="rgba(255,255,255,0.1)"
          stroke-width="1"
        />

        {/* Text Labels */}
        <text
          x="850"
          y="520"
          fill="#EFBF04"
          font-family="monospace"
          font-size="12"
          class="opacity-70"
        >
          AXIS_X [LOCKED]
        </text>
        <text
          x="520"
          y="900"
          fill="#EFBF04"
          font-family="monospace"
          font-size="12"
          class="opacity-70"
        >
          AXIS_Y [LOCKED]
        </text>
      </svg>
    </div>
  );
}
