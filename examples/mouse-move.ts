import AutoPilot from "../mod.ts";
var pilot = new AutoPilot();
var screenSize = await pilot.screenSize();
var widthUnit: number = screenSize.height / 6;
setInterval(async function () {
  var position = await pilot.mousePosition();
  if (position.x > widthUnit * 2) {
    await pilot.moveMouse(widthUnit, position.y);
  } else {
    await pilot.moveMouse(widthUnit * 3, position.y);
  }
}, 200);
