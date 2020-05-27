// lib.rs
// Copyright 2020 Divy Srivastava
//
//! autopilot-deno-rs is the rust automation library behind autopilot-deno

extern crate rs_lib;

// use deno_core and futures
use deno_core::plugin_api::Buf;
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;
use futures::future::FutureExt;

#[cfg(target_os = "linux")]
extern crate wmctrl;
use wmctrl::{Window};

// use serde
use serde::Deserialize;
use serde::Serialize;

use std::path::Path;

// register all ops here
#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {

    let windows = wmctrl::get_windows();

    let win = &windows[0].title();

    println!("{}", win);

    interface.register_op("type", op_type);
    interface.register_op("alert", op_alert);
    interface.register_op("screenSize", op_screen_size);
    interface.register_op("screenScale", op_screen_scale);
    interface.register_op("moveMouse", op_move_mouse);
    interface.register_op("screenshot", op_screen_shot);
    interface.register_op("click", op_click);
    interface.register_op("tap", op_tap);
    interface.register_op("scroll", op_scroll);
    interface.register_op("mousePostition", op_mouse_pos);
    interface.register_op("pixelColor", op_mouse_pixel_color);
    interface.register_op("toggleKey", op_toggle_key);
    interface.register_op("pointVisible", op_point_visible);
}

// deno bindings for `type`
fn op_type(_interface: &mut dyn Interface, data: &[u8], zero_copy: Option<ZeroCopyBuf>) -> Op {
    // convert arg to string
    let data_str = std::str::from_utf8(&data[..]).unwrap().to_string();

    // in case, we need a optional functionality in future
    let fut = async move {
        if let Some(buf) = zero_copy {
            let _buf_str = std::str::from_utf8(&buf[..]).unwrap();
            println!("Typing... data: {}", data_str);
        }
        let (tx, rx) = futures::channel::oneshot::channel::<Result<(), ()>>();
        std::thread::spawn(move || {
            // call type_string
            rs_lib::key::type_string(&data_str, &[], 200., 0.);
            std::thread::sleep(std::time::Duration::from_secs(1));
            tx.send(Ok(())).unwrap();
        });
        assert!(rx.await.is_ok());

        // return true
        let result = b"true";
        let result_box: Buf = Box::new(*result);
        result_box
    };

    Op::Async(fut.boxed())
}

#[derive(Serialize)]
struct Resp {
    height: f64,
    width: f64,
}

fn op_screen_size(
    _interface: &mut dyn Interface,
    data: &[u8],
    zero_copy: Option<ZeroCopyBuf>,
) -> Op {
    let mut response = Resp {
        width: 1000_f64,
        height: 1000_f64,
    };
    if let Some(buf) = zero_copy {
        let _data_str = std::str::from_utf8(&data[..]).unwrap();
        let _buf_str = std::str::from_utf8(&buf[..]).unwrap();
        println!("Getting Screen Size...");
    }
    let result = rs_lib::screen::size();

    response.height = result.height;
    response.width = result.width;

    let result_box: Buf = serde_json::to_vec(&response).unwrap().into_boxed_slice();
    Op::Sync(result_box)
}

#[derive(Serialize)]
struct ScaleResponse {
    scale: f64,
}

fn op_screen_scale(
    _interface: &mut dyn Interface,
    data: &[u8],
    zero_copy: Option<ZeroCopyBuf>,
) -> Op {
    let mut response = ScaleResponse {
        scale: 1000_f64
    };
    if let Some(buf) = zero_copy {
        let _data_str = std::str::from_utf8(&data[..]).unwrap();
        let _buf_str = std::str::from_utf8(&buf[..]).unwrap();
        println!("Getting Screen Scale...");
    }
    let result = rs_lib::screen::scale();

    response.scale = result;

    let result_box: Buf = serde_json::to_vec(&response).unwrap().into_boxed_slice();
    Op::Sync(result_box)
}

#[derive(Deserialize)]
struct MousePostition {
    x: f64,
    y: f64,
    d: f64,
}

fn op_move_mouse(
    _interface: &mut dyn Interface,
    data: &[u8],
    zero_copy: Option<ZeroCopyBuf>,
) -> Op {
    let params: MousePostition = serde_json::from_slice(data).unwrap();

    if let Some(buf) = zero_copy {
        let _data_str = std::str::from_utf8(&data[..]).unwrap();
        let _buf_str = std::str::from_utf8(&buf[..]).unwrap();
        println!("Moving mouse...");
    }
    rs_lib::mouse::smooth_move(
        rs_lib::geometry::Point::new(params.x as f64, params.y as f64),
        params.d as f64,
    )
    .expect("Unable to move mouse");

    let result = b"true";
    let result_box: Buf = Box::new(*result);
    Op::Sync(result_box)
}

fn op_screen_shot(
    _interface: &mut dyn Interface,
    data: &[u8],
    zero_copy: Option<ZeroCopyBuf>,
) -> Op {
    let data_str = std::str::from_utf8(&data[..]).unwrap().to_string();
    let bmp = rs_lib::bitmap::capture_screen().expect("Unable to capture screen");
    if let Some(buf) = zero_copy {
        let _buf_str = std::str::from_utf8(&buf[..]).unwrap();
        println!("Taking screen shot...");
    }
    let bmp_path = Path::new(file!())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(&data_str);
    let _ = bmp
        .image
        .save(&bmp_path)
        .expect("Unable to save screenshot");
    let result = b"true";
    let result_box: Buf = Box::new(*result);
    Op::Sync(result_box)
}

// struct for options used by Alert
#[derive(Deserialize)]
struct AlertOptions {
    msg: String,
    title: String,
}

// deno bindings for `alert`
fn op_alert(_interface: &mut dyn Interface, data: &[u8], zero_copy: Option<ZeroCopyBuf>) -> Op {
    let params: AlertOptions = serde_json::from_slice(data).unwrap();

    if let Some(buf) = zero_copy {
        let _data_str = std::str::from_utf8(&data[..]).unwrap();
        let _buf_str = std::str::from_utf8(&buf[..]).unwrap();
        println!("Alerting...");
    }
    let _ = rs_lib::alert::alert(&params.msg, &params.title, None, None);

    let result = b"true";
    let result_box: Buf = Box::new(*result);
    Op::Sync(result_box)
}

fn op_click(_interface: &mut dyn Interface, data: &[u8], _zero_copy: Option<ZeroCopyBuf>) -> Op {
    // convert arg to string
    let data_str = std::str::from_utf8(&data[..]).unwrap().to_string();

    if data_str == "left" {
        rs_lib::mouse::click(rs_lib::mouse::Button::Left, 10 as u64);
    }
    if data_str == "right" {
        rs_lib::mouse::click(rs_lib::mouse::Button::Right, 10 as u64);
    }
    if data_str == "middle" {
        rs_lib::mouse::click(rs_lib::mouse::Button::Middle, 10 as u64);
    }

    let result = b"true";
    let result_box: Buf = Box::new(*result);
    Op::Sync(result_box)
}

fn op_scroll(_interface: &mut dyn Interface, data: &[u8], _zero_copy: Option<ZeroCopyBuf>) -> Op {
    // convert arg to string
    let _data_str = std::str::from_utf8(&data[..]).unwrap().to_string();

    rs_lib::mouse::scroll(rs_lib::mouse::ScrollDirection::Up, 5 as u32);

    let result = b"true";
    let result_box: Buf = Box::new(*result);
    Op::Sync(result_box)
}

#[derive(Serialize)]
struct PixelRsp {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

// get mouse pixel color
fn op_mouse_pixel_color(
    _interface: &mut dyn Interface,
    data: &[u8],
    zero_copy: Option<ZeroCopyBuf>,
) -> Op {
    let mut response = PixelRsp {
        r: 0x82u8,
        g: 0x82u8,
        b: 0x82u8,
        a: 0x82u8,
    };
    if let Some(buf) = zero_copy {
        let _data_str = std::str::from_utf8(&data[..]).unwrap();
        let _buf_str = std::str::from_utf8(&buf[..]).unwrap();
        println!("Getting Mouse Pixel Color...");
    }
    let result = rs_lib::screen::get_color(rs_lib::mouse::location());
    let r = result.ok().unwrap();

    response.r = r[0];
    response.g = r[1];
    response.b = r[2];
    response.a = r[3];

    let result_box: Buf = serde_json::to_vec(&response).unwrap().into_boxed_slice();
    Op::Sync(result_box)
}

#[derive(Deserialize)]
struct PointPosition {
    x: f64,
    y: f64
}

// point is visible or not
fn op_point_visible(
    _interface: &mut dyn Interface,
    data: &[u8],
    zero_copy: Option<ZeroCopyBuf>,
) -> Op {

    let params: PointPosition = serde_json::from_slice(data).unwrap();

    if let Some(buf) = zero_copy {
        let _data_str = std::str::from_utf8(&data[..]).unwrap();
        let _buf_str = std::str::from_utf8(&buf[..]).unwrap();
        println!("Moving mouse...");
    }

    let r = rs_lib::screen::is_point_visible(
        rs_lib::geometry::Point::new(params.x as f64, params.y as f64)
    );
    let mut result = b"0";
    if r == true { result = b"1" };
    let result_box: Buf = Box::new(*result);
    Op::Sync(result_box)
}

#[derive(Serialize)]
struct MouseResp {
    x: f64,
    y: f64,
}

// get mouse position
fn op_mouse_pos(_interface: &mut dyn Interface, data: &[u8], zero_copy: Option<ZeroCopyBuf>) -> Op {
    let mut response = MouseResp {
        x: 100_f64,
        y: 100_f64,
    };
    if let Some(buf) = zero_copy {
        let _data_str = std::str::from_utf8(&data[..]).unwrap();
        let _buf_str = std::str::from_utf8(&buf[..]).unwrap();
        println!("Getting Mouse position...");
    }
    let result = rs_lib::mouse::location();

    response.x = result.x;
    response.y = result.y;

    let result_box: Buf = serde_json::to_vec(&response).unwrap().into_boxed_slice();
    Op::Sync(result_box)
}

#[derive(Deserialize)]
struct ToggleOptions {
    key: String,
    down: i32,
}

// toggle a key
fn op_toggle_key(
    _interface: &mut dyn Interface,
    data: &[u8],
    _zero_copy: Option<ZeroCopyBuf>,
) -> Op {
    let _data_str = std::str::from_utf8(&data[..]).unwrap().to_string();

    let params: ToggleOptions = serde_json::from_slice(data).unwrap();

    rs_lib::key::toggle(
        &rs_lib::key::Code(bind_tap(&params.key)),
        params.down != 0,
        &[],
        0. as u64,
    );

    let result = b"true";
    let result_box: Buf = Box::new(*result);
    Op::Sync(result_box)
}

// tap a key
fn op_tap(_interface: &mut dyn Interface, data: &[u8], _zero_copy: Option<ZeroCopyBuf>) -> Op {
    // convert arg to string
    let _data_str = std::str::from_utf8(&data[..]).unwrap().to_string();

    rs_lib::key::tap(
        &rs_lib::key::Code(bind_tap(&_data_str)),
        &[],
        0. as u64,
        0. as u64,
    );

    let result = b"true";
    let result_box: Buf = Box::new(*result);
    Op::Sync(result_box)
}

// the below code is complete shit
fn bind_tap(key: &str) -> rs_lib::key::KeyCode {
    if key == "F1" {
        rs_lib::key::KeyCode::F1
    } else if key == "F2" {
        rs_lib::key::KeyCode::F2
    } else if key == "F3" {
        rs_lib::key::KeyCode::F3
    } else if key == "F4" {
        rs_lib::key::KeyCode::F4
    } else if key == "F5" {
        rs_lib::key::KeyCode::F5
    } else if key == "F6" {
        rs_lib::key::KeyCode::F6
    } else if key == "F7" {
        rs_lib::key::KeyCode::F7
    } else if key == "F8" {
        rs_lib::key::KeyCode::F8
    } else if key == "F9" {
        rs_lib::key::KeyCode::F9
    } else if key == "F10" {
        rs_lib::key::KeyCode::F10
    } else if key == "F11" {
        rs_lib::key::KeyCode::F11
    } else if key == "F12" {
        rs_lib::key::KeyCode::F12
    } else if key == "F13" {
        rs_lib::key::KeyCode::F13
    } else if key == "F14" {
        rs_lib::key::KeyCode::F14
    } else if key == "F15" {
        rs_lib::key::KeyCode::F15
    } else if key == "F16" {
        rs_lib::key::KeyCode::F16
    } else if key == "F17" {
        rs_lib::key::KeyCode::F17
    } else if key == "F18" {
        rs_lib::key::KeyCode::F18
    } else if key == "F19" {
        rs_lib::key::KeyCode::F19
    } else if key == "F20" {
        rs_lib::key::KeyCode::F20
    } else if key == "F21" {
        rs_lib::key::KeyCode::F21
    } else if key == "F22" {
        rs_lib::key::KeyCode::F22
    } else if key == "F23" {
        rs_lib::key::KeyCode::F23
    } else if key == "F24" {
        rs_lib::key::KeyCode::F24
    } else if key == "leftarrow" {
        rs_lib::key::KeyCode::LeftArrow
    } else if key == "control" {
        rs_lib::key::KeyCode::Control
    } else if key == "rightarrow" {
        rs_lib::key::KeyCode::RightArrow
    } else if key == "downarrow" {
        rs_lib::key::KeyCode::DownArrow
    } else if key == "end" {
        rs_lib::key::KeyCode::End
    } else if key == "uparrow" {
        rs_lib::key::KeyCode::UpArrow
    } else if key == "pageup" {
        rs_lib::key::KeyCode::PageUp
    } else if key == "alt" {
        rs_lib::key::KeyCode::Alt
    } else if key == "enter" {
        rs_lib::key::KeyCode::Return
    } else if key == "return" {
        rs_lib::key::KeyCode::Return
    } else if key == "pagedown" {
        rs_lib::key::KeyCode::PageDown
    } else if key == "delete" {
        rs_lib::key::KeyCode::Delete
    } else if key == "home" {
        rs_lib::key::KeyCode::Home
    } else if key == "escape" {
        rs_lib::key::KeyCode::Escape
    } else if key == "backspace" {
        rs_lib::key::KeyCode::Backspace
    } else if key == "back" {
        rs_lib::key::KeyCode::Backspace
    } else if key == "meta" {
        rs_lib::key::KeyCode::Meta
    } else if key == "capslock" {
        rs_lib::key::KeyCode::CapsLock
    } else if key == "shift" {
        rs_lib::key::KeyCode::Shift
    } else if key == "tab" {
        rs_lib::key::KeyCode::Tab
    } else if key == "space" {
        rs_lib::key::KeyCode::Space
    } else {
        rs_lib::key::KeyCode::Return
    }
}
