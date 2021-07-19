// deno-lint-ignore-file

const filenameBase = "autopilot_deno";

let filenameSuffix = ".so";
let filenamePrefix = "lib";

if (Deno.build.os === "windows") {
  filenameSuffix = ".dll";
  filenamePrefix = "";
} else if (Deno.build.os === "darwin") {
  filenameSuffix = ".dylib";
}

const filename =
  `target/release/${filenamePrefix}${filenameBase}${filenameSuffix}`;

Deno.openPlugin(filename);

let exports = {};

[
  "type",
  "notify",
  "smoothMouseMove",
  "mouseMove",
  "mouseClick",
  "mouseScroll",
  "screenshot",
  "screensize",
  "screenscale",
  "mousePixelColor",
  "mousePosition",
  "alert",
  "toggleKey",
  "tap",
].forEach((name) => {
  const fn = Deno.core.ops()["op_" + name];
  if (!fn) throw new Error(`${name} op was not registered`);
  exports[name] = function (arg, buf) {
    return Deno.core.opSync("op_" + name, arg, buf || new Uint8Array());
  };
});

export default exports;
