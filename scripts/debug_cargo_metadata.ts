const process = new Deno.Command("cargo", {
  args: ["metadata", "--format-version", "1"],
  stdout: "piped",
  stderr: "piped",
});
const { success, stdout, stderr } = await process.output();
console.log("Success:", success);
if (!success) {
  console.error("Error Output:");
  console.error(new TextDecoder().decode(stderr));
} else {
  console.log("Output Length:", stdout.length);
}
