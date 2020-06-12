// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

import { prepare, logger } from "../deps.ts";
import parseMonitorsMac from "../utils/SP_displays_data_type_parser.ts";
import parseMonitorsWin from "../utils/wmic_data_parser.ts";
import {getMonitors as getMonitorsFallback} from "../lib/monitors.ts";

import filename from "./detect.ts";
import config from "../plugin_config.ts";
import { core } from "../types.ts";

const { filenameBase, pluginBase } = config;

const isDev = Deno.env.get("DEV");

if (isDev) {
  logger.info("Running in DEV mode");

  // This will be checked against open resources after Plugin.close()
  // in runTestClose() below.
  const resourcesPre = Deno.resources();

  const rid = Deno.openPlugin("./target/debug/" + filename(filenameBase));
} else {
  logger.info(`Downloading latest Autopilot release from Github`);
  const pluginId = await prepare({
    name: "autopilot_deno",
    printLog: true,
    checkCache: Deno.env.get("CACHE") ? false : true,
    urls: {
      darwin: `${pluginBase}/libautopilot_deno.dylib`,
      windows: `${pluginBase}/autopilot_deno.dll`,
      linux: `${pluginBase}/libautopilot_deno.so`,
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
} = core.ops();

const textDecoder = new TextDecoder();

export function runNotify(arg: object) {
  let sarg = JSON.stringify(arg);
  const encoder = new TextEncoder();
  const view = encoder.encode(sarg);

  const response = core.dispatch(notify, view);
}

export function runAlert(arg: object | string) {
  let pass: any = { title: "AutoPilot", msg: "Alert" };
  typeof arg == "object"
    ? (pass = JSON.stringify(arg))
    : ((pass.msg = arg), (pass = JSON.stringify(pass)));
  const encoder = new TextEncoder();
  const view = encoder.encode(pass);

  const response = core.dispatch(alert, view);
}

export function runMouseClick(arg: string) {
  const encoder = new TextEncoder();
  const view = encoder.encode(arg);

  const response = core.dispatch(click, view);
}

export function runTransformByIndex(arg: object) {
  const encoder = new TextEncoder();
  const view = encoder.encode(JSON.stringify(arg));

  const response = core.dispatch(transformByIndex, view);
}

export function runGetWindow(arg: number = 0) {
  let i = arg.toString();
  const encoder = new TextEncoder();
  const view = encoder.encode(i);

  const response = core.dispatch(getWindow, view);
  return JSON.parse(textDecoder.decode(response)).window;
}

export function runToggleKey(arg: object) {
  const encoder = new TextEncoder();
  const view = encoder.encode(JSON.stringify(arg));

  const response = core.dispatch(toggleKey, view);
}

export function runPointVisible(arg: object) {
  const encoder = new TextEncoder();
  const view = encoder.encode(JSON.stringify(arg));

  const response = core.dispatch(pointVisible, view);
  return textDecoder.decode(response) == "1" ? true : false;
}

export function runMousePosition() {
  const response = core.dispatch(mousePostition);
  return textDecoder.decode(response);
}

export function runMouseScroll(arg: string) {
  const encoder = new TextEncoder();
  const view = encoder.encode(arg);

  const response = core.dispatch(scroll, view);
}

export function runScreenSize() {
  const response = core.dispatch(screenSize);
  return textDecoder.decode(response);
}

export function runScreenScale() {
  const response = core.dispatch(screenScale);
  return JSON.parse(textDecoder.decode(response)).scale;
}

export async function runGetMonitors() {
  if (Deno.build.os === "windows") {
    return await parseMonitorsWin(getMonitorsFallback());
  }
  const response = core.dispatch(getMonitors);
  if (Deno.build.os === "darwin") {
    return new Promise((resolve, reject) => {
      resolve(parseMonitorsMac(JSON.parse(textDecoder.decode(response)).monitors));
    })
  }
  return JSON.parse(textDecoder.decode(response)).monitors;
}

export function runPixelColor() {
  const response = core.dispatch(pixelColor);
  return textDecoder.decode(response);
}

export function runKeyTap(arg: string) {
  const encoder = new TextEncoder();
  const view = encoder.encode(arg);

  const response = core.dispatch(tap, view);
  return textDecoder.decode(response);
}

export function runScreenShot(arg: string) {
  const encoder = new TextEncoder();
  const view = encoder.encode(arg);

  const response = core.dispatch(screenshot, view);
  return textDecoder.decode(response);
}

export function runMoveMouse(arg: any) {
  let sarg = JSON.stringify(arg);
  const encoder = new TextEncoder();
  const view = encoder.encode(sarg);
  const response = core.dispatch(arg.d ? moveMouse : quickMoveMouse, view);
  return textDecoder.decode(response);
}

export function runType(arg: string) {
  const encoder = new TextEncoder();
  const view = encoder.encode(arg);

  const response = core.dispatch(type, view);
}

core.setAsyncHandler(type, () => {
  // leave this blank
});

core.setAsyncHandler(moveMouse, () => {
  // leave this blank
});

core.setAsyncHandler(screenSize, () => {
  // leave this blank
});

core.setAsyncHandler(alert, () => {
  // leave this blank
});

logger.info(`Autopilot setup complete`);
