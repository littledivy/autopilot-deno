import AutoPilot from "https://raw.githubusercontent.com/divy-work/autopilot-deno/master/mod.ts";

var pilot = new AutoPilot();
var screenSize = pilot.screenSize();

var height = screenSize.height;
var width = screenSize.width;

console.log(`width: ${width}\nheight: ${height}`);
