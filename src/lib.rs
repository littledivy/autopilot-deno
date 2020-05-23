// lib.rs
// Copyright 2020 Divy Srivastava
//
//! autopilot-deno-rs is the rust automation library behind autopilot-deno

extern crate rs_lib;

//! use deno_core and futures
use deno_core::plugin_api::Buf;
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;
use futures::future::FutureExt;

//! use serde
use serde::Deserialize;
use serde::Serialize;

use rs_lib::geometry::{Point, Rect, Size};
use std::path::Path;

//! register all ops here
#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
  interface.register_op("type", op_type);
  interface.register_op("alert", op_alert);
  interface.register_op("screenSize", op_screen_size);
  interface.register_op("moveMouse", op_move_mouse);
  interface.register_op("screenshot", op_screen_shot);
}

//! deno bindings for `type`
fn op_type(
  _interface: &mut dyn Interface,
  data: &[u8],
  zero_copy: Option<ZeroCopyBuf>,
) -> Op {
  /// convert arg to string
  let data_str = std::str::from_utf8(&data[..]).unwrap().to_string();

  /// in case, we need a optional functionality in future
  let fut = async move {
    if let Some(buf) = zero_copy {
      let buf_str = std::str::from_utf8(&buf[..]).unwrap();
      println!(
        "Typing... data: {}",
        data_str
      );
    }
    let (tx, rx) = futures::channel::oneshot::channel::<Result<(), ()>>();
    std::thread::spawn(move || {
      /// call type_string
      rs_lib::key::type_string(&data_str, &[], 200., 0.);
      std::thread::sleep(std::time::Duration::from_secs(1));
      tx.send(Ok(())).unwrap();
  });
    assert!(rx.await.is_ok());

    /// return true
    let result = b"true";
    let result_box: Buf = Box::new(*result);
    result_box
  };

  Op::Async(fut.boxed())
}

/// struct for options used by Alert
#[derive(Deserialize)]
struct AlertOptions {
    msg: str,
    title: str
}

//! deno bindings for `alert`
fn op_alert(
  _interface: &mut dyn Interface,
  data: &[u8],
  zero_copy: Option<ZeroCopyBuf>,
) -> Op {
  let data_str = std::str::from_utf8(&data[..]).unwrap().to_string();

  let params: AlertOptions = serde_json::from_slice(data).unwrap();

  let fut = async move {
    if let Some(buf) = zero_copy {
      let buf_str = std::str::from_utf8(&buf[..]).unwrap();
      println!(
        "Alerting... data: {}",
        data_str
      );
    }
    let (tx, rx) = futures::channel::oneshot::channel::<Result<(), ()>>();
    let _ = rs_lib::alert::alert(&params.msg as &str, &params.title, None, None);
    std::thread::spawn(move || {
      std::thread::sleep(std::time::Duration::from_secs(1));
      tx.send(Ok(())).unwrap();
    });
    assert!(rx.await.is_ok());
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
    let data_str = std::str::from_utf8(&data[..]).unwrap();
    let buf_str = std::str::from_utf8(&buf[..]).unwrap();
    println!(
      "Getting Screen Size..."
    );
  }
  let result = rs_lib::screen::size();

  response.height = result.height;
  response.width = result.width;

  let result_box: Buf = serde_json::to_vec(&response).unwrap().into_boxed_slice();
  Op::Sync(result_box)
}

#[derive(Deserialize)]
struct MousePostition {
    x: f64,
    y: f64
}

fn op_move_mouse(
  _interface: &mut dyn Interface,
  data: &[u8],
  zero_copy: Option<ZeroCopyBuf>,
) -> Op {

  let params: MousePostition = serde_json::from_slice(data).unwrap();

  if let Some(buf) = zero_copy {
    let data_str = std::str::from_utf8(&data[..]).unwrap();
    let buf_str = std::str::from_utf8(&buf[..]).unwrap();
    println!(
      "Moving mouse..."
    );
  }
  rs_lib::mouse::move_to(rs_lib::geometry::Point::new(
            params.x as f64,
            params.y as f64
   )).expect("Unable to move mouse");

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
    let buf_str = std::str::from_utf8(&buf[..]).unwrap();
    println!(
      "Taking screen shot..."
    );
  }
  let bmp_path = Path::new(file!())
       .parent()
       .unwrap()
       .parent()
       .unwrap()
       .join(&data_str);
  let _ = bmp.image
       .save(&bmp_path)
       .expect("Unable to save screenshot");
   let result = b"true";
   let result_box: Buf = Box::new(*result);
   Op::Sync(result_box)
}
