// Copyright 2018, 2019, 2020 Michael Sanders
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT License <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
//! This module contains functions for controlling the keyboard.
extern crate rand;

#[cfg(target_os = "macos")]
use core_graphics::event;
#[cfg(target_os = "macos")]
use core_graphics::event::{CGEvent, CGEventFlags, CGKeyCode};
#[cfg(target_os = "macos")]
use core_graphics::event_source::CGEventSource;
#[cfg(target_os = "macos")]
use core_graphics::event_source::CGEventSourceStateID::HIDSystemState;
#[cfg(target_os = "linux")]
use internal;
#[cfg(target_os = "linux")]
use libc;
#[cfg(target_os = "linux")]
use x11;

use self::rand::Rng;
use std;

/// Device-independent modifier flags.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Flag {
    Shift,
    Control,
    Alt,
    Meta,

    // Special key identifiers.
    Help,
}

/// Device-independent key codes.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KeyCode {
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    LeftArrow,
    Control,
    RightArrow,
    DownArrow,
    End,
    UpArrow,
    PageUp,
    Alt,
    Return,
    PageDown,
    Delete,
    Home,
    Escape,
    Backspace,
    Meta,
    CapsLock,
    Shift,
    Tab,
    Space,
}

pub trait KeyCodeConvertible {
    #[cfg(target_os = "macos")]
    fn code(&self) -> CGKeyCode;
    #[cfg(target_os = "linux")]
    fn code(&self) -> XKeyCode;
    #[cfg(windows)]
    fn code(&self) -> WinKeyCode;
    fn character(&self) -> Option<char> {
        None
    }
    fn flags(&self) -> &[Flag] {
        &[]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Character(pub char);

#[derive(Copy, Clone, Debug)]
pub struct Code(pub KeyCode);

/// Attempts to simulate typing a string at the given WPM, or as fast as
/// possible if the WPM is 0.
pub fn type_string(string: &str, flags: &[Flag], wpm: f64, noise: f64) {
    let cpm = wpm * 5.0;
    let cps = cpm / 60.0;
    let ms_per_character: u64 = if cps == 0.0 {
        0
    } else {
        (1000.0 / cps).round() as u64
    };
    let ms_per_stroke = (ms_per_character as f64 / 2.0).round() as u64;

    for c in string.chars() {
        let tolerance = (noise * ms_per_character as f64).round() as u64;
        let noise = if tolerance > 0 {
            rand::thread_rng().gen_range(0, tolerance)
        } else {
            0
        };

        tap(&Character(c), flags, ms_per_stroke, ms_per_stroke);
        std::thread::sleep(std::time::Duration::from_millis(ms_per_stroke + noise));
    }
}

/// Convenience wrapper around `toggle()` that holds down and then releases the
/// given key and modifier flags. Delay between pressing and releasing the key
/// can be controlled using the `delay_ms` parameter. Delay between pressing and
/// releasing modifiers can be controlled using the `modifier_delay_ms`
/// parameter.
pub fn tap<T: KeyCodeConvertible + Copy>(
    key: &T,
    flags: &[Flag],
    delay_ms: u64,
    modifier_delay_ms: u64,
) {
    toggle(key, true, flags, modifier_delay_ms);
    std::thread::sleep(std::time::Duration::from_millis(delay_ms));
    toggle(key, false, flags, modifier_delay_ms);
}

/// Holds down the given key or keycode if `down` is `true`, or releases it if
/// not. Characters are converted to a keycode corresponding to the current
/// keyboard layout. Delay between pressing and releasing the modifier keys can
/// be controlled using the `modifier_delay_ms` parameter.
pub fn toggle<T: KeyCodeConvertible>(key: &T, down: bool, flags: &[Flag], modifier_delay_ms: u64) {
    let key_flags = key.character().map(flags_for_char).unwrap_or(&[]);
    let mut appended_flags: Vec<Flag> = Vec::with_capacity(flags.len() + key_flags.len());
    appended_flags.extend_from_slice(flags);
    for flag in key_flags.iter() {
        if !flags.contains(flag) {
            appended_flags.push(*flag);
        }
    }
    system_toggle(key, down, &appended_flags, modifier_delay_ms);
}

#[cfg(target_os = "macos")]
fn char_to_key_code(character: char) -> CGKeyCode {
    use core_graphics::event::EventField;
    let source = CGEventSource::new(HIDSystemState).unwrap();
    let event = CGEvent::new_keyboard_event(source, 0, true).unwrap();
    let mut buf = [0; 2];
    event.set_string_from_utf16_unchecked(character.encode_utf16(&mut buf));
    event.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE) as CGKeyCode
}

#[cfg(target_os = "linux")]
fn char_to_key_code(character: char) -> XKeyCode {
    match character {
        ' ' => XKeyCode::from(x11::keysym::XK_space),
        '!' => XKeyCode::from(x11::keysym::XK_exclam),
        '"' => XKeyCode::from(x11::keysym::XK_quotedbl),
        '#' => XKeyCode::from(x11::keysym::XK_numbersign),
        '$' => XKeyCode::from(x11::keysym::XK_dollar),
        '%' => XKeyCode::from(x11::keysym::XK_percent),
        '&' => XKeyCode::from(x11::keysym::XK_ampersand),
        '(' => XKeyCode::from(x11::keysym::XK_parenleft),
        ')' => XKeyCode::from(x11::keysym::XK_parenright),
        '*' => XKeyCode::from(x11::keysym::XK_asterisk),
        '+' => XKeyCode::from(x11::keysym::XK_plus),
        ',' => XKeyCode::from(x11::keysym::XK_comma),
        '-' => XKeyCode::from(x11::keysym::XK_minus),
        '.' => XKeyCode::from(x11::keysym::XK_period),
        '/' => XKeyCode::from(x11::keysym::XK_slash),
        ':' => XKeyCode::from(x11::keysym::XK_colon),
        ';' => XKeyCode::from(x11::keysym::XK_semicolon),
        '<' => XKeyCode::from(x11::keysym::XK_less),
        '=' => XKeyCode::from(x11::keysym::XK_equal),
        '>' => XKeyCode::from(x11::keysym::XK_greater),
        '?' => XKeyCode::from(x11::keysym::XK_question),
        '@' => XKeyCode::from(x11::keysym::XK_at),
        '[' => XKeyCode::from(x11::keysym::XK_bracketleft),
        '\'' => XKeyCode::from(x11::keysym::XK_hyphen),
        '\\' => XKeyCode::from(x11::keysym::XK_backslash),
        '\n' => XKeyCode::from(x11::keysym::XK_Return),
        '\t' => XKeyCode::from(x11::keysym::XK_Tab),
        ']' => XKeyCode::from(x11::keysym::XK_bracketright),
        '^' => XKeyCode::from(x11::keysym::XK_asciicircum),
        '_' => XKeyCode::from(x11::keysym::XK_underscore),
        '`' => XKeyCode::from(x11::keysym::XK_grave),
        '{' => XKeyCode::from(x11::keysym::XK_braceleft),
        '|' => XKeyCode::from(x11::keysym::XK_bar),
        '}' => XKeyCode::from(x11::keysym::XK_braceright),
        '~' => XKeyCode::from(x11::keysym::XK_asciitilde),
        _ => unsafe {
            let mut buf = [0; 2];
            x11::xlib::XStringToKeysym(
                character.encode_utf8(&mut buf).as_ptr() as *const libc::c_char
            ) as XKeyCode
        },
    }
}

#[cfg(target_os = "macos")]
fn flags_for_char<'a>(_character: char) -> &'a [Flag] {
    &[]
}

#[cfg(windows)]
fn flags_for_char<'a>(_character: char) -> &'a [Flag] {
    &[]
}

#[cfg(target_os = "linux")]
fn flags_for_char<'a>(character: char) -> &'a [Flag] {
    const UPPERCASE_CHARACTERS: &[char] = &[
        '!', '#', '$', '%', '&', '(', ')', '*', '+', ':', '<', '>', '?', '@', '{', '|', '}', '~',
        '_', '^', '"',
    ];
    if character.is_uppercase() || UPPERCASE_CHARACTERS.contains(&character) {
        &[Flag::Shift]
    } else {
        &[]
    }
}

impl KeyCodeConvertible for Character {
    fn character(&self) -> Option<char> {
        Some(self.0)
    }

    #[cfg(target_os = "macos")]
    fn code(&self) -> CGKeyCode {
        char_to_key_code(self.0)
    }

    #[cfg(windows)]
    fn code(&self) -> WinKeyCode {
        panic!("Unsupported OS")
    }

    #[cfg(target_os = "linux")]
    fn code(&self) -> XKeyCode {
        char_to_key_code(self.0)
    }
}

impl KeyCodeConvertible for Code {
    #[cfg(target_os = "macos")]
    fn code(&self) -> CGKeyCode {
        CGKeyCode::from(self.0)
    }

    #[cfg(windows)]
    fn code(&self) -> WinKeyCode {
        WinKeyCode::from(self.0)
    }

    #[cfg(target_os = "linux")]
    fn code(&self) -> XKeyCode {
        XKeyCode::from(self.0)
    }
}

#[cfg(target_os = "macos")]
impl From<Flag> for CGEventFlags {
    fn from(flag: Flag) -> CGEventFlags {
        match flag {
            Flag::Shift => event::CGEventFlags::CGEventFlagShift,
            Flag::Control => event::CGEventFlags::CGEventFlagControl,
            Flag::Alt => event::CGEventFlags::CGEventFlagAlternate,
            Flag::Meta => event::CGEventFlags::CGEventFlagCommand,
            Flag::Help => event::CGEventFlags::CGEventFlagHelp,
        }
    }
}

#[cfg(target_os = "macos")]
impl From<KeyCode> for CGKeyCode {
    fn from(code: KeyCode) -> CGKeyCode {
        match code {
            KeyCode::F1 => event::KeyCode::F1,
            KeyCode::F2 => event::KeyCode::F2,
            KeyCode::F3 => event::KeyCode::F3,
            KeyCode::F4 => event::KeyCode::F4,
            KeyCode::F5 => event::KeyCode::F5,
            KeyCode::F6 => event::KeyCode::F6,
            KeyCode::F7 => event::KeyCode::F7,
            KeyCode::F8 => event::KeyCode::F8,
            KeyCode::F9 => event::KeyCode::F9,
            KeyCode::F10 => event::KeyCode::F10,
            KeyCode::F11 => event::KeyCode::F11,
            KeyCode::F12 => event::KeyCode::F12,
            KeyCode::F13 => event::KeyCode::F13,
            KeyCode::F14 => event::KeyCode::F14,
            KeyCode::F15 => event::KeyCode::F15,
            KeyCode::F16 => event::KeyCode::F16,
            KeyCode::F17 => event::KeyCode::F17,
            KeyCode::F18 => event::KeyCode::F18,
            KeyCode::F19 => event::KeyCode::F19,
            KeyCode::F20 => event::KeyCode::F20,
            KeyCode::F21 | KeyCode::F22 | KeyCode::F23 | KeyCode::F24 => 0,
            KeyCode::LeftArrow => event::KeyCode::LEFT_ARROW,
            KeyCode::Control => event::KeyCode::CONTROL,
            KeyCode::RightArrow => event::KeyCode::RIGHT_ARROW,
            KeyCode::DownArrow => event::KeyCode::DOWN_ARROW,
            KeyCode::End => event::KeyCode::END,
            KeyCode::UpArrow => event::KeyCode::UP_ARROW,
            KeyCode::PageUp => event::KeyCode::PAGE_UP,
            KeyCode::Alt => event::KeyCode::OPTION,
            KeyCode::Return => event::KeyCode::RETURN,
            KeyCode::PageDown => event::KeyCode::PAGE_DOWN,
            KeyCode::Delete => event::KeyCode::DELETE,
            KeyCode::Home => event::KeyCode::HOME,
            KeyCode::Escape => event::KeyCode::ESCAPE,
            KeyCode::Backspace => event::KeyCode::DELETE,
            KeyCode::Meta => event::KeyCode::COMMAND,
            KeyCode::CapsLock => event::KeyCode::CAPS_LOCK,
            KeyCode::Shift => event::KeyCode::SHIFT,
            KeyCode::Tab => event::KeyCode::TAB,
            KeyCode::Space => event::KeyCode::SPACE,
        }
    }
}

#[cfg(target_os = "macos")]
fn cg_event_mask_for_flags(flags: &[Flag]) -> CGEventFlags {
    flags
        .iter()
        .map(|&x| CGEventFlags::from(x))
        .fold(event::CGEventFlags::CGEventFlagNull, |x, y| {
            x | y as CGEventFlags
        })
}

#[cfg(target_os = "macos")]
fn system_toggle<T: KeyCodeConvertible>(
    key: &T,
    down: bool,
    flags: &[Flag],
    _modifier_delay_ms: u64,
) {
    use core_graphics::event::CGEventType::*;
    use core_graphics::event::{CGEventTapLocation, CGEventType};
    let source = CGEventSource::new(HIDSystemState).unwrap();

    if flags.is_empty() {
        if let Some(character) = key.character() {
            let mut buf = [0; 2];
            let event = CGEvent::new_keyboard_event(source, 0, down).unwrap();
            event.set_string_from_utf16_unchecked(character.encode_utf16(&mut buf));
            event.post(CGEventTapLocation::HID);
            return;
        }
    }

    let code = key.code();
    if code != 0 {
        let event = CGEvent::new_keyboard_event(source, code, down).unwrap();
        let event_type: CGEventType = if down { KeyDown } else { KeyUp };
        event.set_type(event_type);
        event.set_flags(cg_event_mask_for_flags(flags));
        event.post(CGEventTapLocation::HID);
    }
}

#[cfg(windows)]
type WinKeyCode = i32;

#[cfg(windows)]
impl From<Flag> for WinKeyCode {
    fn from(flag: Flag) -> WinKeyCode {
        use winapi::um::winuser;
        let win_code = match flag {
            Flag::Shift => winuser::VK_SHIFT,
            Flag::Control => winuser::VK_CONTROL,
            Flag::Alt => winuser::VK_MENU,
            Flag::Meta => winuser::VK_LWIN,
            Flag::Help => winuser::VK_HELP,
        };
        win_code as WinKeyCode
    }
}

#[cfg(windows)]
impl From<KeyCode> for WinKeyCode {
    fn from(code: KeyCode) -> WinKeyCode {
        use winapi::um::winuser;
        let win_code = match code {
            KeyCode::F1 => winuser::VK_F1,
            KeyCode::F2 => winuser::VK_F2,
            KeyCode::F3 => winuser::VK_F3,
            KeyCode::F4 => winuser::VK_F4,
            KeyCode::F5 => winuser::VK_F5,
            KeyCode::F6 => winuser::VK_F6,
            KeyCode::F7 => winuser::VK_F7,
            KeyCode::F8 => winuser::VK_F8,
            KeyCode::F9 => winuser::VK_F9,
            KeyCode::F10 => winuser::VK_F10,
            KeyCode::F11 => winuser::VK_F11,
            KeyCode::F12 => winuser::VK_F12,
            KeyCode::F13 => winuser::VK_F13,
            KeyCode::F14 => winuser::VK_F14,
            KeyCode::F15 => winuser::VK_F15,
            KeyCode::F16 => winuser::VK_F16,
            KeyCode::F17 => winuser::VK_F17,
            KeyCode::F18 => winuser::VK_F18,
            KeyCode::F19 => winuser::VK_F19,
            KeyCode::F20 => winuser::VK_F20,
            KeyCode::F21 => winuser::VK_F21,
            KeyCode::F22 => winuser::VK_F22,
            KeyCode::F23 => winuser::VK_F23,
            KeyCode::F24 => winuser::VK_F24,
            KeyCode::LeftArrow => winuser::VK_LEFT,
            KeyCode::Control => winuser::VK_CONTROL,
            KeyCode::RightArrow => winuser::VK_RIGHT,
            KeyCode::DownArrow => winuser::VK_DOWN,
            KeyCode::End => winuser::VK_END,
            KeyCode::UpArrow => winuser::VK_UP,
            KeyCode::PageUp => winuser::VK_PRIOR,
            KeyCode::Alt => winuser::VK_MENU,
            KeyCode::Return => winuser::VK_RETURN,
            KeyCode::PageDown => winuser::VK_NEXT,
            KeyCode::Delete => winuser::VK_DELETE,
            KeyCode::Home => winuser::VK_HOME,
            KeyCode::Escape => winuser::VK_ESCAPE,
            KeyCode::Backspace => winuser::VK_BACK,
            KeyCode::Meta => winuser::VK_LWIN,
            KeyCode::CapsLock => winuser::VK_CAPITAL,
            KeyCode::Shift => winuser::VK_SHIFT,
            KeyCode::Tab => winuser::VK_TAB,
            KeyCode::Space => winuser::VK_SPACE,
        };
        win_code as WinKeyCode
    }
}

#[cfg(windows)]
fn win_send_key_event(keycode: WinKeyCode, down: bool, delay_ms: u64) {
    use winapi::um::winuser::{keybd_event, KEYEVENTF_KEYUP};
    let flags = if down { 0 } else { KEYEVENTF_KEYUP };
    unsafe { keybd_event(keycode as u8, 0, flags, 0) };
    std::thread::sleep(std::time::Duration::from_millis(delay_ms));
}

#[cfg(windows)]
fn system_toggle<T: KeyCodeConvertible>(
    key: &T,
    down: bool,
    flags: &[Flag],
    modifier_delay_ms: u64,
) {
    use winapi::um::winuser::{
        SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE,
    };
    for &flag in flags.iter() {
        win_send_key_event(WinKeyCode::from(flag), down, modifier_delay_ms);
    }
    if let Some(character) = key.character() {
        let flags = if down { 0 } else { KEYEVENTF_KEYUP };
        let mut buf = [0; 2];
        for word in character.encode_utf16(&mut buf) {
            let mut input = INPUT {
                type_: INPUT_KEYBOARD,
                u: unsafe {
                    std::mem::transmute_copy(&KEYBDINPUT {
                        wVk: 0,
                        wScan: *word,
                        dwFlags: KEYEVENTF_UNICODE | flags,
                        time: 0,
                        dwExtraInfo: 0,
                    })
                },
            };
            unsafe {
                SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
            }
        }
    } else {
        win_send_key_event(key.code(), down, 0);
    }
}

#[cfg(target_os = "linux")]
type XKeyCode = u64;

#[cfg(target_os = "linux")]
impl From<Flag> for XKeyCode {
    fn from(flag: Flag) -> XKeyCode {
        let x_code = match flag {
            Flag::Shift => x11::keysym::XK_Shift_L,
            Flag::Control => x11::keysym::XK_Control_L,
            Flag::Alt => x11::keysym::XK_Alt_L,
            Flag::Meta => x11::keysym::XK_Meta_L,
            Flag::Help => x11::keysym::XK_Help,
        };
        XKeyCode::from(x_code)
    }
}

#[cfg(target_os = "linux")]
impl From<KeyCode> for XKeyCode {
    fn from(code: KeyCode) -> XKeyCode {
        let x_code = match code {
            KeyCode::F1 => x11::keysym::XK_F1,
            KeyCode::F2 => x11::keysym::XK_F2,
            KeyCode::F3 => x11::keysym::XK_F3,
            KeyCode::F4 => x11::keysym::XK_F4,
            KeyCode::F5 => x11::keysym::XK_F5,
            KeyCode::F6 => x11::keysym::XK_F6,
            KeyCode::F7 => x11::keysym::XK_F7,
            KeyCode::F8 => x11::keysym::XK_F8,
            KeyCode::F9 => x11::keysym::XK_F9,
            KeyCode::F10 => x11::keysym::XK_F10,
            KeyCode::F11 => x11::keysym::XK_F11,
            KeyCode::F12 => x11::keysym::XK_F12,
            KeyCode::F13 => x11::keysym::XK_F13,
            KeyCode::F14 => x11::keysym::XK_F14,
            KeyCode::F15 => x11::keysym::XK_F15,
            KeyCode::F16 => x11::keysym::XK_F16,
            KeyCode::F17 => x11::keysym::XK_F17,
            KeyCode::F18 => x11::keysym::XK_F18,
            KeyCode::F19 => x11::keysym::XK_F19,
            KeyCode::F20 => x11::keysym::XK_F20,
            KeyCode::F21 => x11::keysym::XK_F21,
            KeyCode::F22 => x11::keysym::XK_F22,
            KeyCode::F23 => x11::keysym::XK_F23,
            KeyCode::F24 => x11::keysym::XK_F24,
            KeyCode::LeftArrow => x11::keysym::XK_Left,
            KeyCode::Control => x11::keysym::XK_Control_L,
            KeyCode::RightArrow => x11::keysym::XK_Right,
            KeyCode::DownArrow => x11::keysym::XK_Down,
            KeyCode::End => x11::keysym::XK_End,
            KeyCode::UpArrow => x11::keysym::XK_Up,
            KeyCode::PageUp => x11::keysym::XK_Page_Up,
            KeyCode::Alt => x11::keysym::XK_Alt_L,
            KeyCode::Return => x11::keysym::XK_Return,
            KeyCode::PageDown => x11::keysym::XK_Page_Down,
            KeyCode::Delete => x11::keysym::XK_Delete,
            KeyCode::Home => x11::keysym::XK_Home,
            KeyCode::Escape => x11::keysym::XK_Escape,
            KeyCode::Backspace => x11::keysym::XK_Delete,
            KeyCode::Meta => x11::keysym::XK_Meta_L,
            KeyCode::CapsLock => x11::keysym::XK_Caps_Lock,
            KeyCode::Shift => x11::keysym::XK_Shift_L,
            KeyCode::Tab => x11::keysym::XK_Tab,
            KeyCode::Space => x11::keysym::XK_space,
        };
        XKeyCode::from(x_code)
    }
}

#[cfg(target_os = "linux")]
fn x_send_key_event(
    display: *mut x11::xlib::Display,
    keycode: XKeyCode,
    down: bool,
    delay_ms: u64,
) {
    unsafe {
        XTestFakeKeyEvent(
            display,
            x11::xlib::XKeysymToKeycode(display, keycode as libc::c_ulong),
            down as i32,
            x11::xlib::CurrentTime,
        );
        x11::xlib::XFlush(display);
    };

    std::thread::sleep(std::time::Duration::from_millis(delay_ms));
}

#[cfg(target_os = "linux")]
fn system_toggle<T: KeyCodeConvertible>(
    key: &T,
    down: bool,
    flags: &[Flag],
    modifier_delay_ms: u64,
) {
    internal::X_MAIN_DISPLAY.with(|display| {
        for &flag in flags.iter() {
            x_send_key_event(
                display.as_ptr(),
                XKeyCode::from(flag),
                down,
                modifier_delay_ms,
            );
        }
        x_send_key_event(display.as_ptr(), key.code(), down, 0);
    })
}

#[cfg(target_os = "linux")]
extern "C" {
    fn XTestFakeKeyEvent(
        display: *mut x11::xlib::Display,
        keycode: u8,
        is_press: i32,
        delay: x11::xlib::Time,
    ) -> i32;
}
