import { type Config } from "tailwindcss";

export default {
  content: [
    "{routes,islands,components}/**/*.{ts,tsx,js,jsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Vedic Futurism Palette
        void: "#050A14",
        "void-deep": "#020408",
        paper: "#F0F4F8",

        // Tech & Schematics
        blueprint: "rgba(0, 240, 255, 0.15)",
        "blueprint-dim": "rgba(0, 240, 255, 0.05)",
        whiteline: "rgba(255, 255, 255, 0.1)",

        // Accents
        "saffron-tech": "#F59E0B",
        "saffron-glow": "rgba(245, 158, 11, 0.4)",
        "saffron-dim": "#C27803",
        "terminal-green": "#10B981",
        "alert-red": "#EF4444",

        // Legacy/Fallback mapping
        background: "#050A14",
        headline: "#ffffff",
        paragraph: "#94a3b8",
        button: "#F59E0B",
        "button-text": "#ffffff",
      },
      fontFamily: {
        sans: ["Inter", "sans-serif"], // Fallback if Neue Montreal isn't loaded
        mono: ["JetBrains Mono", "monospace"],
        display: ["Oswald", "sans-serif"], // Brutalist header
        body: ["Inter", "sans-serif"],
      },
      animation: {
        "spin-slow": "spin 60s linear infinite",
        "pulse-slow": "pulse 4s cubic-bezier(0.4, 0, 0.6, 1) infinite",
      },
      backgroundImage: {
        "grid-pattern":
          "linear-gradient(rgba(0,240,255,0.03) 1px, transparent 1px), linear-gradient(90deg, rgba(0,240,255,0.03) 1px, transparent 1px)",
      },
    },
  },
} satisfies Config;
