// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

export default function convert(list: string) {
  let listOfKeyCodes = list.split("\n").join("").trim().split(",").map((x) =>
    `"${x.trim().toLowerCase()}"`
  );
  return `type ToggleKeys = ${listOfKeyCodes.join("|")}`;
}

let x = convert(`F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    LeftArrow,
    Control,
    RightArrow,
    DownArrow,
    End,
    UpArrow,
    PageUp,
    Alt,
    Return,
    PageDown,
    Delete,
    Home,
    Escape,
    Backspace,
    Meta,
    CapsLock,
    Shift,
    Tab,
    Space`);
console.log(x);
