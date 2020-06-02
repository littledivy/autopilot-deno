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
    name: "pointVisible",
    do: function(pilot: any) {
      pilot.pointVisible(1000, 2000);
    }
  },
  {
    name: "screenScale",
    do: function(pilot: any) {
      pilot.screenScale();
    }
  },
  {
    name: "getMonitors",
    do: function(pilot: any) {
      pilot.getMonitors();
    }
  }
]
