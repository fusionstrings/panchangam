import { resolve } from "@std/path";
import { exists } from "@std/fs";

// 1. Locate WASI SDK if not already set
if (!Deno.env.get("WASI_SDK_PATH")) {
  // Check for the toolchain in the sibling swiss-eph directory (common dev layout)
  const siblingSdk = resolve(
    Deno.cwd(),
    "../swiss-eph/toolchain/wasi-sdk-24.0",
  );

  if (await exists(siblingSdk)) {
    console.log(
      `%c[Build] Auto-detected WASI SDK: ${siblingSdk}`,
      "color: green",
    );
    Deno.env.set("WASI_SDK_PATH", siblingSdk);
  } else {
    // We don't error here because the user might have set up the environment
    // such that cc-rs finds it via other means (e.g. system PATH),
    // or they might be running a build that doesn't trigger the C compilation.
    console.warn(
      "%c[Build] Warning: WASI_SDK_PATH not set. Build may fail if native extensions need recompilation.",
      "color: yellow",
    );
  }
} else {
  console.log(
    `%c[Build] Using configured WASI SDK: ${Deno.env.get("WASI_SDK_PATH")}`,
    "color: green",
  );
}

// 2. Run wasmbuild
// We pass through all arguments to the underlying wasmbuild command
const args = [
  "run",
  "-A",
  "@deno/wasmbuild",
  "-p",
  "panchangam",
  ...Deno.args,
];

console.log(`[Build] Running: deno ${args.join(" ")}`);

const cmd = new Deno.Command(Deno.execPath(), {
  args,
  stdout: "inherit",
  stderr: "inherit",
  env: {
    "WASI_SDK_PATH": Deno.env.get("WASI_SDK_PATH")!,
  },
});

const status = await cmd.spawn().status;

if (!status.success) {
  console.error(
    `%c[Build] Build failed with exit code ${status.code}`,
    "color: red",
  );
  Deno.exit(status.code);
}
