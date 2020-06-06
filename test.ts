import AutoPilot from "./mod.ts";

import keyboard from "./benchmarks/keyboard.ts";
import mouse from "./benchmarks/mouse.ts";
import screen from "./benchmarks/screen.ts";
import alert from "./benchmarks/alert.ts";
import window from "./benchmarks/window.ts";

const preTestList = keyboard.concat(mouse)
  .concat(screen)
  .concat(alert)
  .concat(window);

let pilot = new AutoPilot();

Deno.test({
  name: "create new instance",
  fn: () => {
    pilot = new AutoPilot();
  },
});

for (let i = 0; i < preTestList.length; i++) {
  const benchItem = preTestList[i];
  Deno.test({
    name: benchItem.name,
    async fn(): Promise<void> {
      return new Promise((resolve, reject) => {
          benchItem.do(pilot);
          resolve();
      });
    },
  });
}
