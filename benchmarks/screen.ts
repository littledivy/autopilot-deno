export default [
  {
    name: "screenSize",
    do: function(pilot: any) {
      pilot.screenSize();
    }
  },
  {
    name: "screenshot",
    do: function(pilot: any) {
      pilot.screenshot("screenshot.png");
    }
  },
  {
    name: "getMonitors",
    do: function(pilot: any) {
      pilot.getMonitors();
    }
  }
]
