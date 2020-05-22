import { runType, runAlert, runScreenSize, runMoveMouse } from "./plugin/index.js";

class AutoPilot {
  type(str: string) {
    runType(str);
    return this;
  }
  alert(str: string) {
    runAlert(str);
    return this;
  }
  screenSize() {
    return JSON.parse(runScreenSize());
  }
  moveMouse(x: number, y: number) {
    runMoveMouse({ x, y })
    return this;
  }
}

export default AutoPilot;

/** Example:

new AutoPilot()
  .type("typing works fine...")
  .alert('bruh')
  .moveMouse(100, 400)

console.log(new AutoPilot().screenSize());
**/
