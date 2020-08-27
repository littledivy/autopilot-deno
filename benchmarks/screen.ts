// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

export default [
  {
    name: "screenSize",
    do: async function (pilot: any) {
      await pilot.screenSize();
    },
  },
  {
    name: "screenshot",
    do: async function (pilot: any) {
      await pilot.screenshot("screenshot.png");
    },
  },
  {
    name: "pointVisible",
    do: async function (pilot: any) {
      await pilot.pointVisible(1000, 2000);
    },
  },
  {
    name: "screenScale",
    do: async function (pilot: any) {
      await pilot.screenScale();
    },
  },
  {
    name: "getMonitors",
    do: async function (pilot: any) {
      await pilot.getMonitors();
    },
  },
];
