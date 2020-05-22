import { runType, runAlert, runScreenSize } from "./plugin/index.js";

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
}

export default AutoPilot;


/** Example:
new AutoPilot()
  .type("typing works fine...")
  .alert('bruh')

console.log(new AutoPilot().screenSize());
**/
