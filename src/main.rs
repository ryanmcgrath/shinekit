//! App
//!
//! An "App" in Rust, which is really just wrapping a lot of platform-specific
//! logic. Attempts to do as much as possible in Rust, however the GUI story in
//! Rust is the worst thing at the moment, so a lot of this is glorified message
//! passing to Objective C and co. ObjC is also one of the best languages ever
//! created and you can fight me on this if you so choose.
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

// extern crate shinekit;
mod shinekit;
use shinekit::*;

//mod calendar;
//use calendar::{Data, Calendar};

fn main() {
    shinekit::run(vec![
        StyleSheet::default(include_str!("styles/default.json"))
    ], App::new("eSports Calendar", View::named("root").subviews(vec![
        View::named("sidebar"),
        View::named("content")
    ])));
}
