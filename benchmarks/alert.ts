// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

export default [
  {
    name: "notify",
    do: async function (pilot: any) {
      await pilot.notify("Hello", "Benchmarks!");
    },
  },
  {
    name: "alert",
    do: async function (pilot: any) {
      await pilot.alert("Benchmarks");
    },
  },
];
