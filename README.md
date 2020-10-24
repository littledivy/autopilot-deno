<p align="center">

<p align="center">
  <img src="docs/logo.png">
</p>

</p>

[![stars](https://img.shields.io/github/stars/divy-work/autopilot-deno)](https://github.com/divy-work/autopilot-deno/stargazers)
[![issues](https://img.shields.io/github/issues/divy-work/autopilot-deno)](https://github.com/divy-work/autopilot-deno/issues)
[![ci](https://github.com/divy-work/autopilot-deno/workflows/ci/badge.svg)](https://github.com/divy-work/autopilot-deno/actions)
![version](https://img.shields.io/badge/version-0.1.9-success)
![deno version](https://img.shields.io/badge/deno-1.3.0-success)
[![vr scripts](https://badges.velociraptor.run/flat.svg)](https://velociraptor.run)
[![nest badge](https://nest.land/badge.svg)](https://nest.land/package/autopilot)


Cross-platform desktop automation framework for Deno.


### Features

- [x] __Keyboard__
  - [x] Type a string using `.type`
  - [x] Tap a key using `.tap`
  - [x] Toggle key using `.toggleKey`

- [x] __Mouse__
  - [x] Simulate mouse movement using `.moveMouse`
  - [x] Click using `.click`
  - [x] Simulate a scroll using `.scroll`
  - [x] Get mouse position using `.mousePosition`
  - [x] Get mouse position pixel color `.pixelColor`

- [x] __Screen__
  - [x] Capture screen using `.screenshot`
  - [x] Get screen size using `.screenSize`
  - [x] Check if point out of bounds using `.pointVisible`
  - [x] Get number of pixels in a point using `.screenScale`

- [x] __Notifications__
  - [x] Native popup using `.alert`
  - [x] Notifications using `.notify`

- [x] __Monitors__
   - [x] Get the number of monitors using `.getMonitors`

- [x] __Window management (only for linux)__
  - [x] Get window title using `.getWindow`
  - [x] Transform windows size using `.transformByIndex`

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

Running your Deno script with AutoPilot requires some flags
```sh
deno run --unstable -A file.ts
```

**NOTE**: Prebuilt binaries are automatically downloaded the first time you import Autopilot in your project and are cached.

```typescript
import AutoPilot from 'https://deno.land/x/autopilot@0.2.1/mod.ts';

// create a new AutoPilot instance.
var pilot = new AutoPilot();

// type a string
await pilot.type("Yay! This works");

// alert something
await pilot.alert("This is a alert");

// get screen size
await pilot.screenSize();

// move mouse
await pilot.moveMouse(200, 400);

// take a full-screen screenshot
await pilot.screenshot("screenshot.png");
```

### Env variables

* `CACHE`: Set if you want to update the release from Github.
* `DEV`: Set if you want t use the development debug builds.

## Development

* Requires rust and cargo.
* Requires [velociraptor](https://velociraptor.run)

Clone this repo and run `cargo build` to produce a development build.

#### Commands

* Run __tests__: `vr test`
* Run and produce __benchmarks__: `vr bench`
* Format code __fmt__: `vr fmt`
* Create development build: `vr build`

### Contributing
Contributing code and ideas to AutoPilot is really easy! Create an issue or PR. :)

### License

See [MIT License](LICENSE). All rights reserved. Divy Srivastava 2020.
