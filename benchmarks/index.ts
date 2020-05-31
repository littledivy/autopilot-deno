import AutoPilot from "../mod.ts";
import { createBench, runBench } from "./collect.ts";

let pilot = new AutoPilot();

// collect all benchmark defs
createBench(pilot);

// run all benchmarks
runBench();
