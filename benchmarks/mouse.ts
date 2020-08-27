// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

export default [
  {
    name: "click",
    do: async function (pilot: any) {
      await pilot.click("right");
    },
  },
  {
    name: "scroll",
    do: async function (pilot: any) {
      await pilot.tap("up");
    },
  },
  {
    name: "pixelColor",
    do: async function (pilot: any) {
      await pilot.pixelColor();
    },
  },
  {
    name: "mousePosition",
    do: async function (pilot: any) {
      await pilot.mousePosition();
    },
  },
  {
    name: "moveMouse",
    do: async function (pilot: any) {
      await pilot.moveMouse(500, 500, 0);
    },
  },
];
