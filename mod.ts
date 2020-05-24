// mod.ts
// Copyright 2020 Divy Srivastava
//
// autopilot-deno is a desktop automation module written in rust for deno :)

import {
  runType,
  runAlert,
  runScreenSize,
  runMoveMouse,
  runScreenShot,
  runMouseClick,
  runKeyTap,
  runMouseScroll,
  runMousePosition
} from "./plugin/index.js";

import { AlertOptions, ClickOptions, ScrollOptions } from "./types.ts";

class AutoPilot {
  type(str: string) {
    runType(str);
    return this;
  }
  alert(opt: string | AlertOptions) {
    runAlert(opt);
    return this;
  }
  screenSize() {
    return JSON.parse(runScreenSize());
  }
  moveMouse(x: number, y: number, d?: number) {
    runMoveMouse({ x, y, d });
    return this;
  }
  screenshot(file: string) {
    runScreenShot(file);
    return this;
  }
  tap(arg: string) {
    arg = arg.trim().toLowerCase();
    runKeyTap(arg);
    return this;
  }
  click(arg: ClickOptions) {
    runMouseClick(arg);
    return this;
  }
  scroll(arg: ScrollOptions) {
    runMouseScroll(arg);
    return this;
  }
  mousePostition() {
    return JSON.parse(runMousePosition());
  }
}

export default AutoPilot;
