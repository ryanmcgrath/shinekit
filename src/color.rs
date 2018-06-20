///
/// Color.rs
///
/// Interface that wraps [NS/UI]Color depending on the platform. Decidedly
/// basic as I don't care to get into the whole colorspace issue, and would
/// rather just be able to put color on a screen.
///
/// @author Ryan McGrath <ryan@rymc.io>
/// @created 05/23/2018
///

use cocoa::base::{id, class};
use serde_json::Value;

#[derive(Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64
}

#[allow(dead_code)]
impl Color {
    pub fn rgb(r: i32, g: i32, b: i32) -> Self {
        Color {
            r: r as f64 / 255.0,
            g: g as f64 / 255.0,
            b: b as f64 / 255.0,
            a: 1.
        }
    }

    pub fn from_json(obj: &Value) -> Self {
        Color {
            r: obj["r"].as_f64().unwrap() / 255.0,
            g: obj["g"].as_f64().unwrap() / 255.0,
            b: obj["b"].as_f64().unwrap() / 255.0,
            a: 1.
        }
    }

    pub fn into_platform_specific_color(&self) -> id {
        unsafe { msg_send![class("NSColor"), colorWithRed:self.r green:self.g blue:self.b alpha:self.a] }
    }
}
