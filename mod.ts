import { runType, runAlert, runScreenSize, runMoveMouse, runScreenShot } from "./plugin/index.js";

interface AlertOptions = {
  title: string,
  msg: string
}

class AutoPilot {
  type(str: string) {
    runType(str);
    return this;
  }
  alert(opt: string | obj) {
    runAlert(opt);
    return this;
  }
  screenSize() {
    return JSON.parse(runScreenSize());
  }
  moveMouse(x: number, y: number) {
    runMoveMouse({ x, y })
    return this;
  }
  screenshot(file: string) {
    runScreenShot(file);
    return this;
  }
}

export default AutoPilot;

/** Example:

new AutoPilot()
  .type("typing works fine...")
  .alert('bruh')
  .moveMouse(100, 400)
  .screenshot()

console.log(new AutoPilot().screenSize());
**/
