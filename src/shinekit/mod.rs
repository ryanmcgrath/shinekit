//! mod.rs
//!
//! Shinekit main module, which is basically handling various things
//! like ensuring styles are registered, the App starts running, and
//! everything is properly resolved and installed.
//!
//! @author Ryan McGrath <ryan@rymc.io>
//! @created 05/30/2018

pub mod application;
//pub mod tableview;
pub mod color;
//pub mod scrollview;
//pub mod text;
pub mod util;
pub mod view;
pub mod stylesheet;

pub use application::App;
pub use stylesheet::StyleSheet;
pub use view::View;

use serde_json::Value;
use stylesheet::load_styles;

pub fn run(user_styles: Vec<(String, Value)>, mut application: App) {
    let mut styles = load_styles(user_styles); 
    let mut current_style = styles["default"].as_object_mut().unwrap();
    application.apply_styles(&mut current_style); 
    application.run();
}
