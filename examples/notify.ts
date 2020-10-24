// https://deno.land/x/autopilot/mod.ts
import AutoPilot from "../mod.ts";

const pilot = new AutoPilot();

await pilot.notify("Hello", "World!");