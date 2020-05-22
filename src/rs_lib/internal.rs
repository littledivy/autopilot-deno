// Copyright 2018, 2019, 2020 Michael Sanders
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT License <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
#[cfg(target_os = "macos")]
use geometry::{Point, Rect, Size};

#[cfg(target_os = "macos")]
use core_graphics::base::CGFloat;
#[cfg(target_os = "macos")]
use core_graphics::geometry::{CGPoint, CGRect, CGSize};
#[cfg(target_os = "linux")]
use std;
#[cfg(target_os = "linux")]
use std::ptr::NonNull;
#[cfg(target_os = "linux")]
use x11;

#[cfg(target_os = "macos")]
impl From<Point> for CGPoint {
    fn from(point: Point) -> CGPoint {
        CGPoint::new(point.x as CGFloat, point.y as CGFloat)
    }
}

#[cfg(target_os = "macos")]
impl From<CGPoint> for Point {
    fn from(point: CGPoint) -> Point {
        Point::new(point.x as f64, point.y as f64)
    }
}

#[cfg(target_os = "macos")]
impl From<Size> for CGSize {
    fn from(size: Size) -> CGSize {
        CGSize::new(size.width as CGFloat, size.height as CGFloat)
    }
}

#[cfg(target_os = "macos")]
impl From<CGSize> for Size {
    fn from(size: CGSize) -> Size {
        Size::new(size.width as f64, size.height as f64)
    }
}

#[cfg(target_os = "macos")]
impl From<Rect> for CGRect {
    fn from(rect: Rect) -> CGRect {
        CGRect {
            origin: CGPoint::from(rect.origin),
            size: CGSize::from(rect.size),
        }
    }
}

#[cfg(target_os = "linux")]
thread_local!(pub static X_MAIN_DISPLAY: NonNull<x11::xlib::Display> = unsafe {
    let display = x11::xlib::XOpenDisplay(std::ptr::null());
    if display.is_null() {
        panic!("Can't open X display. Is it currently running?");
    }
    NonNull::new_unchecked(display)
});

#[cfg(target_os = "linux")]
thread_local!(pub static X_SCALE_FACTOR: f64 = {
    use std::ffi::{CString, CStr};
    use libc;
    // From https://github.com/glfw/glfw/issues/1019#issuecomment-302772498
    X_MAIN_DISPLAY.with(|display| unsafe {
        let screen = x11::xlib::XDefaultScreen(display.as_ptr());
        let width = f64::from(x11::xlib::XDisplayWidth(display.as_ptr(), screen));
        let width_mm = f64::from(x11::xlib::XDisplayWidthMM(display.as_ptr(), screen));

        // Default to display-wide DPI if Xft.dpi is unset.
        let mut dpi = width * 25.4 / width_mm;

        // Prefer value set in xrdb.
        let rms = x11::xlib::XResourceManagerString(display.as_ptr());
        if !rms.is_null() {
            let db = x11::xlib::XrmGetStringDatabase(rms);
            if !db.is_null() {
                defer!({
                    x11::xlib::XrmDestroyDatabase(db);
                });
                let mut value = x11::xlib::XrmValue{
                    size: 0,
                    addr: std::ptr::null_mut(),
                };

                let mut value_type: *mut libc::c_char = std::ptr::null_mut();
                let dpi_c_str = CString::new("Xft.dpi").unwrap();
                let c_str = CString::new("String").unwrap();
                if x11::xlib::XrmGetResource(
                    db,
                    dpi_c_str.as_ptr(),
                    c_str.as_ptr(),
                    &mut value_type,
                    &mut value
                ) != 0 && !value.addr.is_null() {
                    let value_addr: &CStr = CStr::from_ptr(value.addr);
                    if let Some(parsed_dpi) = value_addr
                        .to_str()
                        .ok()
                        .and_then(|s| s.parse::<f64>().ok()) {
                        dpi = parsed_dpi;
                    }
                }
            }
        }
        let scale = dpi / 96.0;
        (scale * 100.0).floor() / 100.0
    })
});
