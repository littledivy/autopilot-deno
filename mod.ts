// mod.ts
// Copyright 2020 Divy Srivastava
//
// autopilot-deno is a desktop automation module written in rust for deno :)

// Import deno plugin methods
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
  runPointVisible
} from "./plugin/index.js";

// Import types
import { AlertOptions, ClickOptions, ScrollOptions } from "./types.ts";

/**
 * Creates an autopilot instance
 */
class AutoPilot {
  /**
   * Simulates keyboard typing
   * executes runType with the str param
   * @param {string} str The string to be typed
   */
  type(str: string) {
    runType(str);
    return this;
  }
  /**
   * Alert bindings with deno plugin
   * executes runAlert with the opt param
   * @param {string | AlertOptions} opt The arg to be passed
   */
  alert(opt: string | AlertOptions) {
    runAlert(opt);
    return this;
  }
  /**
   * Gets the screen size
   * executes runScreenSize and returns a JSON
   * @return {object} width and height of the screen
   */
  screenSize() {
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
    runScreenShot(file);
    return this;
  }
  /**
   * Simulate key tap
   * executes runKeyTap with the key name
   * @param {string} arg The key name
   */
  tap(arg: string) {
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
    runMouseClick(arg);
    return this;
  }
  /**
   * Simulate mouse scroll
   * executes runMouseScroll with the given arg
   * @param {ScrollOptions} arg The direction of scroll
   */
  scroll(arg: ScrollOptions) {
    runMouseScroll(arg);
    return this;
  }
  /**
   * Get mouse position
   * executes runMousePostition and returns a JSON
   * @return {object} The coordinates of mouse on screen
   */
  mousePosition() {
    return JSON.parse(runMousePosition());
  }
  /**
   * Get pixel color
   * executes runPixelColor and returns a JSON
   * @return {object} The RGBA color
   */
  pixelColor() {
    return JSON.parse(runPixelColor());
  }
  /**
   * Toggle a key
   * executes runToggleKey with given params
   * @param {string} key The key to be toggled
   * @param {boolean} down Whether to press the key or unpress it
   */
  toggleKey(key: string, down: boolean) {
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
  pointVisible(x:number, y:number) {
    return runPointVisible({
      x,
      y
    });
  }
}

/**
 * Export the AutoPilot class
 */
export default AutoPilot;
