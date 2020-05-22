extern crate rs_lib;

use deno_core::plugin_api::Buf;
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;
use futures::future::FutureExt;

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
  interface.register_op("type", op_type);
  interface.register_op("alert", op_alert);
}

fn op_type(
  _interface: &mut dyn Interface,
  data: &[u8],
  zero_copy: Option<ZeroCopyBuf>,
) -> Op {
  let data_str = std::str::from_utf8(&data[..]).unwrap().to_string();

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
      rs_lib::key::type_string(&data_str, &[], 200., 0.);
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

fn op_alert(
  _interface: &mut dyn Interface,
  data: &[u8],
  zero_copy: Option<ZeroCopyBuf>,
) -> Op {
  let data_str = std::str::from_utf8(&data[..]).unwrap().to_string();

  let fut = async move {
    if let Some(buf) = zero_copy {
      let buf_str = std::str::from_utf8(&buf[..]).unwrap();
      println!(
        "Alerting... data: {}",
        data_str
      );
    }
    let (tx, rx) = futures::channel::oneshot::channel::<Result<(), ()>>();
    std::thread::spawn(move || {
      let _ = rs_lib::alert::alert(&data_str, None, None, None);
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
