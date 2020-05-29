// window.rs
use std::process::Command;
use std::str::FromStr;
use std::time::Duration;

#[cfg(target_os = "linux")]
extern crate wmctrl;

#[cfg(target_os = "linux")]
pub fn get_window() {
    let windows = wmctrl::get_windows();
    let win = &windows[0].title();
    println!("{}", win);
}

#[cfg(target_os = "macos")]
pub fn get_window() {
    println!("{}", "get_window is not supported for MacOS");
}

#[cfg(target_os = "windows")]
pub fn get_window() {
    println!("{}", "get_window is not supported for Windows");
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
    println!("{}", "get_monitors is not supported for Windows");
    String::from("Monitors: 1")
}

#[cfg(target_os = "macos")]
pub fn get_active_monitors() -> String {
    println!("{}", "get_monitors is not supported for MacOS");
    String::from("Monitors: 1")
}
