## Introduction

![deno version](https://img.shields.io/badge/deno-1.12.5-success)

AutoPilot is a simple cross-platform desktop automation library for Deno.

## Requirements

### Linux

```sh
sudo apt-get install libxtst-dev cmake libc-dev libx11-dev libxcb1-dev
```

## Quick start

Simple import `Autopilot` from Github raw cdn.

```typescript
import AutoPilot from "https://x.nest.land/autopilot/mod.ts";
```

## Usage

The `AutoPilot` constructors provides the API for interacting with the rust
bindings.

```typescript
const pilot = new AutoPilot();
```

See the list of available methods inside `Autopilot` constructor.

## API

### Keyboard

#### .type

Simulates keyboard input to type the given string.

Example:

```typescript
await pilot.type("Hello, World!");
```

#### .tap

Simulates a key tap for the given key.

Example:

```typescript
await pilot.tap("enter");
```

#### .toggleKey

Toggle the given key.

Arguments:

- `key` - string - The key to be toggled.
- `down` - boolean - Whether to press down or not.

Example:

```typescript
await pilot.toggleKey("enter", true);
```

##### Keymap

Below is the key map that is accepted by AutoPilot.

```typescript
type ToggleKeys =
  | "f1"
  | "f2"
  | "f3"
  | "f4"
  | "f5"
  | "f6"
  | "f7"
  | "f8"
  | "f9"
  | "f10"
  | "f11"
  | "f12"
  | "f13"
  | "f14"
  | "f15"
  | "f16"
  | "f17"
  | "f18"
  | "f19"
  | "f20"
  | "f21"
  | "f22"
  | "f23"
  | "f24"
  | "leftarrow"
  | "control"
  | "rightarrow"
  | "downarrow"
  | "end"
  | "uparrow"
  | "pageup"
  | "alt"
  | "return"
  | "pagedown"
  | "delete"
  | "home"
  | "escape"
  | "backspace"
  | "meta"
  | "capslock"
  | "shift"
  | "tab"
  | "space";
```

### Mouse

#### .moveMouse

Move the mouse cursor to the given position.

Arguments:

- `x` - number - abcissa of mouse position
- `y` - number - ordinate of mouse position

Example:

```typescript
await pilot.moveMouse(300, 500);
```

#### .mousePosition

Get current mouse position.

Returns type `Point`.

```typescript
interface Point {
  x: number;
  y: number;
}
```

Example:

```typescript
await pilot.mousePosition();
```

#### .pixelColor

Get current mouse pixel color.

Returns type `RGBA`.

```typescript
interface RGBA {
  r: number;
  g: number;
  b: number;
  a: number;
}
```

Example:

```typescript
await pilot.pixelColor();
```

#### .click

Simulate a click

Argument:

```typescript
type ClickArguments = "right" | "left" | "middle";
```

Example:

```typescript
await pilot.click("right");
```

#### .scroll

Simulate a mouse scroll (5 ticks).

Argument:

```typescript
type ScrollArguments = "up" | "down";
```

Example:

```typescript
await pilot.scroll("up");
```

### Screen

#### .screenSize

Get the screen size.

Returns:

```typescript
interface ScreenSize {
  height: number;
  width: number;
}
```

Example:

```typescript
await pilot.screenSize();
```

#### .screenshot

Simulate a screengrab and output an image.

Argument: Output image name

Example:

```typescript
await pilot.screenshot("screenshot.png");
```

#### .screenScale

Get the number of pixels in a point

Returns: `number`

Example:

```typescript
await pilot.screenScale(); // mostly outputs 1
```

### Notifications

#### .alert

Trigger a native popup.

Arguments:

```typescript
interface AlertOptions {
  title?: string;
  msg: string;
}
```

Example:

```typescript
await pilot.alert({
  title: "Oops! This is an alert!",
  msg: "Something definetly went wrong!",
});
```

#### .notify

Trigger a native OS notification.

Arguments:

- `title` - string - notification title
- `body` - string - notification body

```typescript
await pilot.notify("Hello", "World");
```

### Contributing

```shell
git clone https://github.com/littledivy/autopilot-deno \
  && cd autopilot-deno \
  && cargo build
```

```shell
# DEV env variable tells autopilot to use local builds
DEV=1 deno run --unstable --allow-plugin example.ts
```
