import { runBenchmarks, bench } from "https://deno.land/std/testing/bench.ts";

import keyboard from "./keyboard.ts";
import mouse from "./mouse.ts";

const prebenchList = keyboard.concat(mouse);

export function createBench(pilot: any, logger: any) {
  for (let i = 0; i < prebenchList.length; i++) {
    const benchItem = prebenchList[i];
    logger.debug(`collecting ${benchItem.name}`);
    bench({
      name: benchItem.name,
      runs: 1,
      func(b): void {
        b.start();
        benchItem.do(pilot);
        b.stop();
      },
    });
  }
}

export function runBench() {
  var x = runBenchmarks();
  console.log(x);
}
