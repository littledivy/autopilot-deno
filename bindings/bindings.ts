// Auto-generated with deno_bindgen

const encode = (s: string) => new TextEncoder().encode(s);
const _lib = Deno.dlopen("target/debug/libautopilot_deno.so", {
  op_mouse_pixel_color_b: { parameters: [], result: "u8" },
  op_screensize_width: { parameters: [], result: "f64" },
  op_mouse_pixel_color_g: { parameters: [], result: "u8" },
  op_mouse_pixel_color_r: { parameters: [], result: "u8" },
  op_mouse_click: { parameters: ["buffer", "usize"], result: "void" },
  op_smooth_mouse_move: { parameters: ["buffer", "usize"], result: "void" },
  op_mouse_move: { parameters: ["buffer", "usize"], result: "void" },
  op_alert: { parameters: ["buffer", "usize"], result: "void" },
  op_mouse_pos_y: { parameters: [], result: "f64" },
  op_mouse_pos_x: { parameters: [], result: "f64" },
  op_mouse_scroll: { parameters: [], result: "void" },
  op_screenshot: { parameters: ["buffer", "usize"], result: "void" },
  op_type: { parameters: ["buffer", "usize"], result: "void" },
  op_screensize_height: { parameters: [], result: "f64" },
  op_notify: { parameters: ["buffer", "usize"], result: "void" },
  op_toggle_key: { parameters: ["buffer", "usize"], result: "void" },
  op_tap: { parameters: ["buffer", "usize"], result: "void" },
  op_mouse_pixel_color_a: { parameters: [], result: "u8" },
});
type Pixel = { r: number; a: number; g: number; b: number };
type ScreenSize = { width: number; height: number };
type MouseClickParamsWrapper = { params: any };
type AlertParams = { title: any; msg: any };
type KeyToggleParams = { down: any; key: any };
type MouseMoveParams = { d: any; x: number; y: number };
type NotifyParams = { body: any; title: any };
type StrArg = { text: any };
type Point = { x: number; y: number };
export function op_mouse_pixel_color_b() {
  return _lib.symbols.op_mouse_pixel_color_b() as number;
}
export function op_screensize_width() {
  return _lib.symbols.op_screensize_width() as number;
}
export function op_mouse_pixel_color_g() {
  return _lib.symbols.op_mouse_pixel_color_g() as number;
}
export function op_mouse_pixel_color_r() {
  return _lib.symbols.op_mouse_pixel_color_r() as number;
}
export function op_mouse_click(a0: MouseClickParamsWrapper) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.op_mouse_click(a0_buf, a0_buf.byteLength) as null;
}
export function op_smooth_mouse_move(a0: MouseMoveParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.op_smooth_mouse_move(a0_buf, a0_buf.byteLength) as null;
}
export function op_mouse_move(a0: MouseMoveParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.op_mouse_move(a0_buf, a0_buf.byteLength) as null;
}
export function op_alert(a0: AlertParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.op_alert(a0_buf, a0_buf.byteLength) as null;
}
export function op_mouse_pos_y() {
  return _lib.symbols.op_mouse_pos_y() as number;
}
export function op_mouse_pos_x() {
  return _lib.symbols.op_mouse_pos_x() as number;
}
export function op_mouse_scroll() {
  return _lib.symbols.op_mouse_scroll() as null;
}
export function op_screenshot(a0: StrArg) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.op_screenshot(a0_buf, a0_buf.byteLength) as null;
}
export function op_type(a0: StrArg) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.op_type(a0_buf, a0_buf.byteLength) as null;
}
export function op_screensize_height() {
  return _lib.symbols.op_screensize_height() as number;
}
export function op_notify(a0: NotifyParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.op_notify(a0_buf, a0_buf.byteLength) as null;
}
export function op_toggle_key(a0: KeyToggleParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.op_toggle_key(a0_buf, a0_buf.byteLength) as null;
}
export function op_tap(a0: KeyToggleParams) {
  const a0_buf = encode(JSON.stringify(a0));
  return _lib.symbols.op_tap(a0_buf, a0_buf.byteLength) as null;
}
export function op_mouse_pixel_color_a() {
  return _lib.symbols.op_mouse_pixel_color_a() as number;
}
