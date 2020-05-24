import {
  runType,
  runAlert,
  runScreenSize,
  runMoveMouse,
  runScreenShot,
  runMouseClick,
  runKeyTap,
} from "./plugin/index.js";

interface AlertOptions {
  title?: string;
  msg: string;
}

type ClickOptions = "left" | "right";

class AutoPilot {
  type(str: string) {
    runType(str);
    return this;
  }
  alert(opt: string | AlertOptions) {
    runAlert(opt);
    return this;
  }
  screenSize() {
    return JSON.parse(runScreenSize());
  }
  moveMouse(x: number, y: number, d?: number) {
    runMoveMouse({ x, y, d });
    return this;
  }
  screenshot(file: string) {
    runScreenShot(file);
    return this;
  }
  tap(arg: string) {
    arg = arg.trim().toLowerCase();
    runKeyTap(arg);
    return this;
  }
  click(arg: ClickOptions) {
    runMouseClick(arg);
    return this;
  }
}

export default AutoPilot;
