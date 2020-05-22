// Copyright 2018, 2019, 2020 Michael Sanders
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT License <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
//! This module contains functions for displaying alerts.
#[cfg(target_os = "macos")]
use core_foundation::base::{CFOptionFlags, TCFType};
#[cfg(target_os = "macos")]
use core_foundation::date::CFTimeInterval;
#[cfg(target_os = "macos")]
use core_foundation::string::{CFString, CFStringRef};
#[cfg(target_os = "macos")]
use core_foundation::url::CFURLRef;

use std;

#[derive(Copy, Clone, Debug)]
pub enum Response {
    Default,
    Cancel,
}

/// Displays an alert with the given attributes. If `cancel_button` is not
/// given, only the default button is displayed.
///
/// Due to limitations in the Win32 API, Windows currently replaces
/// `default_button` with "OK" and `cancel_button` (if given) with "Cancel".
/// This may be fixed in a later release.
pub fn alert(
    msg: &str,
    title: Option<&str>,
    default_button: Option<&str>,
    cancel_button: Option<&str>,
) -> Response {
    let title = title.unwrap_or("AutoPilot Alert");
    let default_button = if default_button.unwrap_or("").is_empty() {
        "OK"
    } else {
        default_button.unwrap()
    };

    system_alert(title, msg, default_button, cancel_button).unwrap_or(Response::Cancel)
}

#[cfg(target_os = "macos")]
impl Response {
    fn from(value: CFOptionFlags) -> Option<Response> {
        match value {
            CF_USER_NOTIFICATION_DEFAULT_RESPONSE => Some(Response::Default),
            CF_USER_NOTIFICATION_CANCEL_RESPONSE | CF_USER_NOTIFICATION_ALTERNATE_RESPONSE => {
                Some(Response::Cancel)
            }
            _ => None,
        }
    }
}

#[cfg(target_os = "macos")]
const CF_USER_NOTIFICATION_DEFAULT_RESPONSE: CFOptionFlags = 0;
#[cfg(target_os = "macos")]
const CF_USER_NOTIFICATION_ALTERNATE_RESPONSE: CFOptionFlags = 1;
#[cfg(target_os = "macos")]
const CF_USER_NOTIFICATION_CANCEL_RESPONSE: CFOptionFlags = 3;

#[cfg(target_os = "macos")]
fn system_alert(
    title: &str,
    msg: &str,
    default_button: &str,
    cancel_button: Option<&str>,
) -> Option<Response> {
    let title = CFString::new(title);
    let msg = CFString::new(msg);
    let default_button = CFString::new(default_button);
    let mut flags: CFOptionFlags = 0;
    let resp = unsafe {
        CFUserNotificationDisplayAlert(
            0.0,
            1,
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            title.as_concrete_TypeRef(),
            msg.as_concrete_TypeRef(),
            default_button.as_concrete_TypeRef(),
            if cancel_button.unwrap_or("").is_empty() {
                std::ptr::null()
            } else {
                CFString::new(cancel_button.unwrap()).as_concrete_TypeRef()
            },
            std::ptr::null(),
            &mut flags,
        )
    };

    if resp != 0 {
        None
    } else {
        Response::from(flags)
    }
}

#[cfg(windows)]
fn system_alert(
    title: &str,
    msg: &str,
    _default_button: &str,
    cancel_button: Option<&str>,
) -> Option<Response> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use winapi::um::winuser::{MessageBoxW, IDOK, MB_OK, MB_OKCANCEL};
    let title_wide: Vec<u16> = OsStr::new(title).encode_wide().chain(once(0)).collect();
    let msg_wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let prompt_type = if cancel_button.is_none() {
        MB_OK
    } else {
        MB_OKCANCEL
    };
    let ret = unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            msg_wide.as_ptr(),
            title_wide.as_ptr(),
            prompt_type,
        )
    };

    if ret == IDOK {
        Some(Response::Default)
    } else {
        Some(Response::Cancel)
    }
}

#[cfg(target_os = "linux")]
fn system_alert(
    title: &str,
    msg: &str,
    default_button: &str,
    cancel_button: Option<&str>,
) -> Option<Response> {
    let button_list: &str = &cancel_button
        .map(|cancel_text| format!("{}:2,{}:3", default_button, cancel_text))
        .unwrap_or_else(|| format!("{}:2", default_button));
    let args = [
        msg,
        "-title",
        title,
        "-center",
        "-buttons",
        button_list,
        "-default",
        default_button,
    ];
    let message_programs = ["gmessage", "gxmessage", "kmessage", "xmessage"];
    for program in &message_programs {
        match std::process::Command::new(program)
            .args(&args)
            .spawn()
            .and_then(std::process::Child::wait_with_output)
        {
            Ok(output) => {
                return output.status.code().and_then({
                    |code| {
                        if code == 2 {
                            Some(Response::Default)
                        } else {
                            Some(Response::Cancel)
                        }
                    }
                })
            }
            _ => continue,
        }
    }

    eprintln!("xmessage or equivalent not found");
    None
}

#[cfg(target_os = "macos")]
extern "C" {
    fn CFUserNotificationDisplayAlert(
        timeout: CFTimeInterval,
        flags: CFOptionFlags,
        iconURL: CFURLRef,
        soundURL: CFURLRef,
        localizationURL: CFURLRef,
        alertHeader: CFStringRef,
        alertMessage: CFStringRef,
        defaultButtonTitle: CFStringRef,
        alternateButtonTitle: CFStringRef,
        otherButtonTitle: CFStringRef,
        responseFlags: *mut CFOptionFlags,
    ) -> i32;
}
