// deno-lint-ignore-file
import { prepare } from "https://deno.land/x/plugin_prepare@v0.8.0/mod.ts";

const releaseUrl =
  "https://github.com/littledivy/autopilot-deno/releases/0.3.1/download";
const filenameBase = "autopilot_deno";

let filenameSuffix = ".so";
let filenamePrefix = "lib";

if (Deno.build.os === "windows") {
  filenameSuffix = ".dll";
  filenamePrefix = "";
} else if (Deno.build.os === "darwin") {
  filenameSuffix = ".dylib";
}

const filename = `${filenamePrefix}${filenameBase}${filenameSuffix}`;

const pluginOptions = {
  name: filenameBase,
  urls: {
    darwin: `${releaseUrl}/${filename}`,
    windows: `${releaseUrl}/${filename}`,
    linux: `${releaseUrl}/${filename}`,
  },
};

if (Deno.env.get("DEV")) {
  Deno.openPlugin("target/release/" + filename);
} else {
  await prepare(pluginOptions);
}

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
