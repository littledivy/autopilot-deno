// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

export default [
  {
    name: "type",
    do: async function (pilot: any) {
      await pilot.type("hello");
    },
  },
  {
    name: "tap",
    do: async function (pilot: any) {
      await pilot.tap("alt");
    },
  },
  {
    name: "toggle",
    do: async function (pilot: any) {
      await pilot.toggleKey("alt");
    },
  },
];
