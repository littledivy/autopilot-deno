use deno_bindgen::deno_bindgen;
use serde::Deserialize;
use std::path::Path;

#[deno_bindgen]
struct StrArg {
    text: String,
}

#[deno_bindgen]
fn op_type(arg: StrArg) {
    autopilot::key::type_string(&arg.text, &[], 200., 0.);
}

#[deno_bindgen]
pub struct KeyToggleParams {
    pub key: KeyCode,
    pub down: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
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
    UpArrow,
    RightArrow,
    DownArrow,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    Backspace,
    Enter,
    Escape,
    Space,
    Meta,
    Alt,
    Control,
    Shift,
    CapsLock,
}

impl From<KeyCode> for autopilot::key::KeyCode {
    fn from(value: KeyCode) -> autopilot::key::KeyCode {
        match value {
            KeyCode::F1 => autopilot::key::KeyCode::F1,
            KeyCode::F2 => autopilot::key::KeyCode::F2,
            KeyCode::F3 => autopilot::key::KeyCode::F3,
            KeyCode::F4 => autopilot::key::KeyCode::F4,
            KeyCode::F5 => autopilot::key::KeyCode::F5,
            KeyCode::F6 => autopilot::key::KeyCode::F6,
            KeyCode::F7 => autopilot::key::KeyCode::F7,
            KeyCode::F8 => autopilot::key::KeyCode::F8,
            KeyCode::F9 => autopilot::key::KeyCode::F9,
            KeyCode::F10 => autopilot::key::KeyCode::F10,
            KeyCode::F11 => autopilot::key::KeyCode::F11,
            KeyCode::F12 => autopilot::key::KeyCode::F12,
            KeyCode::F13 => autopilot::key::KeyCode::F13,
            KeyCode::F14 => autopilot::key::KeyCode::F14,
            KeyCode::F15 => autopilot::key::KeyCode::F15,
            KeyCode::F16 => autopilot::key::KeyCode::F16,
            KeyCode::F17 => autopilot::key::KeyCode::F17,
            KeyCode::F18 => autopilot::key::KeyCode::F18,
            KeyCode::F19 => autopilot::key::KeyCode::F19,
            KeyCode::F20 => autopilot::key::KeyCode::F20,
            KeyCode::F21 => autopilot::key::KeyCode::F21,
            KeyCode::F22 => autopilot::key::KeyCode::F22,
            KeyCode::F23 => autopilot::key::KeyCode::F23,
            KeyCode::F24 => autopilot::key::KeyCode::F24,
            KeyCode::LeftArrow => autopilot::key::KeyCode::LeftArrow,
            KeyCode::UpArrow => autopilot::key::KeyCode::UpArrow,
            KeyCode::RightArrow => autopilot::key::KeyCode::RightArrow,
            KeyCode::DownArrow => autopilot::key::KeyCode::DownArrow,
            KeyCode::Insert => autopilot::key::KeyCode::Insert,
            KeyCode::Delete => autopilot::key::KeyCode::Delete,
            KeyCode::Home => autopilot::key::KeyCode::Home,
            KeyCode::End => autopilot::key::KeyCode::End,
            KeyCode::PageUp => autopilot::key::KeyCode::PageUp,
            KeyCode::PageDown => autopilot::key::KeyCode::PageDown,
            KeyCode::Tab => autopilot::key::KeyCode::Tab,
            KeyCode::Backspace => autopilot::key::KeyCode::Backspace,
            KeyCode::Enter => autopilot::key::KeyCode::Return,
            KeyCode::Escape => autopilot::key::KeyCode::Escape,
            KeyCode::Space => autopilot::key::KeyCode::Space,
            KeyCode::Delete => autopilot::key::KeyCode::Delete,
            KeyCode::Meta => autopilot::key::KeyCode::Meta,
            KeyCode::Alt => autopilot::key::KeyCode::Alt,
            KeyCode::Control => autopilot::key::KeyCode::Control,
            KeyCode::Shift => autopilot::key::KeyCode::Shift,
            KeyCode::CapsLock => autopilot::key::KeyCode::CapsLock,
        }
    }
}

#[deno_bindgen]
fn op_toggle_key(arg: KeyToggleParams) {
    autopilot::key::toggle(&autopilot::key::Code(arg.key.into()), arg.down, &[], 0);
}

// fn op_toggle_key(
//     _state: &mut OpState,
//     arg: KeyToggleParams,
//     zero_copy: Option<ZeroCopyBuf>,
// ) -> Result<(), AnyError> {
//     autopilot::key::toggle(&autopilot::key::Code(arg.key.into()), arg.down, &[], 0);
//     Ok(())
// }

#[deno_bindgen]
fn op_tap(arg: KeyToggleParams) {
    autopilot::key::tap(&autopilot::key::Code(arg.key.into()), &[], 0, 0);
}
// fn op_tap(
//     _state: &mut OpState,
//     arg: KeyToggleParams,
//     zero_copy: Option<ZeroCopyBuf>,
// ) -> Result<(), AnyError> {
//     autopilot::key::tap(&autopilot::key::Code(arg.key.into()), &[], 0, 0);
//     Ok(())
// }

#[deno_bindgen]
pub struct NotifyParams {
    pub title: String,
    pub body: String,
}

#[deno_bindgen]
fn op_notify(arg: NotifyParams) {
    autopilot::notify::notify(&arg.title, &arg.body).unwrap();
}

// fn op_notify(
//     _state: &mut OpState,
//     arg: NotifyParams,
//     zero_copy: Option<ZeroCopyBuf>,
// ) -> Result<(), AnyError> {
//     autopilot::notify::notify(&arg.title, &arg.body);
//     Ok(())
// }

#[deno_bindgen]
pub struct MouseMoveParams {
    pub x: f64,
    pub y: f64,
    pub d: Option<f64>,
}

#[deno_bindgen]
fn op_smooth_mouse_move(arg: MouseMoveParams) {
    autopilot::mouse::smooth_move(autopilot::geometry::Point::new(arg.x, arg.y), arg.d);
}

// fn op_smooth_mouse_move(
//     _state: &mut OpState,
//     arg: MouseMoveParams,
//     zero_copy: Option<ZeroCopyBuf>,
// ) -> Result<(), AnyError> {
//     autopilot::mouse::smooth_move(autopilot::geometry::Point::new(arg.x, arg.y), arg.d);
//     Ok(())
// }

#[deno_bindgen]
fn op_mouse_move(arg: MouseMoveParams) {
    autopilot::mouse::move_to(autopilot::geometry::Point::new(arg.x, arg.y));
}

// fn op_mouse_move(
//     _state: &mut OpState,
//     arg: MouseMoveParams,
//     zero_copy: Option<ZeroCopyBuf>,
// ) -> Result<(), AnyError> {
//     autopilot::mouse::move_to(autopilot::geometry::Point::new(arg.x, arg.y));
//     Ok(())
// }

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MouseClickParams {
     Left,
     Middle,
     Right,
}

impl From<MouseClickParams> for autopilot::mouse::Button {
     fn from(value: MouseClickParams) -> autopilot::mouse::Button {
         match value {
             MouseClickParams::Left => autopilot::mouse::Button::Left,
             MouseClickParams::Middle => autopilot::mouse::Button::Middle,
             MouseClickParams::Right => autopilot::mouse::Button::Right,
         }
     }
}

// TODO: Wrapper for MouseClickParams enum
//       remove when deno_bindgen supports enums. 
#[deno_bindgen]
pub struct MouseClickParamsWrapper {
    params: MouseClickParams
}

#[deno_bindgen]
fn op_mouse_click(arg: MouseClickParamsWrapper) {
    autopilot::mouse::click(autopilot::mouse::Button::from(arg.params), Some(10));
}

// fn op_mouse_click(
//     _state: &mut OpState,
//     arg: MouseClickParams,
//     zero_copy: Option<ZeroCopyBuf>,
// ) -> Result<(), AnyError> {
//     autopilot::mouse::click(autopilot::mouse::Button::from(arg), Some(10));
//     Ok(())
// }

#[deno_bindgen]
fn op_mouse_scroll() {
    // TODO
    autopilot::mouse::scroll(autopilot::mouse::ScrollDirection::Up, 5);
}

// fn op_mouse_scroll(
//     _state: &mut OpState,
//     arg: (),
//     zero_copy: Option<ZeroCopyBuf>,
// ) -> Result<(), AnyError> {
//     autopilot::mouse::scroll(autopilot::mouse::ScrollDirection::Up, 5);
//     Ok(())
// }

#[deno_bindgen]
pub struct Point {
     pub x: f64,
     pub y: f64,
}

#[deno_bindgen]
fn op_mouse_pos_x() -> f64 {
    let pos = autopilot::mouse::location();
    pos.x
}

#[deno_bindgen]
fn op_mouse_pos_y() -> f64 {
    let pos = autopilot::mouse::location();
    pos.y
}

// fn op_mouse_pos(
//     _state: &mut OpState,
//     arg: (),
//     zero_copy: Option<ZeroCopyBuf>,
// ) -> Result<Point, AnyError> {
//     let pos = autopilot::mouse::location();
//     Ok(Point { x: pos.x, y: pos.y })
// }

#[deno_bindgen]
pub struct AlertParams {
     pub msg: String,
     pub title: String,
}

#[deno_bindgen]
fn op_alert(arg: AlertParams) {
    autopilot::alert::alert(&arg.msg, Some(&arg.title), None, None);
} 

// fn op_alert(
//     _state: &mut OpState,
//     arg: AlertParams,
//     zero_copy: Option<ZeroCopyBuf>,
// ) -> Result<(), AnyError> {
//     autopilot::alert::alert(&arg.msg, Some(&arg.title), None, None);
//     Ok(())
// }

#[deno_bindgen]
fn op_screenshot(arg: StrArg) {
    let bitmap = autopilot::bitmap::capture_screen().unwrap();
    let path = Path::new(file!()).parent().unwrap().parent().unwrap().join(&arg.text);
    bitmap.image.save(&path).unwrap();
}

// fn op_screenshot(
//     _state: &mut OpState,
//     arg: String,
//     zero_copy: Option<ZeroCopyBuf>,
// ) -> Result<(), AnyError> {
//     let bitmap = autopilot::bitmap::capture_screen()?;
//     let path = Path::new(file!())
//         .parent()
//         .unwrap()
//         .parent()
//         .unwrap()
//         .join(&arg);
//     bitmap.image.save(&path)?;
//     Ok(())
// }

#[deno_bindgen]
pub struct ScreenSize {
     pub height: f64,
     pub width: f64,
}

#[deno_bindgen]
fn op_screensize_height() -> f64 {
  let size = autopilot::screen::size();
  size.height
}

#[deno_bindgen]
fn op_screensize_width() -> f64 {
  let size = autopilot::screen::size();
  size.width
}

// fn op_screensize(
//     _state: &mut OpState,
//     arg: (),
//     zero_copy: Option<ZeroCopyBuf>,
// ) -> Result<ScreenSize, AnyError> {
//     let size = autopilot::screen::size();
//     let width = size.width;
//     let height = size.height;

//     Ok(ScreenSize { width, height })
// }

// fn op_screenscale(
//     _state: &mut OpState,
//     arg: (),
//     zero_copy: Option<ZeroCopyBuf>,
// ) -> Result<f64, AnyError> {
//     let scale = autopilot::screen::scale();
//     Ok(scale)
// }

#[deno_bindgen]
pub struct Pixel {
   pub r: u8,
   pub g: u8,
   pub b: u8,
   pub a: u8,
}

macro_rules! mouse_px_op {
    ($fn: ident, $idx: literal) => {
        #[deno_bindgen]
        fn $fn() -> u8 {
            let pixel = autopilot::screen::get_color(autopilot::mouse::location()).unwrap();
            pixel[$idx]
        }
    }
}

mouse_px_op!(op_mouse_pixel_color_r, 0);
mouse_px_op!(op_mouse_pixel_color_g, 1);
mouse_px_op!(op_mouse_pixel_color_b, 2);
mouse_px_op!(op_mouse_pixel_color_a, 3);

// fn op_mouse_pixel_color(
//     _state: &mut OpState,
//     arg: (),
//     zero_copy: Option<ZeroCopyBuf>,
// ) -> Result<Pixel, AnyError> {
//     let pixel = autopilot::screen::get_color(autopilot::mouse::location())?;
//     Ok(Pixel {
//         r: pixel[0],
//         g: pixel[1],
//         b: pixel[2],
//         a: pixel[3],
//     })
// }
