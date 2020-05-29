// mod.ts
// Copyright 2020 Divy Srivastava
//
// autopilot-deno is a desktop automation module written in rust for deno :)

// Import deno plugin methods

import { logger } from "./deps.ts";

import {
  runType,
  runAlert,
  runScreenSize,
  runMoveMouse,
  runScreenShot,
  runMouseClick,
  runKeyTap,
  runMouseScroll,
  runMousePosition,
  runPixelColor,
  runToggleKey,
  runPointVisible,
  runScreenScale,
  runGetWindow,
  runGetMonitors,
} from "./plugin/index.js";

// Import types
import { AlertOptions, ClickOptions, ScrollOptions } from "./types.ts";

/**
 * Creates an autopilot instance
 */
class AutoPilot {
  constructor() {
    logger.debug("[mod.ts] New AutoPilot instance created");
  }
  /**
   * Simulates keyboard typing
   * executes runType with the str param
   * @param {string} str The string to be typed
   */
  type(str: string) {
    logger.debug("[mod.ts] Running type");
    runType(str);
    return this;
  }
  /**
   * Alert bindings with deno plugin
   * executes runAlert with the opt param
   * @param {string | AlertOptions} opt The arg to be passed
   */
  alert(opt: string | AlertOptions) {
    logger.debug("[mod.ts] Running alert");
    runAlert(opt);
    return this;
  }
  /**
   * Gets the screen size
   * executes runScreenSize and returns a JSON
   * @return {object} width and height of the screen
   */
  screenSize() {
    logger.debug("[mod.ts] Running screenSize");
    return JSON.parse(runScreenSize());
  }
  /**
   * Simulate mouse movement
   * executes runMoveMouse with the given params
   * @param {number} x The x corrdinate
   * @param {number} y The y corrdinate
   * @param {number} d The speed of mouse
   */
  moveMouse(x: number, y: number, d?: number) {
    logger.debug("[mod.ts] Running moveMouse");
    if (isNaN(x) || isNaN(y)) throw "TypeError: height or width is NaN";
    runMoveMouse({ x, y, d });
    return this;
  }
  /**
   * Take a screegrab of the screen
   * executes runScreenShot with the file name and writes file to disk
   * @param {string} file The output file name
   */
  screenshot(file: string) {
    logger.debug("[mod.ts] Running screenshot");
    runScreenShot(file);
    return this;
  }
  /**
   * Simulate key tap
   * executes runKeyTap with the key name
   * @param {string} arg The key name
   */
  tap(arg: string) {
    logger.debug("[mod.ts] Running tap");
    arg = arg.trim().toLowerCase();
    runKeyTap(arg);
    return this;
  }
  /**
   * Simulate mouse click
   * executes runMouseClick with the given arg
   * @param {ClickOptions} arg The mouse section to click
   */
  click(arg: ClickOptions) {
    logger.debug("[mod.ts] Running click");
    runMouseClick(arg);
    return this;
  }
  /**
   * Simulate mouse scroll
   * executes runMouseScroll with the given arg
   * @param {ScrollOptions} arg The direction of scroll
   */
  scroll(arg: ScrollOptions) {
    logger.debug("[mod.ts] Running scroll");
    runMouseScroll(arg);
    return this;
  }
  /**
   * Get mouse position
   * executes runMousePostition and returns a JSON
   * @return {object} The coordinates of mouse on screen
   */
  mousePosition() {
    logger.debug("[mod.ts] Running mousePosition");
    return JSON.parse(runMousePosition());
  }
  /**
   * Get pixel color
   * executes runPixelColor and returns a JSON
   * @return {object} The RGBA color
   */
  pixelColor() {
    logger.debug("[mod.ts] Running pixelColor");
    return JSON.parse(runPixelColor());
  }
  /**
   * Toggle a key
   * executes runToggleKey with given params
   * @param {string} key The key to be toggled
   * @param {boolean} down Whether to press the key or unpress it
   */
  toggleKey(key: string, down: boolean) {
    logger.debug("[mod.ts] Running toggleKey");
    runToggleKey({
      key,
      down: down ? 1 : 0,
    });
    return this;
  }
  /**
   * Checks if point is out of bounds or not
   * executes runPointVisible with given coordinates
   * @param {number} x The x corrdinate
   * @param {number} y The y corrdinate
   * @return {boolean} true if point is visible else false
   */
  pointVisible(x: number, y: number) {
    logger.debug("[mod.ts] Running pointVisible");
    return runPointVisible({
      x,
      y,
    });
  }
  /**
   * Gets the number of pixels in a point.
   * executes runScreenScale and returns the scale
   * @return {number} The number of pixels in a point
   */
  screenScale(): number {
    logger.debug("[mod.ts] Running screenScale");
    return runScreenScale();
  }
  /**
   * Gets the window at 0th index. (needs improvement)
   * executes runGetWindow and returns the window name
   * Works only on Linux
   */
  // **EXPERIMENTAL** (Only for Linux)
  getWindow(index?: number) {
    logger.debug("[mod.ts] Running getWindow");
    return runGetWindow(index || 0);
  }
  /**
   * Gets the number of monitors
   * executes runGetMonitors and returns the nyumber of monitors
   * Works only on Linux
   */
  // **EXPERIMENTAL** (Only for Linux)
  getMonitors(): number {
    logger.debug("[mod.ts] Running getMonitors");
    return parseInt(
      runGetMonitors().split("\n")[0].split("Monitors:").join("").trim(),
    );
  }
}

/**
 * Export the AutoPilot class
 */
export default AutoPilot;
