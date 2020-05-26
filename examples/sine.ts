import AutoPilot from "../mod.ts";

var pilot = new AutoPilot();
var twoPI = Math.PI * 2.0;
var screenSize = pilot.screenSize();
var height = (screenSize.height / 2) - 10;
var width = screenSize.width;

var y;
for (var x = 0; x < width; x++) {
  y = height * Math.sin((twoPI * x) / width) + height;
  pilot.moveMouse(x, y);
}
