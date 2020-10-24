import AutoPilot from "../mod.ts";

var pilot = new AutoPilot();
var screenSize = await pilot.screenSize();

var height = screenSize.height;
var width = screenSize.width;

console.log(`width: ${width}\nheight: ${height}`);
