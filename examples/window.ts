// ONLY SUPPORTED ON LINUX
import AutoPilot from "../mod.ts";

const pilot = new AutoPilot();

console.log(await pilot.getWindow(parseInt(Deno.args[0]) || 1));
