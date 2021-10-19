// Auto-generated with deno_bindgen
import { Plug } from "https://deno.land/x/plug@0.4.0/mod.ts";
function encode(v: string | Uint8Array): Uint8Array {
  if (typeof v !== "string") return v;
  return new TextEncoder().encode(v);
}
const opts = {
  name: "autopilot_deno",
  url: "target/debug",
};
const _lib = await Plug.prepare(opts, {
  mouse_pos_x: { parameters: [], result: "f64", nonblocking: false },
  notify: {
    parameters: ["buffer", "usize"],
    result: "void",
    nonblocking: false,
  },
  mouse_pixel_color_g: { parameters: [], result: "u8", nonblocking: false },
  smooth_mouse_move: {
    parameters: ["buffer", "usize"],
    result: "void",
    nonblocking: false,
  },
  mouse_move: {
    parameters: ["buffer", "usize"],
    result: "void",
    nonblocking: false,
  },
  screensize_height: { parameters: [], result: "f64", nonblocking: false },
  mouse_scroll: { parameters: [], result: "void", nonblocking: false },
  mouse_pixel_color_r: { parameters: [], result: "u8", nonblocking: false },
  mouse_pixel_color_a: { parameters: [], result: "u8", nonblocking: false },
  screensize_width: { parameters: [], result: "f64", nonblocking: false },
  mouse_pixel_color_b: { parameters: [], result: "u8", nonblocking: false },
  tap: { parameters: ["buffer", "usize"], result: "void", nonblocking: false },
  mouse_pos_y: { parameters: [], result: "f64", nonblocking: false },
  alert: {
    parameters: ["buffer", "usize"],
    result: "void",
    nonblocking: false,
  },
  type_: {
    parameters: ["buffer", "usize"],
    result: "void",
    nonblocking: false,
  },
  mouse_click: {
    parameters: ["buffer", "usize"],
    result: "void",
    nonblocking: false,
  },
  screenscale: { parameters: [], result: "f64", nonblocking: false },
  toggle_key: {
    parameters: ["buffer", "usize"],
    result: "void",
    nonblocking: false,
  },
  screenshot: {
    parameters: ["buffer", "usize"],
    result: "void",
    nonblocking: false,
  },
});
export type ScreenSize = {
  height: number;
  width: number;
};
export type Point = {
  x: number;
  y: number;
};
export type NotifyParams = {
  title: string;
  body: string;
};
export type AlertParams = {
  msg: string;
  title: string;
};
export type MouseMoveParams = {
  x: number;
  y: number;
  d: number | undefined | null;
};
export type MouseClickParams =
  | "left"
  | "middle"
  | "right";
export type KeyToggleParams = {
  key: KeyCode;
  down: boolean;
};
export type Pixel = {
  r: number;
  g: number;
  b: number;
  a: number;
};
export type KeyCode =
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
  | "uparrow"
  | "rightarrow"
  | "downarrow"
  | "insert"
  | "delete"
  | "home"
  | "end"
  | "pageup"
  | "pagedown"
  | "tab"
  | "backspace"
  | "enter"
  | "escape"
  | "space"
  | "meta"
  | "alt"
  | "control"
  | "shift"
  | "capslock";
export function mouse_pos_x() {
  return _lib.symbols.mouse_pos_x() as number;
}
export function notify(a0: NotifyParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.notify(a0_buf, a0_buf.byteLength) as null;
}
export function mouse_pixel_color_g() {
  return _lib.symbols.mouse_pixel_color_g() as number;
}
export function smooth_mouse_move(a0: MouseMoveParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.smooth_mouse_move(a0_buf, a0_buf.byteLength) as null;
}
export function mouse_move(a0: MouseMoveParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.mouse_move(a0_buf, a0_buf.byteLength) as null;
}
export function screensize_height() {
  return _lib.symbols.screensize_height() as number;
}
export function mouse_scroll() {
  return _lib.symbols.mouse_scroll() as null;
}
export function mouse_pixel_color_r() {
  return _lib.symbols.mouse_pixel_color_r() as number;
}
export function mouse_pixel_color_a() {
  return _lib.symbols.mouse_pixel_color_a() as number;
}
export function screensize_width() {
  return _lib.symbols.screensize_width() as number;
}
export function mouse_pixel_color_b() {
  return _lib.symbols.mouse_pixel_color_b() as number;
}
export function tap(a0: KeyToggleParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.tap(a0_buf, a0_buf.byteLength) as null;
}
export function mouse_pos_y() {
  return _lib.symbols.mouse_pos_y() as number;
}
export function alert(a0: AlertParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.alert(a0_buf, a0_buf.byteLength) as null;
}
export function type_(a0: string) {
  const a0_buf = encode(a0);
  return _lib.symbols.type_(a0_buf, a0_buf.byteLength) as null;
}
export function mouse_click(a0: MouseClickParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.mouse_click(a0_buf, a0_buf.byteLength) as null;
}
export function screenscale() {
  return _lib.symbols.screenscale() as number;
}
export function toggle_key(a0: KeyToggleParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.toggle_key(a0_buf, a0_buf.byteLength) as null;
}
export function screenshot(a0: string) {
  const a0_buf = encode(a0);
  return _lib.symbols.screenshot(a0_buf, a0_buf.byteLength) as null;
}
