// Auto-generated with deno_bindgen

const encode = (s: string) => new TextEncoder().encode(s);
const _lib = Deno.dlopen("target/debug/libautopilot_deno.so", {
  alert: { parameters: ["buffer", "usize"], result: "void" },
  screenshot: { parameters: ["buffer", "usize"], result: "void" },
  mouse_move: { parameters: ["buffer", "usize"], result: "void" },
  screenscale: { parameters: [], result: "f64" },
  mouse_scroll: { parameters: [], result: "void" },
  mouse_pixel_color_a: { parameters: [], result: "u8" },
  mouse_click: { parameters: ["buffer", "usize"], result: "void" },
  mouse_pos_y: { parameters: [], result: "f64" },
  type_: { parameters: ["buffer", "usize"], result: "void" },
  notify: { parameters: ["buffer", "usize"], result: "void" },
  mouse_pos_x: { parameters: [], result: "f64" },
  toggle_key: { parameters: ["buffer", "usize"], result: "void" },
  mouse_pixel_color_r: { parameters: [], result: "u8" },
  tap: { parameters: ["buffer", "usize"], result: "void" },
  screensize_height: { parameters: [], result: "f64" },
  screensize_width: { parameters: [], result: "f64" },
  smooth_mouse_move: { parameters: ["buffer", "usize"], result: "void" },
  mouse_pixel_color_g: { parameters: [], result: "u8" },
  mouse_pixel_color_b: { parameters: [], result: "u8" },
});
type MouseMoveParams = { x: number; y: number; d: any };
type StrArg = { text: any };
type AlertParams = { msg: any; title: any };
type ScreenSize = { height: number; width: number };
type Pixel = { a: number; b: number; g: number; r: number };
type KeyToggleParams = { down: any; key: any };
type NotifyParams = { body: any; title: any };
type MouseClickParamsWrapper = { params: any };
type Point = { y: number; x: number };
export function alert(a0: AlertParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.alert(a0_buf, a0_buf.byteLength) as null;
}
export function screenshot(a0: StrArg) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.screenshot(a0_buf, a0_buf.byteLength) as null;
}
export function mouse_move(a0: MouseMoveParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.mouse_move(a0_buf, a0_buf.byteLength) as null;
}
export function screenscale() {
  return _lib.symbols.screenscale() as number;
}
export function mouse_scroll() {
  return _lib.symbols.mouse_scroll() as null;
}
export function mouse_pixel_color_a() {
  return _lib.symbols.mouse_pixel_color_a() as number;
}
export function mouse_click(a0: MouseClickParamsWrapper) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.mouse_click(a0_buf, a0_buf.byteLength) as null;
}
export function mouse_pos_y() {
  return _lib.symbols.mouse_pos_y() as number;
}
export function type_(a0: StrArg) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.type_(a0_buf, a0_buf.byteLength) as null;
}
export function notify(a0: NotifyParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.notify(a0_buf, a0_buf.byteLength) as null;
}
export function mouse_pos_x() {
  return _lib.symbols.mouse_pos_x() as number;
}
export function toggle_key(a0: KeyToggleParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.toggle_key(a0_buf, a0_buf.byteLength) as null;
}
export function mouse_pixel_color_r() {
  return _lib.symbols.mouse_pixel_color_r() as number;
}
export function tap(a0: KeyToggleParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.tap(a0_buf, a0_buf.byteLength) as null;
}
export function screensize_height() {
  return _lib.symbols.screensize_height() as number;
}
export function screensize_width() {
  return _lib.symbols.screensize_width() as number;
}
export function smooth_mouse_move(a0: MouseMoveParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.smooth_mouse_move(a0_buf, a0_buf.byteLength) as null;
}
export function mouse_pixel_color_g() {
  return _lib.symbols.mouse_pixel_color_g() as number;
}
export function mouse_pixel_color_b() {
  return _lib.symbols.mouse_pixel_color_b() as number;
}
