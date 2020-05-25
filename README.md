<p align="center">

![](./docs/logo.png)

</p>

[![stars](https://img.shields.io/github/stars/divy-work/autopilot-deno)](https://github.com/divy-work/autopilot-deno/stargazers)
[![issues](https://img.shields.io/github/issues/divy-work/autopilot-deno)](https://github.com/divy-work/autopilot-deno/issues)
[![ci](https://github.com/divy-work/autopilot-deno/workflows/ci/badge.svg)](https://github.com/divy-work/autopilot-deno/actions)
[![releases](https://img.shields.io/github/downloads/divy-work/autopilot-deno/total)](https://github.com/divy-work/autopilot-deno/releases/latest/)
![deno version](https://img.shields.io/badge/deno-1.0.2-success)

AutoPilot is a simple cross-platform desktop automation library for Deno.


### Features

- [x] Type a string using `.type`
- [x] Simulate mouse movement using `.moveMouse`
- [x] Get screen size using `.screenSize`
- [x] Alert box using `.alert`
- [x] Capture screen using `.screenshot`

### Documentation

https://autopilot.divy.work

### Requirements

#### Linux
```sh
sudo apt-get update
sudo apt-get install libxtst-dev cmake libc-dev libx11-dev libxcb1-dev
```

### Windows
Run cmd as adminsirator

### Usage

**NOTE**: Prebuilt binaries are automatically downloaded the first time you import Autopilot in your project and are cached.

```typescript
import AutoPilot from 'https://raw.githubusercontent.com/divy-work/autopilot-deno/master/mod.ts';

// create a new AutoPilot instance.
var pilot = new AutoPilot();

// type a string
pilot.type("Yay! This works");

// alert something
pilot.alert("This is a alert");

// get screen size
pilot.screenSize();

// move mouse
pilot.moveMouse(200, 400);

// take a full-screen screenshot
pilot.screenshot("screenshot.png");
```
