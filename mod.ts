import { runType } from './plugin/index.js';

class AutoPilot {
  type(str: string) {
    runType(str);
    return this;
  }
}

export default AutoPilot;

/**
new AutoPilot()
  .type("typing works fine...")
**/
