//! mod.rs
//!
//! Shinekit main module, which is basically handling various things
//! like ensuring styles are registered, the App starts running, and
//! everything is properly resolved and installed.
//!
//! @author Ryan McGrath <ryan@rymc.io>
//! @created 05/30/2018

#![allow(dead_code)]
#![allow(non_upper_case_globals)]

#[macro_use]
extern crate objc;
extern crate cocoa;
extern crate objc_id;
extern crate core_graphics;

#[macro_use]
extern crate serde_json;
use serde_json::Value;

pub mod application;
pub use application::App;

pub mod window;
pub use window::Window;

pub mod color;
pub mod util;
pub mod stylesheet;
use stylesheet::load_styles;
pub use stylesheet::StyleSheet;

pub mod view;
pub use view::View;

pub fn run(user_styles: Vec<(String, Value)>, mut application: App) {
    let mut styles = load_styles(user_styles); 
    let mut current_style = styles["default"].as_object_mut().unwrap();
    application.apply_styles(&mut current_style); 
    application.run();
}
