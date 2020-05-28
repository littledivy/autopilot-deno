// window.rs

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
