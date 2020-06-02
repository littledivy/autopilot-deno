import { runBenchmarks, bench } from "https://deno.land/std/testing/bench.ts";
import write from "./write.ts";

import keyboard from "./keyboard.ts";
import mouse from "./mouse.ts";
import screen from "./screen.ts";
import alert from "./alert.ts";
import window from "./window.ts";

const prebenchList = keyboard.concat(mouse).concat(screen).concat(alert).concat(window);

export function createBench(pilot: any, logger: any) {
  for (let i = 0; i < prebenchList.length; i++) {
    const benchItem = prebenchList[i];
    logger.debug(`collecting ${benchItem.name}`);
    bench({
      name: benchItem.name,
      runs: 1,
      async func(b): Promise<void> {
        return new Promise((resolve, reject) => {
          setTimeout(() => {
            b.start();
            benchItem.do(pilot);
            b.stop();
            resolve();
          }, 2000);
        })
      },
    });
  }
}

export async function runBench() {
  var res = await runBenchmarks();
  let toWrite = {
    results: res.results,
    os: Deno.build,
    metrics: Deno.metrics()
  }
  write(toWrite);
}
