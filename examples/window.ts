// ONLY SUPPORTED ON LINUX
import AutoPilot from "../mod.ts";

const pilot = new AutoPilot();

console.log(pilot.getWindow(parseInt(Deno.args[0]) || 3));
