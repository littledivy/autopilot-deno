import AutoPilot from "../mod.ts";

var pilot = new AutoPilot();

setTimeout(() => {
  pilot.toggleKey("enter", false);
}, 2000);

pilot.toggleKey("enter", true);
