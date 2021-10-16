<p align="center">

<p align="center">
  <img src="docs/logo.png">
</p>

</p>

[![ci](https://github.com/littledivy/autopilot-deno/workflows/ci/badge.svg)](https://github.com/littledivy/autopilot-deno/actions)
![deno version](https://img.shields.io/badge/deno-1.15.1-success)
[![nest badge](https://nest.land/badge.svg)](https://nest.land/package/autopilot)

**AutoPilot Deno -** Cross-platform desktop automation framework for Deno.

### Features

- [x] **Keyboard**

  - [x] Type a string using `.type`
  - [x] Tap a key using `.tap`
  - [x] Toggle key using `.toggleKey`

- [x] **Mouse**

  - [x] Simulate mouse movement using `.moveMouse`
  - [x] Click using `.click`
  - [x] Simulate a scroll using `.scroll`
  - [x] Get mouse position using `.mousePosition`
  - [x] Get mouse position pixel color `.pixelColor`

- [x] **Screen**

  - [x] Capture screen using `.screenshot`
  - [x] Get screen size using `.screenSize`
  - [x] Get number of pixels in a point using `.screenScale`

- [x] **Notifications**

  - [x] Native popup using `.alert`
  - [x] Notifications using `.notify`

### Documentation

Detailed documentation of the API is available at:

https://autopilot.mod.land

### Requirements

#### Linux

```sh
sudo apt-get update
sudo apt-get install libdbus-1-dev x11-xserver-utils wmctrl libxtst-dev cmake libc-dev libx11-dev libxcb1-dev
```

### Usage

```sh
deno run --unstable --allow-ffi example.ts
```

```typescript
// example.ts
import AutoPilot from "https://deno.land/x/autopilot/mod.ts";

const pilot = new AutoPilot();

// type a string
await pilot.type("Yay! This works");
// get screen size
const screenSize = await pilot.screenSize();
// move mouse
await pilot.moveMouse(200, 400);
// take a screenshot
await pilot.screenshot("screenshot.png");
// ...and more stuff
```

### License

[MIT License](LICENSE).
