export default [
  {
    name: "getWindow",
    do: function (pilot: any) {
      pilot.getWindow(0);
    },
  },
  {
    name: "transformByIndex",
    do: function (pilot: any) {
      pilot.transformByIndex(3, 300, 500);
    },
  },
];
