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
      pilot.tap("down");
    },
  },
];
