export default [
  {
    name: "notify",
    do: function (pilot: any) {
      pilot.notify("Hello", "Benchmarks!");
    },
  },
  {
    name: "alert",
    do: function (pilot: any) {
      pilot.alert("Benchmarks");
    },
  },
];
