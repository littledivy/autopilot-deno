// Copyright 2018, 2019, 2020 Michael Sanders
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT License <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
//! autopilot is a simple, cross-platform GUI automation library for Rust.
pub mod alert;
pub mod bitmap;
pub mod geometry;
mod internal;
pub mod key;
pub mod mouse;
pub mod screen;

extern crate image;
extern crate libc;
extern crate rand;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(target_os = "macos")]
extern crate cocoa;
#[cfg(target_os = "macos")]
extern crate core_foundation;
#[cfg(target_os = "macos")]
extern crate core_graphics;
#[cfg(windows)]
extern crate winapi;
#[cfg(target_os = "linux")]
#[macro_use(defer)]
extern crate scopeguard;
#[cfg(windows)]
extern crate scopeguard;
#[cfg(target_os = "linux")]
extern crate x11;
