import AutoPilot from "./mod.ts";

const pilot = new AutoPilot();

// alert without params
pilot.alert("alert without params");

// alert with params
pilot.alert({
  title: "alert title",
  msg: "alert with params",
});

// type a string
pilot.type("typing smth...");

// move mouse
pilot.moveMouse(300, 500);

// take a screenshot
pilot.screenshot("screenshot.png");

// mouse click
pilot.click("left");

// tap a key
pilot.tap("enter");
