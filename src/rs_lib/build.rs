extern crate pkg_config;

#[cfg(target_os = "macos")]
fn main() {}

#[cfg(target_os = "windows")]
fn main() {}

#[cfg(target_os = "linux")]
fn main() {
    println!("cargo:rustc-flags=-l X11 -l Xtst");
}
