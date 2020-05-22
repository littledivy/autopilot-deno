## Autopilot Deno

AutoPilot is a simple cross-platform desktop automation library for Deno.

### Features

- [x] Type a string using `.type`
- [x] Simulate mouse movement using `.moveMouse`
- [x] Get screen size
- [x] Alert box using `.alert`
- [ ] Capture screen

### Documentation

In progress...

### Usage

```typescript
import AutoPilot from 'https://raw.githubusercontent.com/divy-work/autopilot-deno/master/mod.ts';

// create a new AutoPilot instance.
var pilot = new AutoPilot();

// type and alert a string
pilot
  .type("Yay! This works")
  .alert("This is a alert")

// get screen size
pilot.screenSize();

// move mouse
pilot.moveMouse(200, 400)
```
