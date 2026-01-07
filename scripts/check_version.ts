import { parse } from "jsr:@std/toml";
import { assertEquals } from "jsr:@std/assert";

const textDecoder = new TextDecoder("utf-8");

async function checkVersion() {
  const cargoTomlRaw = await Deno.readFile("Cargo.toml");
  const denoJsonRaw = await Deno.readFile("deno.json");

  const cargoToml = parse(textDecoder.decode(cargoTomlRaw));
  const denoJson = JSON.parse(textDecoder.decode(denoJsonRaw));

  const cargoVersion = (cargoToml as any).package.version;
  const denoVersion = denoJson.version;

  console.log(`Cargo version: ${cargoVersion}`);
  console.log(`Deno version: ${denoVersion}`);

  assertEquals(cargoVersion, denoVersion, "Versions do not match!");
  console.log("Versions match.");
}

if (import.meta.main) {
  await checkVersion();
}
