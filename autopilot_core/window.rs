// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

// window.rs
use std::process::Command;

#[cfg(target_os = "linux")]
extern crate wmctrl;

#[cfg(target_os = "linux")]
pub fn get_window(index: usize) -> String {
    let windows = wmctrl::get_windows();
    if windows.is_empty() {
      panic!("No windows are open.")
    }
    let win = &windows[index].title();
    format!("{}", win)
}

#[cfg(target_os = "linux")]
pub fn transform_by_index(index: usize, height: u16, width: u16) {
    let mut windows = wmctrl::get_windows();
    let win = &mut windows[index];
    win.transform(wmctrl::Transformation::new(0, 0, height, width));
}

#[cfg(target_os = "macos")]
pub fn transform_by_index(index: usize, height: u16, width: u16) {
    println!("{}", "transform_by_index is not supported for MacOS");
}

#[cfg(target_os = "windows")]
pub fn transform_by_index(index: usize, height: u16, width: u16) {
    println!("{}", "transform_by_index is not supported for Windows");
}

#[cfg(target_os = "macos")]
pub fn get_window(index: usize) -> String {
    println!("{}", "get_window is not supported for MacOS");
    String::from("null")
}

#[cfg(target_os = "windows")]
pub fn get_window(index: usize) -> String {
    println!("{}", "get_window is not supported for Windows");
    String::from("null")
}

// xrandr --listactivemonitors
#[cfg(target_os = "linux")]
pub fn get_active_monitors() -> String {
    let output = Command::new("xrandr")
                     .arg("--listactivemonitors")
                     .output()
                     .expect("failed to execute process");
    let active_monitors_cli = String::from_utf8_lossy(&output.stdout);
    if !active_monitors_cli.is_empty() {
        active_monitors_cli.to_string()
    } else {
        String::from("Monitors: 1")
    }
}

// cmd /C monitors.bat
#[cfg(target_os = "windows")]
pub fn get_active_monitors() -> String {
    let output = Command::new("cmd")
                     .args(&["/C", r#"for /F %%M in (' wmic path Win32_PnPEntity where "Service='monitor' and Status='OK'" get DeviceID /VALUE ') do echo Monitors: %%M"#])
                     .output()
                     .expect("failed to execute process");
    let active_monitors_cli = String::from_utf8_lossy(&output.stdout);
    if !active_monitors_cli.is_empty() {
        active_monitors_cli.to_string()
    } else {
        String::from("Monitors: 1")
    }
}
// system_profiler SPDisplaysDataType
#[cfg(target_os = "macos")]
pub fn get_active_monitors() -> String {
    let output = Command::new("system_profiler")
                     .arg("SPDisplaysDataType")
                     .output()
                     .expect("failed to execute process");
    let active_monitors_cli = String::from_utf8_lossy(&output.stdout);
    if !active_monitors_cli.is_empty() {
        active_monitors_cli.to_string()
    } else {
        String::from("Monitors: 1")
    }
}
