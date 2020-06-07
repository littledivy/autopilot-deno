// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

export default [
  {
    name: "click",
    do: function (pilot: any) {
      pilot.click("right");
    },
  },
  {
    name: "scroll",
    do: function (pilot: any) {
      pilot.tap("up");
    },
  },
  {
    name: "pixelColor",
    do: function (pilot: any) {
      pilot.pixelColor();
    },
  },
  {
    name: "mousePosition",
    do: function (pilot: any) {
      pilot.mousePosition();
    },
  },
  {
    name: "moveMouse",
    do: function (pilot: any) {
      pilot.moveMouse(500, 500, 0);
    },
  },
];
