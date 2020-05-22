## Autopilot Deno

AutoPilot is a simple cross-platform desktop automation library for Deno.

### Features

- [x] Type a string using `.type`
- [x] Simulate mouse movement using `.moveMouse`
- [x] Get screen size using `.screenSize`
- [ ] Capture screen
- [x] Alert box using `.alert`

### Usage

NOTE: This package has not been configured to use directly. You will have to clone this repo and use. Any PRs that so this are welcomed :)

```sh
$ cargo build
```

```typescript
import AutoPilot from './mod.ts';

// create a new AutoPilot instance.
var pilot = new AutoPilot();

// type and alert a string
pilot
  .type("Yay! This works")
  .alert("This is a alert")

// get screen size
pilot.screenSize();

// set mouse position
pilot.moveMouse(200, 300);
```
