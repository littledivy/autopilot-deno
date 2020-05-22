## Autopilot Deno

AutoPilot is a simple cross-platform desktop automation library for Deno.

### Features

- [x] Type a string using keyboard
- [ ] Simulate mouse movement
- [ ] Get screen size
- [ ] Capture screen

### Usage

```typescript
import AutoPilot from './mod.ts';

// create a new AutoPilot instance.
var pilot = new AutoPilot();

// type a string
pilot.type("Yay! This works")
```
