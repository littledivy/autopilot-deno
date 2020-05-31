import AutoPilot from "../mod.ts";
import { createBench, runBench } from "./collect.ts";

let pilot = new AutoPilot();

createBench(pilot);

runBench();
