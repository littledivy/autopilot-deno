const filenameBase = "autopilot_deno";

let filenameSuffix = ".so";
let filenamePrefix = "lib";

if (Deno.build.os === "windows") {
  filenameSuffix = ".dll";
  filenamePrefix = "";
}
if (Deno.build.os === "darwin") {
  filenameSuffix = ".dylib";
}

const filename = `./target/debug/${filenamePrefix}${filenameBase}${filenameSuffix}`;

// This will be checked against open resources after Plugin.close()
// in runTestClose() below.
const resourcesPre = Deno.resources();

const rid = Deno.openPlugin(filename);

const { type } = Deno.core.ops();

const textDecoder = new TextDecoder();

export function runType(arg) {
  const encoder = new TextEncoder()
  const view = encoder.encode(arg)

  const response = Deno.core.dispatch(
    type,
    view
  );
}

Deno.core.setAsyncHandler(type, (response) => {
  // leave this blank
});
