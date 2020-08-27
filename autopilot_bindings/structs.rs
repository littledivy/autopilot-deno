// use serde
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize)]
pub struct WindowResponse<'a> {
    pub window: &'a str,
}

#[derive(Deserialize)]
pub struct NotifyParams {
    pub title: String,
    pub body: String,
}

#[derive(Serialize)]
pub struct Resp {
    pub height: f64,
    pub width: f64,
}

#[derive(Serialize)]
pub struct MonitorResponse<'a> {
    pub monitors: &'a str,
}

#[derive(Serialize)]
pub struct ScaleResponse {
    pub scale: f64,
}

#[derive(Deserialize)]
pub struct QuickMousePostition {
    pub x: f64,
    pub y: f64,
}

#[derive(Deserialize)]
pub struct MousePostition {
    pub x: f64,
    pub y: f64,
    pub d: f64,
}

#[derive(Deserialize)]
pub struct TransformParams {
    pub height: u16,
    pub width: u16,
    pub index: usize,
}

// struct for options used by Alert
#[derive(Deserialize)]
pub struct AlertOptions {
    pub msg: String,
    pub title: String,
}

#[derive(Serialize)]
pub struct PixelRsp {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Deserialize)]
pub struct PointPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize)]
pub struct MouseResp {
    pub x: f64,
    pub y: f64,
}

#[derive(Deserialize)]
pub struct ToggleOptions {
    pub key: String,
    pub down: i32,
}
