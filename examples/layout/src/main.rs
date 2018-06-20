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

extern crate shinekit;
use shinekit::*;

fn main() {
    shinekit::run(vec![
        StyleSheet::default(include_str!("styles/default.json"))
    ], App::new("eSports Calendar", View::named("root").subviews(vec![
        View::named("sidebar"),
        View::named("content")
    ])));
}
