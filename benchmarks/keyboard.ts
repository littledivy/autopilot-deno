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
