import { runBenchmarks, bench } from "https://deno.land/std/testing/bench.ts";

import keyboard from "./keyboard.ts";
import mouse from "./mouse.ts";

const prebenchList = keyboard.concat(mouse);

export function createBench(pilot: any) {
  for (let i = 0; i < prebenchList.length; i++) {
    const benchItem = prebenchList[i];
    bench({
      name: benchItem.name,
      runs: 1,
      func(b): void {
        b.start();
        benchItem.do(pilot);
        b.stop();
        setTimeout(() => {
          
        }, 1000);
      },
    });
  }
}

export function runBench() {
  runBenchmarks();
}
