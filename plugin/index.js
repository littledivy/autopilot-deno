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

const { type, alert, screenSize, moveMouse } = Deno.core.ops();

const textDecoder = new TextDecoder();

export function runAlert(arg) {
  const encoder = new TextEncoder()
  const view = encoder.encode(arg)

  const response = Deno.core.dispatch(
    alert,
    view
  );
}

export function runScreenSize() {
  const response = Deno.core.dispatch(
    screenSize
  );
  return textDecoder.decode(response);
}

export function runMoveMouse(arg) {
  arg = JSON.stringify(arg)
  const encoder = new TextEncoder()
  const view = encoder.encode(arg)

  const response = Deno.core.dispatch(
    moveMouse,
    view
  );
  return textDecoder.decode(response);
}

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

Deno.core.setAsyncHandler(moveMouse, (response) => {
  // leave this blank
});

Deno.core.setAsyncHandler(screenSize, (response) => {
  // leave this blank
});

Deno.core.setAsyncHandler(alert, (response) => {
  // leave this blank
});
