import { type Config } from "tailwindcss";

export default {
  content: [
    "{routes,islands,components}/**/*.{ts,tsx,js,jsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Horological Mysticism Palette (Strict)
        void: "#050505", // Void Black
        primary: "#E5E5E5", // Off-White / Cream
        "tech-amber": "#EFBF04", // The new Primary Accent (Gold/Amber)
        "tech-blue": "#EFBF04", // ALIASED TO GOLD for instant component fixes (User requested NO BLUE)
        "burnt-saffron": "#FF5722", // Alerts / Primary CTA
        "grid-line": "rgba(255, 255, 255, 0.08)",

        // Semantic Mappings
        background: "#050505",
        headline: "#E5E5E5",
        paragraph: "#A3A3A3",

        // Legacy support
        "saffron-tech": "#FF5722",
        blueprint: "rgba(239, 191, 4, 0.15)", // Mapped to Amber alpha
      },
      fontFamily: {
        sans: ["Space Grotesk", "Montserrat", "sans-serif"],
        mono: ["JetBrains Mono", "monospace"],
        display: ["Space Grotesk", "sans-serif"],
        body: ["Inter", "sans-serif"],
      },
      animation: {
        "spin-slow": "spin 60s linear infinite",
        "pulse-slow": "pulse 4s cubic-bezier(0.4, 0, 0.6, 1) infinite",
        "scan": "scan 2s linear infinite",
      },
      keyframes: {
        scan: {
          "0%": { transform: "translateY(-100%)" },
          "100%": { transform: "translateY(100%)" },
        },
      },
      cursor: {
        "crosshair": "crosshair",
        "wait": "wait",
        "text": "text",
        "move": "move",
        "not-allowed": "not-allowed",
      },
    },
  },
} satisfies Config;
