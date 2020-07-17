// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

extern crate pkg_config;

#[cfg(target_os = "macos")]
fn main() {}

#[cfg(target_os = "windows")]
fn main() {}

#[cfg(target_os = "linux")]
fn main() {
    println!("cargo:rustc-flags=-l X11 -l Xtst");
}
