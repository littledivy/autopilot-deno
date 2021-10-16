// deno-lint-ignore-file
import { prepare } from "https://deno.land/x/plugin_prepare@v0.8.0/mod.ts";

const releaseUrl =
  "https://github.com/littledivy/autopilot-deno/releases/download/0.3.2";
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
