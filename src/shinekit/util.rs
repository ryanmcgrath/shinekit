//! util.rs
//!
//! Basic utility functions to handle some boilerplate things littered
//! throughout this app.
//!
//! @author Ryan McGrath <ryan@rymc.io>
//! @created 05/30/2018

use serde_json::{Value};
use cocoa::foundation::{NSRect, NSPoint, NSSize};

pub fn empty_frame() -> NSRect {
    NSRect::new(NSPoint::new(0., 0.), NSSize::new(0., 0.))
}

pub fn merge_json_values(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge_json_values(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }

        (a, b) => {
            *a = b.clone();
        }
    }
}
