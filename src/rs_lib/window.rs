// window.rs
use std::process::Command;
use std::str::FromStr;
use std::time::Duration;

#[cfg(target_os = "linux")]
extern crate wmctrl;

#[cfg(target_os = "linux")]
pub fn get_window(index: usize) -> String {
    let windows = wmctrl::get_windows();
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

#[cfg(target_os = "linux")]
pub fn get_active_monitors() -> String {
    let output = Command::new("xrandr")
                     .arg("--listactivemonitors")
                     .output()
                     .expect("failed to execute process");
    let active_monitors_cli = String::from_utf8_lossy(&output.stdout);
    println!("{}", String::from_utf8_lossy(&output.stderr));
    if !active_monitors_cli.is_empty() {
        active_monitors_cli.to_string()
    } else {
        String::from("Monitors: 1")
    }
}

#[cfg(target_os = "windows")]
pub fn get_active_monitors() -> String {
    let output = Command::new("cmd")
                     .args(&["/C", "monitors.bat"])
                     .output()
                     .expect("failed to execute process");
    let active_monitors_cli = String::from_utf8_lossy(&output.stdout);
    println!("{}", String::from_utf8_lossy(&output.stderr));
    if !active_monitors_cli.is_empty() {
        active_monitors_cli.to_string()
    } else {
        String::from("Monitors: 1")
    }
}

#[cfg(target_os = "macos")]
pub fn get_active_monitors() -> String {
    println!("{}", "get_monitors is not supported for MacOS");
    String::from("Monitors: 1")
}
