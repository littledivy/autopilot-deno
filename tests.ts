import AutoPilot from './mod.ts';

const pilot = new AutoPilot();

// alert without params
pilot.alert("alert without params")

// alert with params
pilot.alert({
  title: "alert title",
  msg: "alert with params"
})

// type a string
pilot.type("types a string")

// move mouse
pilot.moveMouse(300, 500);

// take a screenshot
pilot.screenshot("screenshot.png");

///// EXPERIMENTAL STUFF BELOW
// tap a key
pilot.tap("Enter")
