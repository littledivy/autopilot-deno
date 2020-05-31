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
      pilot.tap("control");
    },
  },
  {
    name: "toggle",
    do: function (pilot: any) {
      pilot.toggleKey("alt");
    },
  },
];
