// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

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
  runTransformByIndex,
  runGetMonitors,
  runNotify,
} from "../autopilot_plugin/index.ts";

// Import types
import {
  AlertOptions,
  ClickOptions,
  ScrollOptions,
  ToggleKeys,
  NotificationParams,
} from "./types.ts";

import {
  isAscii,
  throwAsciiError,
} from "./utils/isAscii.ts";

import { logger } from "../deps.ts";

/**
 * Creates an autopilot instance
 */
class AutoPilot {
  constructor() {
    logger.info("[mod.ts] New AutoPilot instance created");
  }
  /**
   * Types a string.
   *
   * ```typescript
   * import AutoPilot from "https://deno.land/x/autopilot/mod.ts";
   * const pilot = new AutoPilot();
   * pilot.type("Hello, World!");
   * ```
   *
   * @param str String to type
   */
  async type(str: string) {
    !isAscii(str) && throwAsciiError();
    logger.info("[mod.ts] Running type");
    await runType(str);
    return this;
  }
  /**
   * Displays an alert.
   *
   * ```typescript
   * import AutoPilot from "https://deno.land/x/autopilot/mod.ts";
   * const pilot = new AutoPilot();
   * pilot.alert("Hello, World!");
   * ```
   *
   * @param opt alert options or msg to display alert.
   */
  async alert(opt: string | AlertOptions) {
    logger.info("[mod.ts] Running alert");
    await runAlert(opt);
    return this;
  }
  /**
   * Triggers a system notification.
   *
   * ```typescript
   * import AutoPilot from "https://deno.land/x/autopilot/mod.ts";
   * const pilot = new AutoPilot();
   * pilot.notify("Hello", "World!");
   * ```
   *
   * @param title The title of the notification
   * @param body The body of the notification
   */
  async notify(title: string, body: string) {
    logger.info("Running notify");
    await runNotify(
      {
        title,
        body,
      } as NotificationParams,
    );
    return this;
  }
  /**
   * Gets the screen size
   * executes runScreenSize and returns a JSON
   * @return {object} width and height of the screen
   */
  async screenSize() {
    logger.info("[mod.ts] Running screenSize");
    return JSON.parse(await runScreenSize());
  }
  /**
   * Simulate mouse movement
   * executes runMoveMouse with the given params
   * @param {number} x The x corrdinate
   * @param {number} y The y corrdinate
   * @param {number} d The speed of mouse
   */
  async moveMouse(x: number, y: number, d?: number) {
    logger.info("[mod.ts] Running moveMouse");
    if (isNaN(x) || isNaN(y)) {
      console.error("TypeError: height or width is NaN");
    }
    await runMoveMouse({ x, y, d });
    return this;
  }
  /**
   * Take a screegrab of the screen
   * executes runScreenShot with the file name and writes file to disk
   * @param {string} file The output file name
   */
  async screenshot(file: string) {
    logger.info("[mod.ts] Running screenshot");
    await runScreenShot(file);
    return this;
  }
  /**
   * Simulate key tap
   * executes runKeyTap with the key name
   * @param {string} arg The key name
   */
  async tap(arg: string) {
    logger.info("[mod.ts] Running tap");
    arg = arg.trim().toLowerCase();
    await runKeyTap(arg as ToggleKeys);
    return this;
  }
  /**
   * Simulate mouse click
   * executes runMouseClick with the given arg
   * @param {ClickOptions} arg The mouse section to click
   */
  async click(arg: ClickOptions) {
    logger.info("[mod.ts] Running click");
    await runMouseClick(arg);
    return this;
  }
  /**
   * Simulate mouse scroll
   * executes runMouseScroll with the given arg
   * @param {ScrollOptions} arg The direction of scroll
   */
  async scroll(arg: ScrollOptions) {
    logger.info("[mod.ts] Running scroll");
    await runMouseScroll(arg);
    return this;
  }
  /**
   * Get mouse position
   * executes runMousePostition and returns a JSON
   * @return {object} The coordinates of mouse on screen
   */
  async mousePosition() {
    logger.info("[mod.ts] Running mousePosition");
    return JSON.parse(await runMousePosition());
  }
  /**
   * Get pixel color
   * executes runPixelColor and returns a JSON
   * @return {object} The RGBA color
   */
  async pixelColor() {
    await logger.info("[mod.ts] Running pixelColor");
    return await runPixelColor();
  }
  /**
   * Toggle a key
   * executes runToggleKey with given params
   * @param {string} key The key to be toggled
   * @param {boolean} down Whether to press the key or unpress it
   */
  async toggleKey(key: ToggleKeys, down: boolean) {
    logger.info("[mod.ts] Running toggleKey");
    await runToggleKey({
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
  async pointVisible(x: number, y: number) {
    logger.info("[mod.ts] Running pointVisible");
    return await runPointVisible({
      x,
      y,
    });
  }
  /**
   * Gets the number of pixels in a point.
   * executes runScreenScale and returns the scale
   * @return {number} The number of pixels in a point
   */
  async screenScale(): Promise<number> {
    logger.info("[mod.ts] Running screenScale");
    return await runScreenScale();
  }
  /**
   * Gets the number of monitors
   * executes runGetMonitors and returns the nyumber of monitors
   * @return {Promise<number>} The number of monitors
   */
  async getMonitors(): Promise<number> {
    logger.info("[mod.ts] Running getMonitors");
    let n = await runGetMonitors();
    return parseInt(n.split("\n")[0].split("Monitors:").join("").trim());
  }
  /**
   * Gets the window at 0th index. (needs improvement)
   * executes runGetWindow and returns the window name
   * Works only on Linux
   */
  // **EXPERIMENTAL** (Only for Linux)
  async getWindow(index?: number) {
    logger.info("[mod.ts] Running getWindow");
    return await runGetWindow(index || 0);
  }
  /**
   * Transform a window by index
   * executes runTransformByIndex
   * Works only on Linux
   */
  // **EXPERIMENTAL** (Only for Linux)
  async transformByIndex(index: number, width: number, height: number) {
    logger.info("[mod.ts] Running transformByIndex");
    await runTransformByIndex({
      index,
      width,
      height,
    });
    return this;
  }
}

/**
 * Export the AutoPilot class
 */
export default AutoPilot;
