import { prepare, logger } from "../deps.ts";
import parseMonitorsMac from "../utils/SP_displays_data_type_parser.ts";

const filenameBase = "autopilot_deno";

const PLUGIN_URL_BASE =
  "https://github.com/divy-work/autopilot-deno/releases/latest/download";

const isDev = Deno.env.get("DEV");

if (isDev) {
  logger.info("Running in DEV mode");
  let filenameSuffix = ".so";
  let filenamePrefix = "lib";

  if (Deno.build.os === "windows") {
    filenameSuffix = ".dll";
    filenamePrefix = "";
  }
  if (Deno.build.os === "darwin") {
    filenameSuffix = ".dylib";
  }

  const filename =
    `./target/debug/${filenamePrefix}${filenameBase}${filenameSuffix}`;

  // This will be checked against open resources after Plugin.close()
  // in runTestClose() below.
  const resourcesPre = Deno.resources();

  const rid = Deno.openPlugin(filename);
} else {
  logger.info(`Downloading latest Autopilot release from Github`);
  const pluginId = await prepare({
    name: "autopilot_deno",
    printLog: true,
    checkCache: Boolean(Deno.env.get("CACHE")) || true,
    urls: {
      darwin: `${PLUGIN_URL_BASE}/libautopilot_deno.dylib`,
      windows: `${PLUGIN_URL_BASE}/autopilot_deno.dll`,
      linux: `${PLUGIN_URL_BASE}/libautopilot_deno.so`,
    },
  });
}

logger.info(`Preparing Autopilot for ${Deno.build.os}`);

const {
  type,
  alert,
  click,
  screenSize,
  moveMouse,
  screenshot,
  tap,
  scroll,
  mousePostition,
  pixelColor,
  toggleKey,
  pointVisible,
  screenScale,
  getWindow,
  getMonitors,
  transformByIndex,
  notify,
  quickMoveMouse,
} = Deno.core.ops();

const textDecoder = new TextDecoder();

export function runNotify(arg) {
  arg = JSON.stringify(arg);
  const encoder = new TextEncoder();
  const view = encoder.encode(arg);

  const response = Deno.core.dispatch(notify, view);
}

export function runAlert(arg) {
  let pass = { title: "AutoPilot", msg: "Alert" };
  typeof arg == "object"
    ? (pass = JSON.stringify(arg))
    : ((pass.msg = arg), (pass = JSON.stringify(pass)));
  const encoder = new TextEncoder();
  const view = encoder.encode(pass);

  const response = Deno.core.dispatch(alert, view);
}

export function runMouseClick(arg) {
  const encoder = new TextEncoder();
  const view = encoder.encode(arg);

  const response = Deno.core.dispatch(click, view);
}

export function runTransformByIndex(arg) {
  const encoder = new TextEncoder();
  const view = encoder.encode(JSON.stringify(arg));

  const response = Deno.core.dispatch(transformByIndex, view);
}

export function runGetWindow(arg) {
  if (!arg) arg = "0";
  arg = arg.toString();
  const encoder = new TextEncoder();
  const view = encoder.encode(arg);

  const response = Deno.core.dispatch(getWindow, view);
  return JSON.parse(textDecoder.decode(response)).window;
}

export function runToggleKey(arg) {
  const encoder = new TextEncoder();
  const view = encoder.encode(JSON.stringify(arg));

  const response = Deno.core.dispatch(toggleKey, view);
}

export function runPointVisible(arg) {
  const encoder = new TextEncoder();
  const view = encoder.encode(JSON.stringify(arg));

  const response = Deno.core.dispatch(pointVisible, view);
  return textDecoder.decode(response) == "1" ? true : false;
}

export function runMousePosition() {
  const response = Deno.core.dispatch(mousePostition);
  return textDecoder.decode(response);
}

export function runMouseScroll(arg) {
  const encoder = new TextEncoder();
  const view = encoder.encode(arg);

  const response = Deno.core.dispatch(scroll, view);
}

export function runScreenSize() {
  const response = Deno.core.dispatch(screenSize);
  return textDecoder.decode(response);
}

export function runScreenScale() {
  const response = Deno.core.dispatch(screenScale);
  return JSON.parse(textDecoder.decode(response)).scale;
}

export function runGetMonitors() {
  const response = Deno.core.dispatch(getMonitors);
  if (Deno.build.os === "darwin") {
    return parseMonitorsMac(JSON.parse(textDecoder.decode(response)).monitors);
  }
  return JSON.parse(textDecoder.decode(response)).monitors;
}

export function runPixelColor() {
  const response = Deno.core.dispatch(pixelColor);
  return textDecoder.decode(response);
}

export function runKeyTap(arg) {
  const encoder = new TextEncoder();
  const view = encoder.encode(arg);

  const response = Deno.core.dispatch(tap, view);
  return textDecoder.decode(response);
}

export function runScreenShot(arg) {
  const encoder = new TextEncoder();
  const view = encoder.encode(arg);

  const response = Deno.core.dispatch(screenshot, view);
  return textDecoder.decode(response);
}

export function runMoveMouse(arg) {
  let sarg = JSON.stringify(arg);
  const encoder = new TextEncoder();
  const view = encoder.encode(sarg);
  console.log(arg.d)
  const response = Deno.core.dispatch(arg.d ? moveMouse : quickMoveMouse, view);
  return textDecoder.decode(response);
}

export function runType(arg) {
  const encoder = new TextEncoder();
  const view = encoder.encode(arg);

  const response = Deno.core.dispatch(type, view);
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
logger.info(`Autopilot setup complete`);
