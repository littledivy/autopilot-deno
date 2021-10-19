import {
  alert,
  AlertParams,
  KeyCode,
  mouse_click,
  mouse_pixel_color_a,
  mouse_pixel_color_b,
  mouse_pixel_color_g,
  mouse_pixel_color_r,
  mouse_pos_x,
  mouse_pos_y,
  mouse_scroll,
  MouseClickParams,
  notify,
  screenscale,
  screenshot,
  screensize_height,
  screensize_width,
  smooth_mouse_move,
  tap,
  toggle_key,
  type_,
} from "../bindings/bindings.ts";

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
    type_(text);
    return this;
  }

  /**
   * @param opt alert options or msg to display alert.
   */
  alert(opt: AlertParams) {
    alert(opt);
    return this;
  }

  /**
   * @param title The title of the notification
   * @param body The body of the notification
   */
  notify(title: string, body: string) {
    notify({
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
      width: screensize_width(),
      height: screensize_height(),
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
    smooth_mouse_move({ x, y, d });
    return this;
  }

  /**
   * @param {string} file The output file name
   */
  screenshot(file: string) {
    screenshot(file);
    return this;
  }

  /**
   * @param {string} arg The key name
   */
  tap(arg: KeyCode) {
    (arg as string) = arg.trim().toLowerCase();
    tap({ key: arg, down: false });
    return this;
  }

  /**
   * @param {ClickOptions} arg The mouse section to click
   */
  click(param: MouseClickParams) {
    mouse_click(param);
    return this;
  }

  /**
   * @param {ScrollOptions} arg The direction of scroll
   */
  scroll(arg: string) {
    mouse_scroll();
    return this;
  }

  /**
   * @return {object} The coordinates of mouse on screen
   */
  mousePosition(): Point {
    return { x: mouse_pos_x(), y: mouse_pos_y() };
  }

  /**
   * @return {object} The RGBA color
   */
  pixelColor(): Pixel {
    return {
      r: mouse_pixel_color_r(),
      g: mouse_pixel_color_g(),
      b: mouse_pixel_color_b(),
      a: mouse_pixel_color_a(),
    };
  }

  /**
   * @param {string} key The key to be toggled
   * @param {boolean} down Whether to press the key or unpress it
   */
  toggleKey(key: KeyCode, down: boolean) {
    toggle_key({
      key,
      down,
    });
    return this;
  }

  /**
   * @return {number} The number of pixels in a point
   */
  screenScale(): number {
    return screenscale();
  }
}

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
