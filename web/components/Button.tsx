import { JSX } from "preact";
import { IS_BROWSER } from "$fresh/runtime.ts";

export function Button(props: JSX.HTMLAttributes<HTMLButtonElement>) {
  return (
    <button
      {...props}
      disabled={!IS_BROWSER || props.disabled}
      class={`px-6 py-3 border border-transparent text-base font-medium rounded-none text-button-text bg-button hover:bg-secondary focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-highlight transition-all duration-300 shadow-[4px_4px_0px_0px_rgba(255,137,6,0.3)] hover:shadow-[2px_2px_0px_0px_rgba(255,137,6,0.5)] active:shadow-none active:translate-x-[2px] active:translate-y-[2px] font-mono tracking-wider uppercase ${
        props.class ?? ""
      }`}
    />
  );
}
