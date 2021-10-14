import _inner from "./bind.js";
const inner = _inner as any;

export function isAscii(str: string): boolean {
  return /^[\x00-\x7F]*$/.test(str);
}

export function throwAsciiError() {
  throw new TypeError("String is not a valid ascii.");
}

class AutoPilot {
  /**
   * @param str String to type
   */
  type(str: string) {
    !isAscii(str) && throwAsciiError();
    inner.type(str);
    return this;
  }

  /**
   * @param opt alert options or msg to display alert.
   */
  alert(opt: AlertOptions) {
    inner.alert(opt);
    return this;
  }

  /**
   * @param title The title of the notification
   * @param body The body of the notification
   */
  notify(title: string, body: string) {
    inner.notify({
      title,
      body,
    } as NotificationParams);
    return this;
  }

  /**
   * @return {object} width and height of the screen
   */
  screenSize(): Dimensions {
    return inner.screensize();
  }

  /**
   * @param {number} x The x corrdinate
   * @param {number} y The y corrdinate
   * @param {number} d The speed of mouse
   */

  moveMouse(x: number, y: number, d?: number) {
    if (isNaN(x) || isNaN(y)) {
      throw new TypeError("height or width is NaN");
    }
    inner.smoothMouseMove({ x, y, d });
    return this;
  }

  /**
   * @param {string} file The output file name
   */
  screenshot(file: string) {
    inner.screenshot(file);
    return this;
  }

  /**
   * @param {string} arg The key name
   */
  tap(arg: ToggleKeys) {
    (arg as string) = arg.trim().toLowerCase();
    inner.tap(arg);
    return this;
  }

  /**
   * @param {ClickOptions} arg The mouse section to click
   */
  click(arg: string) {
    inner.mouseClick(arg);
    return this;
  }

  /**
   * @param {ScrollOptions} arg The direction of scroll
   */
  scroll(arg: ScrollOptions) {
    inner.mouseScroll(arg);
    return this;
  }

  /**
   * @return {object} The coordinates of mouse on screen
   */
  mousePosition(): Point {
    return inner.mousePosition();
  }

  /**
   * @return {object} The RGBA color
   */
  pixelColor(): Pixel {
    return inner.mousePixelColor();
  }

  /**
   * @param {string} key The key to be toggled
   * @param {boolean} down Whether to press the key or unpress it
   */
  toggleKey(key: ToggleKeys, down: boolean) {
    inner.toggleKey({
      key,
      down,
    });
    return this;
  }

  /**
   * @return {number} The number of pixels in a point
   */
  screenScale(): number {
    return inner.screenscale();
  }
}

export interface AlertOptions {
  title: string;
  msg: string;
}

export interface NotificationParams {
  title: string;
  body: string;
}

export type ClickOptions = "left" | "right" | "middle";

export type ScrollOptions = "up" | "down";

export type ToggleKeys =
  | "f1"
  | "f2"
  | "f3"
  | "f4"
  | "f5"
  | "f6"
  | "f7"
  | "f8"
  | "f9"
  | "f10"
  | "f11"
  | "f12"
  | "f13"
  | "f14"
  | "f15"
  | "f16"
  | "f17"
  | "f18"
  | "f19"
  | "f20"
  | "f21"
  | "f22"
  | "f23"
  | "f24"
  | "leftarrow"
  | "control"
  | "rightarrow"
  | "downarrow"
  | "end"
  | "uparrow"
  | "pageup"
  | "alt"
  | "return"
  | "pagedown"
  | "delete"
  | "home"
  | "escape"
  | "backspace"
  | "meta"
  | "capslock"
  | "shift"
  | "tab"
  | "space";

interface Point {
  x: number;
  y: number;
}

interface Pixel {
  r: number;
  g: number;
  b: number;
  a: number;
}

interface Dimensions {
  width: number;
  height: number;
}

export default AutoPilot;
