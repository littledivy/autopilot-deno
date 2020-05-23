## Autopilot Deno

AutoPilot is a simple cross-platform desktop automation library for Deno.

### Features

- [x] Type a string using `.type`
- [x] Simulate mouse movement using `.moveMouse`
- [x] Get screen size using `.screenSize`
- [x] Alert box using `.alert`
- [x] Capture screen using `.screenshot`

### Documentation

In progress...

### Usage

Currently, I have not published plugins for Mac and Windows (because I don't have access to either). I encourage you to build this project on the platform and create an issue.

```typescript
import AutoPilot from 'https://raw.githubusercontent.com/divy-work/autopilot-deno/master/mod.ts';

// create a new AutoPilot instance.
var pilot = new AutoPilot();

// type a string
pilot.type("Yay! This works");

// alert something
pilot.alert("This is a alert")

// get screen size
pilot.screenSize();

// move mouse
pilot.moveMouse(200, 400);

// take a full-screen screenshot
pilot.screenshot("screenshot.png");
```
