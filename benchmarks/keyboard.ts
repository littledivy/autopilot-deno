// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

export default [
  {
    name: "type",
    do: function (pilot: any) {
      pilot.type("hello");
    },
  },
  {
    name: "tap",
    do: function (pilot: any) {
      pilot.tap("alt");
    },
  },
  {
    name: "toggle",
    do: function (pilot: any) {
      pilot.toggleKey("alt");
    },
  },
];
