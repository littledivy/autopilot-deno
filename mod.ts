import { runType, runAlert } from './plugin/index.js';

class AutoPilot {
  type(str: string) {
    runType(str);
    return this;
  }
  alert(str: string) {
    runAlert(str);
    return this;
  }
}

export default AutoPilot;

/** Example:
new AutoPilot()
  .type("typing works fine...")
  .alert('bruh')
**/
