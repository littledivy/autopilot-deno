import { runBenchmarks, bench } from "https://deno.land/std/testing/bench.ts";
import AutoPilot from '../mod.ts';

let pilot = new AutoPilot();

bench({
  name: "type",
  runs: 1,
  func(b): void {
    b.start();
    pilot.type("hello");
    b.stop();
  },
});

bench({
  name: "click",
  runs: 1,
  func(b): void {
    b.start();
    pilot.click("left");
    b.stop();
  }
})

runBenchmarks()
