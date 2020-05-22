// Copyright 2018, 2019, 2020 Michael Sanders
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT License <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
//! This module contains functions for getting the current state of and
//! controlling the mouse cursor.
//!
//! Unless otherwise stated, coordinates are those of a screen coordinate
//! system, where the origin is at the top left.

use geometry::Point;
use screen;
use std;
use std::fmt;

#[cfg(target_os = "macos")]
use core_graphics::event::{
    CGEvent, CGEventTapLocation, CGEventType, CGMouseButton, ScrollEventUnit,
};
#[cfg(target_os = "macos")]
use core_graphics::event_source::CGEventSource;
#[cfg(target_os = "macos")]
use core_graphics::event_source::CGEventSourceStateID::HIDSystemState;
#[cfg(target_os = "macos")]
use core_graphics::geometry::CGPoint;
#[cfg(windows)]
use winapi::shared::minwindef::DWORD;

#[cfg(target_os = "linux")]
use internal;
#[cfg(target_os = "linux")]
use x11;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Button {
    Left,
    Middle,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ScrollDirection {
    Up,
    Down,
}

#[derive(Debug)]
pub enum MouseError {
    OutOfBounds,
}

impl fmt::Display for MouseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MouseError::OutOfBounds => write!(f, "Out of bounds"),
        }
    }
}

impl std::error::Error for MouseError {}

/// Gradually moves the mouse to a coordinate in a straight line in the given
/// time frame (in seconds). If no duration is given a 1 millisecond delay is
/// defaulted to between mouse movements.
///
/// Returns `MouseError` if coordinate is outside the screen boundaries.
pub fn smooth_move(destination: Point, duration: Option<f64>) -> Result<(), MouseError> {
    if !screen::is_point_visible(destination) {
        return Err(MouseError::OutOfBounds);
    }

    let start_position = location();
    let distance = (start_position.x - destination.x).hypot(start_position.y - destination.y);
    let step_count = distance.ceil() as i64;
    let interval: u64 = duration
        .map(|d| (d * 1000.0) / distance)
        .unwrap_or(1.0)
        .round() as u64;

    for step in 1..=step_count {
        let position = Point::new(
            (destination.x - start_position.x) * (step as f64 / step_count as f64)
                + start_position.x,
            (destination.y - start_position.y) * (step as f64 / step_count as f64)
                + start_position.y,
        );

        move_to(position)?;
        std::thread::sleep(std::time::Duration::from_millis(interval));
    }

    Ok(())
}

/// A convenience wrapper around `toggle()` that holds down and then releases
/// the given mouse button. Delay between pressing and releasing the key can be
/// controlled using the `delay_ms` parameter. If `delay` is not given, the
/// value defaults to 100 ms.
pub fn click(button: Button, delay_ms: Option<u64>) {
    toggle(button, true);
    std::thread::sleep(std::time::Duration::from_millis(delay_ms.unwrap_or(100)));
    toggle(button, false);
}

/// Immediately moves the mouse to the given coordinate.
///
/// Returns `MouseError` if coordinate is outside the screen boundaries.
pub fn move_to(point: Point) -> Result<(), MouseError> {
    if !screen::is_point_visible(point) {
        Err(MouseError::OutOfBounds)
    } else {
        system_move_to(point);
        Ok(())
    }
}

/// Returns the current position of the mouse cursor.
pub fn location() -> Point {
    system_location()
}

/// Holds down or releases a mouse button in the current position.
pub fn toggle(button: Button, down: bool) {
    system_toggle(button, down);
}

/// Performs a scroll event in a direction a given number of times.
pub fn scroll(direction: ScrollDirection, clicks: u32) {
    system_scroll(direction, clicks);
}

#[cfg(target_os = "macos")]
impl Button {
    fn event_type(self, down: bool) -> CGEventType {
        use core_graphics::event::CGEventType::*;
        match (self, down) {
            (Button::Left, true) => LeftMouseDown,
            (Button::Left, false) => LeftMouseUp,
            (Button::Right, true) => RightMouseDown,
            (Button::Right, false) => RightMouseUp,
            (Button::Middle, true) => OtherMouseDown,
            (Button::Middle, false) => OtherMouseUp,
        }
    }
}

#[cfg(target_os = "macos")]
impl From<Button> for CGMouseButton {
    fn from(button: Button) -> CGMouseButton {
        use core_graphics::event::CGMouseButton::*;
        match button {
            Button::Left => Left,
            Button::Middle => Center,
            Button::Right => Right,
        }
    }
}

#[cfg(target_os = "macos")]
fn system_move_to(point: Point) {
    let point = CGPoint::from(point);
    let source = CGEventSource::new(HIDSystemState).unwrap();
    let event =
        CGEvent::new_mouse_event(source, CGEventType::MouseMoved, point, CGMouseButton::Left);
    event.unwrap().post(CGEventTapLocation::HID);
}

#[cfg(target_os = "macos")]
fn system_location() -> Point {
    let source = CGEventSource::new(HIDSystemState).unwrap();
    let event = CGEvent::new(source).unwrap();
    Point::from(event.location())
}

#[cfg(target_os = "macos")]
fn system_toggle(button: Button, down: bool) {
    let point = CGPoint::from(location());
    let source = CGEventSource::new(HIDSystemState).unwrap();
    let event_type = button.event_type(down);
    let event = CGEvent::new_mouse_event(source, event_type, point, CGMouseButton::from(button));
    event.unwrap().post(CGEventTapLocation::HID);
}

#[cfg(target_os = "macos")]
fn system_scroll(direction: ScrollDirection, clicks: u32) {
    for _ in 0..clicks {
        let wheel_count = if direction == ScrollDirection::Up {
            10
        } else {
            -10
        };
        let source = CGEventSource::new(HIDSystemState).unwrap();
        let event = CGEvent::new_scroll_event(source, ScrollEventUnit::LINE, 1, wheel_count, 0, 0);
        event.unwrap().post(CGEventTapLocation::HID);
    }
}

#[cfg(windows)]
fn mouse_event_for_button(button: Button, down: bool) -> DWORD {
    use winapi::um::winuser::{
        MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP,
        MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,
    };
    match (button, down) {
        (Button::Left, true) => MOUSEEVENTF_LEFTDOWN,
        (Button::Left, false) => MOUSEEVENTF_LEFTUP,
        (Button::Right, true) => MOUSEEVENTF_RIGHTDOWN,
        (Button::Right, false) => MOUSEEVENTF_RIGHTUP,
        (Button::Middle, true) => MOUSEEVENTF_MIDDLEDOWN,
        (Button::Middle, false) => MOUSEEVENTF_MIDDLEUP,
    }
}

#[cfg(windows)]
fn system_move_to(point: Point) {
    use winapi::ctypes::c_int;
    use winapi::um::winuser::SetCursorPos;
    let scaled_point = point.scaled(screen::scale()).round();
    unsafe {
        SetCursorPos(scaled_point.x as c_int, scaled_point.y as c_int);
    };
}

#[cfg(windows)]
fn system_location() -> Point {
    use winapi::shared::windef::POINT;
    use winapi::um::winuser::GetCursorPos;
    let mut point: POINT = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut point);
    }
    Point::from_pixel(f64::from(point.x), f64::from(point.y), screen::scale())
}

#[cfg(windows)]
fn system_toggle(button: Button, down: bool) {
    use winapi::um::winuser::mouse_event;
    unsafe {
        mouse_event(mouse_event_for_button(button, down), 0, 0, 0, 0);
    };
}

#[cfg(windows)]
fn system_scroll(direction: ScrollDirection, clicks: u32) {
    use winapi::um::winuser::{mouse_event, MOUSEEVENTF_WHEEL, WHEEL_DELTA};
    unsafe {
        let distance: DWORD = WHEEL_DELTA as DWORD * clicks as DWORD;
        let units: DWORD = if direction == ScrollDirection::Up {
            distance
        } else {
            std::u32::MAX - (distance - 1)
        };
        mouse_event(MOUSEEVENTF_WHEEL, 0, 0, units, 0);
    };
}

#[cfg(target_os = "linux")]
impl From<Button> for XButton {
    fn from(button: Button) -> XButton {
        match button {
            Button::Left => X_BUTTON_LEFT,
            Button::Middle => X_BUTTON_MIDDLE,
            Button::Right => X_BUTTON_RIGHT,
        }
    }
}

#[cfg(target_os = "linux")]
impl From<ScrollDirection> for XButton {
    fn from(direction: ScrollDirection) -> XButton {
        match direction {
            ScrollDirection::Up => X_BUTTON_SCROLL_UP,
            ScrollDirection::Down => X_BUTTON_SCROLL_DOWN,
        }
    }
}

#[cfg(target_os = "linux")]
fn system_move_to(point: Point) {
    use scopeguard::guard;
    internal::X_MAIN_DISPLAY.with(|display| unsafe {
        let scaled_point = point.scaled(screen::scale()).round();
        let root_window = guard(x11::xlib::XDefaultRootWindow(display.as_ptr()), |w| {
            x11::xlib::XDestroyWindow(display.as_ptr(), w);
        });
        x11::xlib::XWarpPointer(
            display.as_ptr(),
            0,
            *root_window,
            0,
            0,
            0,
            0,
            scaled_point.x as i32,
            scaled_point.y as i32,
        );
        x11::xlib::XFlush(display.as_ptr());
    });
}

#[cfg(target_os = "linux")]
fn system_location() -> Point {
    internal::X_MAIN_DISPLAY.with(|display| unsafe {
        let root_window = x11::xlib::XDefaultRootWindow(display.as_ptr());
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut unused_a: x11::xlib::Window = 0;
        let mut unused_b: x11::xlib::Window = 0;
        let mut unused_c: i32 = 0;
        let mut unused_d: i32 = 0;
        let mut unused_e: u32 = 0;
        x11::xlib::XQueryPointer(
            display.as_ptr(),
            root_window,
            &mut unused_a,
            &mut unused_b,
            &mut x,
            &mut y,
            &mut unused_c,
            &mut unused_d,
            &mut unused_e,
        );
        Point::from_pixel(f64::from(x), f64::from(y), screen::scale())
    })
}

#[cfg(target_os = "linux")]
fn send_button_event(display: *mut x11::xlib::Display, button: XButton, down: bool) {
    unsafe {
        XTestFakeButtonEvent(display, button, down as i32, x11::xlib::CurrentTime);
        x11::xlib::XFlush(display);
    };
}

#[cfg(target_os = "linux")]
fn system_toggle(button: Button, down: bool) {
    internal::X_MAIN_DISPLAY.with(|display| {
        send_button_event(display.as_ptr(), XButton::from(button), down);
    });
}

#[cfg(target_os = "linux")]
fn system_scroll(direction: ScrollDirection, clicks: u32) {
    internal::X_MAIN_DISPLAY.with(|display| {
        for _ in 0..clicks {
            send_button_event(display.as_ptr(), XButton::from(direction), true);
            send_button_event(display.as_ptr(), XButton::from(direction), false);
        }
    });
}

#[cfg(target_os = "linux")]
type XButton = u32;

#[cfg(target_os = "linux")]
const X_BUTTON_LEFT: XButton = 1;
#[cfg(target_os = "linux")]
const X_BUTTON_MIDDLE: XButton = 2;
#[cfg(target_os = "linux")]
const X_BUTTON_RIGHT: XButton = 3;
#[cfg(target_os = "linux")]
const X_BUTTON_SCROLL_UP: XButton = 4;
#[cfg(target_os = "linux")]
const X_BUTTON_SCROLL_DOWN: XButton = 5;

#[cfg(target_os = "linux")]
extern "C" {
    fn XTestFakeButtonEvent(
        display: *mut x11::xlib::Display,
        button: u32,
        is_press: i32,
        delay: x11::xlib::Time,
    ) -> i32;
}

#[cfg(test)]
mod tests {
    use geometry::Point;
    use mouse;
    use rand::{thread_rng, Rng};
    use screen;

    #[test]
    fn test_move_to() {
        let size = screen::size();
        let scale = screen::scale();
        let mut rng = thread_rng();
        for _ in 0..100 {
            let x: f64 = rng.gen_range(0.0, size.width - 1.0);
            let y: f64 = rng.gen_range(0.0, size.height - 1.0);
            let target = round_pt_nearest_hundredth(Point::new(x, y));
            mouse::move_to(target).expect("mouse::move_to call failed");
            std::thread::sleep(std::time::Duration::from_millis(10));
            let result = mouse::location();
            assert_eq!(
                target.scaled(scale).round(),
                result.scaled(scale).round(),
                "{} does not map to same pixel as {} for scale {} at size {}",
                target,
                result,
                scale,
                size
            );
        }
    }

    fn round_nearest_hundredth(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    fn round_pt_nearest_hundredth(pt: Point) -> Point {
        Point::new(round_nearest_hundredth(pt.x), round_nearest_hundredth(pt.y))
    }
}
