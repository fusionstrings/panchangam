import { type Config } from "tailwindcss";

export default {
  content: [
    "{routes,islands,components}/**/*.{ts,tsx,js,jsx}",
  ],
  theme: {
    extend: {
      colors: {
        background: "#020617", // void
        headline: "#ffffff",
        paragraph: "#94a3b8", // slate-400
        button: "#f59e0b", // saffron
        "button-text": "#ffffff",
        stroke: "rgba(255,255,255,0.1)",
        main: "#ffffff",
        highlight: "#f59e0b", // saffron
        secondary: "#D4AF37", // gold
        tertiary: "#be123c", // om (rose-700)
        void: "#020617",
        saffron: "#f59e0b",
        gold: "#D4AF37",
        om: "#be123c",
        glass: "rgba(255, 255, 255, 0.05)",
      },
      fontFamily: {
        sans: ["Outfit", "Inter", "sans-serif"],
        serif: ["Cinzel", "serif"],
        mono: ["JetBrains Mono", "monospace"],
        display: ["Cinzel", "serif"],
        body: ["Outfit", "sans-serif"],
      },
      animation: {
        "spin-slow": "spin 60s linear infinite",
        "pulse-slow": "pulse 4s cubic-bezier(0.4, 0, 0.6, 1) infinite",
      },
      backgroundImage: {
        "hero-mandala": "url('/hero-mandala.png')",
        "gradient-radial": "radial-gradient(var(--tw-gradient-stops))",
      },
    },
  },
} satisfies Config;
