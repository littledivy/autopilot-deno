## Introduction

> Warning: These are for the latest version of Autopilot

![deno version](https://img.shields.io/badge/deno-1.0.5-success)

AutoPilot is a simple cross-platform desktop automation library for Deno.

## Requirements

### Linux

```sh
sudo apt-get install libxtst-dev cmake libc-dev libx11-dev libxcb1-dev
```

## Quick start

Simple import `Autopilot` from Github raw cdn.

```typescript
import AutoPilot from 'https://x.nest.land/autopilot@0.2.0/mod.ts';
```

> Info: AutoPilot automatically installs prebuilt binaries for the first time and caches it for future runs.

## Usage

The `AutoPilot` constructors provides the API for interacting with the rust bindings.

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

![](assets/type.gif)

#### .tap

Simulates a key tap for the given key.

Example:
```typescript
await pilot.tap("enter");
```

#### .toggleKey

Toggle the given key.


Arguments:

* `key` - string - The key to be toggled.
* `down` - boolean - Whether to press down or not.

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

* `x` - number - abcissa of mouse position
* `y` - number - ordinate of mouse position

Example:
```typescript
await pilot.moveMouse(300, 500);
```

![](assets/mouse-move.gif)

#### .mousePosition

Get current mouse position.

Returns type `Point`.

```typescript
interface Point {
    x: number,
    y: number
}
```

Example:
```typescript
await pilot.mousePosition();
```

![](assets/mouse-position.gif)

#### .pixelColor

Get current mouse pixel color.

Returns type `RGBA`.

```typescript
interface RGBA {
    r: number,
    g: number,
    b: number,
    a: number
}
```

Example:
```typescript
await pilot.pixelColor();
```

![](assets/pixel-color.gif)


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
  height: number,
  width: number
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

#### .pointVisible

Check whether point is out of screen bounds or not.

Arguments:

* `x` - number - abcissa of point
* `y` - number - ordinate of point

Example:
```typescript
await pilot.pointVisible(100, 35);
```

### Notifications

#### .alert

Trigger a native popup.

Arguments:
```typescript
type Alert = string | AlertOptions;
// where
interface AlertOptions {
  title?: string;
  msg: string;
}
```

Example:
```typescript
await pilot.alert("alert without title");
// or
await pilot.alert({
  title: "Oops! This is an alert!"
  msg: "Something definetly went wrong!"
})
```

![](assets/alert.gif)

#### .notify

Trigger a native OS notification.

Arguments:
* `title` - string - notification title
* `body` - string - notification body

```typescript
await pilot.notify("Hello", "World");
```

### Monitors

#### .getMonitors

Get the number of monitors.

Returns: `Promise<number>`

Example:
```typescript
await pilot.getMonitors();
```

### Window management (linux)

#### .getWindow

Get window title by its index.

Arguments:
* `index` - number - The index of window

```typescript
await pilot.getWindow(0); // outputs 'Desktop', maybe?
```

#### .transformByIndex

Transform a window by its index.

Arguments:
* `index` - number - The index of window
* `height` - number - Desired height of the window
* `width` - number - Desired width of the window

```typescript
await pilot.transformByIndex(3, 960, 300);
```

### Building from source

> Warning: This requires Rust and Cargo setup in your machine.

Clone the github repo
```sh
git clone https://github.com/divy-work/autopilot-deno
cd autopilot
cargo build
```

Running `cargo build` might take several minutes to build for the first time.

Now, you can test the current builds using the `DEV` env variable

```sh
DEV=* deno run --unstable --allow-* examples/sine.ts
```

### OS Support

* Ubuntu - Full Support
* MacOS - Full support except Window management
* Windows - Full support except Window managment

If you find any difficulty while building from source or using this module, please open a issue or PR stating your issue.
