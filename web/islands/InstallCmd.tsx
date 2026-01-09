import { useState } from "preact/hooks";

export default function InstallCmd() {
  const [copied, setCopied] = useState(false);
  const cmd = "npm install @fusionstrings/panchangam";

  const copy = async () => {
    try {
      await navigator.clipboard.writeText(cmd);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error("Failed to copy", err);
    }
  };

  return (
    <div
      onClick={copy}
      class="group relative cursor-pointer select-none"
    >
      <div class="absolute inset-0 bg-tech-amber blur-lg opacity-20 group-hover:opacity-40 transition-opacity">
      </div>
      <div class="relative bg-void border border-tech-amber px-8 py-4 font-mono text-tech-amber hover:bg-tech-amber hover:text-void transition-colors duration-200 tracking-widest text-sm font-bold flex items-center gap-3">
        <span>{copied ? "COPIED TO CLIPBOARD" : cmd}</span>
        {!copied && <span class="w-2 h-2 bg-current animate-pulse"></span>}
        {copied && <span class="w-2 h-2 bg-current">✓</span>}
      </div>
    </div>
  );
}
