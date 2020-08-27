// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

export default [
  {
    name: "getWindow",
    do: async function (pilot: any) {
      await pilot.getWindow(0);
    },
  },
  {
    name: "transformByIndex",
    do: async function (pilot: any) {
      await pilot.transformByIndex(3, 300, 500);
    },
  },
];
