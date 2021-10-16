// Auto-generated with deno_bindgen
import { Plug } from "https://deno.land/x/plug@0.4.0/mod.ts";
const encode = (s: string) => new TextEncoder().encode(s);
const opts = {
  name: "autopilot_deno",
  url: "target/debug",
};
const _lib = await Plug.prepare(opts, {
  mouse_pos_x: { parameters: [], result: "f64" },
  mouse_move: { parameters: ["buffer", "usize"], result: "void" },
  smooth_mouse_move: { parameters: ["buffer", "usize"], result: "void" },
  mouse_pixel_color_b: { parameters: [], result: "u8" },
  notify: { parameters: ["buffer", "usize"], result: "void" },
  type_: { parameters: ["buffer", "usize"], result: "void" },
  tap: { parameters: ["buffer", "usize"], result: "void" },
  mouse_pos_y: { parameters: [], result: "f64" },
  screensize_width: { parameters: [], result: "f64" },
  screenscale: { parameters: [], result: "f64" },
  screensize_height: { parameters: [], result: "f64" },
  alert: { parameters: ["buffer", "usize"], result: "void" },
  mouse_pixel_color_a: { parameters: [], result: "u8" },
  mouse_pixel_color_r: { parameters: [], result: "u8" },
  screenshot: { parameters: ["buffer", "usize"], result: "void" },
  mouse_click: { parameters: ["buffer", "usize"], result: "void" },
  mouse_scroll: { parameters: [], result: "void" },
  mouse_pixel_color_g: { parameters: [], result: "u8" },
  toggle_key: { parameters: ["buffer", "usize"], result: "void" },
});
type AlertParams = { title: any; msg: any };
type KeyToggleParams = { key: any; down: any };
type MouseClickParamsWrapper = { params: any };
type Point = { x: number; y: number };
type MouseMoveParams = { y: number; d: any; x: number };
type StrArg = { text: any };
type Pixel = { g: number; b: number; a: number; r: number };
type NotifyParams = { title: any; body: any };
type ScreenSize = { width: number; height: number };
export function mouse_pos_x() {
  return _lib.symbols.mouse_pos_x() as number;
}
export function mouse_move(a0: MouseMoveParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.mouse_move(a0_buf, a0_buf.byteLength) as null;
}
export function smooth_mouse_move(a0: MouseMoveParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.smooth_mouse_move(a0_buf, a0_buf.byteLength) as null;
}
export function mouse_pixel_color_b() {
  return _lib.symbols.mouse_pixel_color_b() as number;
}
export function notify(a0: NotifyParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.notify(a0_buf, a0_buf.byteLength) as null;
}
export function type_(a0: StrArg) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.type_(a0_buf, a0_buf.byteLength) as null;
}
export function tap(a0: KeyToggleParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.tap(a0_buf, a0_buf.byteLength) as null;
}
export function mouse_pos_y() {
  return _lib.symbols.mouse_pos_y() as number;
}
export function screensize_width() {
  return _lib.symbols.screensize_width() as number;
}
export function screenscale() {
  return _lib.symbols.screenscale() as number;
}
export function screensize_height() {
  return _lib.symbols.screensize_height() as number;
}
export function alert(a0: AlertParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.alert(a0_buf, a0_buf.byteLength) as null;
}
export function mouse_pixel_color_a() {
  return _lib.symbols.mouse_pixel_color_a() as number;
}
export function mouse_pixel_color_r() {
  return _lib.symbols.mouse_pixel_color_r() as number;
}
export function screenshot(a0: StrArg) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.screenshot(a0_buf, a0_buf.byteLength) as null;
}
export function mouse_click(a0: MouseClickParamsWrapper) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.mouse_click(a0_buf, a0_buf.byteLength) as null;
}
export function mouse_scroll() {
  return _lib.symbols.mouse_scroll() as null;
}
export function mouse_pixel_color_g() {
  return _lib.symbols.mouse_pixel_color_g() as number;
}
export function toggle_key(a0: KeyToggleParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.toggle_key(a0_buf, a0_buf.byteLength) as null;
}
