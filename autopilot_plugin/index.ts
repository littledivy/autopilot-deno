// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

import { prepare, logger } from "../deps.ts";
import parseMonitorsMac from "../autopilot_deno/utils/SP_displays_data_type_parser.ts";
import parseMonitorsWin from "../autopilot_deno/utils/wmic_data_parser.ts";
import { getMonitors as getMonitorsFallback } from "../autopilot_deno/monitors.ts";

import filename from "./detect.ts";
import config from "../plugin_config.ts";
import { core } from "../autopilot_deno/types.ts";

const { filenameBase, pluginBase } = config;

const isDev = Deno.env.get("DEV");

if (isDev) {
  logger.info("Running in DEV mode");

  // This will be checked against open resources after Plugin.close()
  // in runTestClose() below.
  const resourcesPre = Deno.resources();

  const rid = Deno.openPlugin("./target/debug/" + filename(filenameBase));
} else {
  // logger.info(`Downloading latest Autopilot release from Github`);
  const pluginId = await prepare({
    name: "autopilot_deno",
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
const decoder = new TextDecoder();

export async function runNotify(arg: object) {
  let sarg = JSON.stringify(arg);
  const encoder = new TextEncoder();
  const view = encoder.encode(sarg);
  return new Promise((resolve, reject) => {
    core.setAsyncHandler(notify, (bytes) => {
      resolve(textDecoder.decode(bytes));
    });
    core.dispatch(notify, view);
  });
}

export async function runAlert(arg: object | string) {
  let pass: any = { title: "AutoPilot", msg: "Alert" };
  typeof arg == "object"
    ? (pass = JSON.stringify(arg))
    : ((pass.msg = arg), (pass = JSON.stringify(pass)));
  const encoder = new TextEncoder();
  const view = encoder.encode(pass);
  return new Promise((resolve, reject) => {
    core.setAsyncHandler(alert, (bytes) => {
      resolve(textDecoder.decode(bytes));
    });
    core.dispatch(alert, view);
  });
}

export async function runMouseClick(arg: string) {
  const encoder = new TextEncoder();
  const view = encoder.encode(arg);
  return new Promise((resolve, reject) => {
    core.setAsyncHandler(click, (bytes) => {
      resolve(textDecoder.decode(bytes));
    });
    core.dispatch(click, view);
  });
}

export async function runTransformByIndex(arg: object) {
  const encoder = new TextEncoder();
  const view = encoder.encode(JSON.stringify(arg));
  return new Promise((resolve, reject) => {
    core.setAsyncHandler(transformByIndex, (bytes) => {
      resolve(textDecoder.decode(bytes));
    });
    core.dispatch(transformByIndex, view);
  });
}

export async function runGetWindow(arg: number = 0) {
  let i = arg.toString();
  const encoder = new TextEncoder();
  const view = encoder.encode(i);
  return new Promise((resolve, reject) => {
    core.setAsyncHandler(getWindow, (bytes) => {
      resolve(JSON.parse(textDecoder.decode(bytes)).window);
    });
    core.dispatch(getWindow, view);
  });
}

export async function runToggleKey(arg: object) {
  const encoder = new TextEncoder();
  const view = encoder.encode(JSON.stringify(arg));
  return new Promise((resolve, reject) => {
    core.setAsyncHandler(toggleKey, (bytes) => {
      resolve(textDecoder.decode(bytes));
    });
    core.dispatch(toggleKey, view);
  });
}

export async function runPointVisible(arg: object) {
  const encoder = new TextEncoder();
  const view = encoder.encode(JSON.stringify(arg));
  return new Promise((resolve, reject) => {
    core.setAsyncHandler(pointVisible, (bytes) => {
      resolve(textDecoder.decode(bytes) == "1" ? true : false);
    });
    core.dispatch(pointVisible, view);
  });
}

export async function runMousePosition(): Promise<string> {
  return new Promise((resolve, reject) => {
    core.setAsyncHandler(mousePostition, (bytes) => {
      resolve(textDecoder.decode(bytes));
    });
    core.dispatch(mousePostition);
  });
}

export async function runMouseScroll(arg: string) {
  const encoder = new TextEncoder();
  const view = encoder.encode(arg);
  return new Promise((resolve, reject) => {
    core.setAsyncHandler(scroll, (bytes) => {
      resolve(textDecoder.decode(bytes));
    });
    core.dispatch(scroll, view);
  });
}

export async function runScreenSize(): Promise<string> {
  return new Promise((resolve, reject) => {
    core.setAsyncHandler(screenSize, (bytes) => {
      resolve(textDecoder.decode(bytes));
    });
    core.dispatch(screenSize);
  });
}

export async function runScreenScale(): Promise<number> {
  return new Promise((resolve, reject) => {
    core.setAsyncHandler(screenScale, (bytes) => {
      let response = JSON.parse(decoder.decode(bytes));
      resolve(response.scale);
    });
    core.dispatch(screenScale);
  });
}

export async function runGetMonitors(): Promise<string> {
  return new Promise((resolve, reject) => {
    core.setAsyncHandler(getMonitors, (response) => {
      let res = textDecoder.decode(response);
      if (Deno.build.os === "windows") {
        resolve(parseMonitorsWin(res));
      }
      if (Deno.build.os === "darwin") {
        resolve(parseMonitorsMac(JSON.parse(res).monitors));
      }
      resolve(textDecoder.decode(response));
    });
    core.dispatch(getMonitors);
  });
}

export async function runPixelColor(): Promise<string> {
  return new Promise((resolve, reject) => {
    core.setAsyncHandler(pixelColor, (bytes) => {
      let response = JSON.parse(decoder.decode(bytes));
      resolve(response);
    });
    core.dispatch(pixelColor);
  });
}

export async function runKeyTap(arg: string) {
  const encoder = new TextEncoder();
  const view = encoder.encode(arg);
  return new Promise((resolve, reject) => {
    core.setAsyncHandler(tap, (bytes) => {
      let response = JSON.parse(decoder.decode(bytes));
      resolve(response);
    });
    core.dispatch(tap, view);
  });
}

export async function runScreenShot(arg: string) {
  const encoder = new TextEncoder();
  const view = encoder.encode(arg);
  return new Promise((resolve, reject) => {
    core.setAsyncHandler(screenshot, (bytes) => {
      let response = JSON.parse(decoder.decode(bytes));
      resolve(response);
    });
    core.dispatch(screenshot, view);
  });
}

export async function runMoveMouse(arg: any) {
  let sarg = JSON.stringify(arg);
  const encoder = new TextEncoder();
  const view = encoder.encode(sarg);
  return new Promise((resolve, reject) => {
    core.setAsyncHandler(arg.d ? moveMouse : quickMoveMouse, (bytes) => {
      let response = JSON.parse(decoder.decode(bytes));
      resolve(response);
    });
    core.dispatch(arg.d ? moveMouse : quickMoveMouse, view);
  });
}

export async function runType(arg: string) {
  const encoder = new TextEncoder();
  const view = encoder.encode(arg);
  return new Promise((resolve, reject) => {
    core.setAsyncHandler(type, (bytes) => {
      let response = JSON.parse(decoder.decode(bytes));
      resolve(response);
    });
    core.dispatch(type, view);
  });
}

logger.info(`Autopilot setup complete`);
