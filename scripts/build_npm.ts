import { ensureDir } from "jsr:@std/fs";

async function buildNpm() {
  console.log("Building for Node.js (NPM)...");

  // 1. Build Wasm
  const buildCmd = new Deno.Command("cargo", {
    args: ["build", "--release", "--target", "wasm32-unknown-unknown"],
  });
  const buildOutput = await buildCmd.output();
  if (!buildOutput.success) {
    console.error(
      "Cargo build failed:",
      new TextDecoder().decode(buildOutput.stderr),
    );
    Deno.exit(1);
  }

  // 2. Ensure wasm-bindgen-cli is available and matches version 0.2.105
  // Note: Cargo dependencies pin to 0.2.105 via js-sys.
  const requiredVersion = "0.2.105";
  let installedVersion = "";

  const checkCmd = new Deno.Command("wasm-bindgen", { args: ["--version"] });
  try {
    const output = await checkCmd.output();
    if (output.success) {
      const versionOutput = new TextDecoder().decode(output.stdout).trim();
      const match = versionOutput.match(/wasm-bindgen (\d+\.\d+\.\d+)/);
      if (match) {
        installedVersion = match[1];
      }
    }
  } catch (e) {
    // ignore
  }

  if (installedVersion !== requiredVersion) {
    console.log(
      `wasm-bindgen ${installedVersion} found, but ${requiredVersion} needed. Installing...`,
    );
    // Need to install the EXACT version to match the crate
    const installCmd = new Deno.Command("cargo", {
      args: ["install", "-f", "wasm-bindgen-cli", "--version", requiredVersion],
    });
    const installOutput = await installCmd.output();
    if (!installOutput.success) {
      console.error(
        "Failed to install wasm-bindgen-cli:",
        new TextDecoder().decode(installOutput.stderr),
      );
      Deno.exit(1);
    }
  } else {
    console.log(`wasm-bindgen ${requiredVersion} is already installed.`);
  }

  await ensureDir("./npm");

  // 3. Generate Bindings
  const bindgenCmd = new Deno.Command("wasm-bindgen", {
    args: [
      "target/wasm32-unknown-unknown/release/panchangam.wasm",
      "--out-dir",
      "npm",
      "--target",
      "nodejs",
    ],
  });
  const bindgenOutput = await bindgenCmd.output();
  if (!bindgenOutput.success) {
    console.error(
      "wasm-bindgen failed:",
      new TextDecoder().decode(bindgenOutput.stderr),
    );
    Deno.exit(1);
  }

  // 4. Create package.json
  const packageJson = {
    name: "@fusionstrings/panchangam",
    version: JSON.parse(await Deno.readTextFile("deno.json")).version,
    description:
      "High-precision Vedic Panchangam calculations using Swiss Ephemeris",
    main: "panchangam.js",
    types: "panchangam.d.ts",
    files: [
      "panchangam.js",
      "panchangam.d.ts",
      "panchangam_bg.wasm",
      "panchangam_bg.js",
    ], // wasm-bindgen nodejs output
    license: "MIT",
    repository: {
      type: "git",
      url: "git+https://github.com/fusionstrings/panchangam.git",
    },
    bugs: {
      url: "https://github.com/fusionstrings/panchangam/issues",
    },
  };

  await Deno.writeTextFile(
    "npm/package.json",
    JSON.stringify(packageJson, null, 2),
  );

  // 5. Copy Readme
  await Deno.copyFile("README.md", "npm/README.md");

  console.log("NPM build complete in ./npm");
}

if (import.meta.main) {
  await buildNpm();
}
