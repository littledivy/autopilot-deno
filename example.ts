import AutoPilot from "./mod.ts";

const pilot = new AutoPilot();

const screenSize = pilot.screenSize();

const height = (screenSize.height / 2) - 10;
const width = screenSize.width;

let y: number;
for (let x = 0; x < width; x++) {
  y = height * Math.sin((Math.PI * 2 * x) / width) + height;
  pilot.moveMouse(x, y);
}
