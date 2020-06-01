export interface AlertOptions {
  title?: string;
  msg: string;
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
