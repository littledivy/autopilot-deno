import * as inner from "../bindings/bindings.ts";

export function isAscii(str: string): boolean {
  return /^[\x00-\x7F]*$/.test(str);
}

export function throwAsciiError() {
  throw new TypeError("String is not a valid ascii.");
}

class AutoPilot {
  /**
   * @param text String to type
   */
  type(text: string) {
    !isAscii(text) && throwAsciiError();
    inner.type_({ text });
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
    });
    return this;
  }

  /**
   * @return {object} width and height of the screen
   */
  screenSize(): Dimensions {
    return {
      width: inner.screensize_width(),
      height: inner.screensize_height(),
    };
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
    inner.smooth_mouse_move({ x, y, d });
    return this;
  }

  /**
   * @param {string} file The output file name
   */
  screenshot(file: string) {
    inner.screenshot({ text: file });
    return this;
  }

  /**
   * @param {string} arg The key name
   */
  tap(arg: ToggleKeys) {
    (arg as string) = arg.trim().toLowerCase();
    inner.tap({ key: arg, down: false });
    return this;
  }

  /**
   * @param {ClickOptions} arg The mouse section to click
   */
  click(arg: string) {
    inner.mouse_click({ params: arg });
    return this;
  }

  /**
   * @param {ScrollOptions} arg The direction of scroll
   */
  scroll(arg: ScrollOptions) {
    inner.mouse_scroll();
    return this;
  }

  /**
   * @return {object} The coordinates of mouse on screen
   */
  mousePosition(): Point {
    return { x: inner.mouse_pos_x(), y: inner.mouse_pos_y() };
  }

  /**
   * @return {object} The RGBA color
   */
  pixelColor(): Pixel {
    return {
      r: inner.mouse_pixel_color_r(),
      g: inner.mouse_pixel_color_g(),
      b: inner.mouse_pixel_color_b(),
      a: inner.mouse_pixel_color_a(),
    };
  }

  /**
   * @param {string} key The key to be toggled
   * @param {boolean} down Whether to press the key or unpress it
   */
  toggleKey(key: ToggleKeys, down: boolean) {
    inner.toggle_key({
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
