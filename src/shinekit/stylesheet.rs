//  stylesheet.rs
//
//  A basic struct for implementing namespaced stylesheets. Useful
//  so themes are built-in from the beginning. Currently mostly a
//  fun little JSON based thing, but I could see this being made
//  better in the future.
//
//  @author Ryan McGrath <ryan@rymc.io>
//  @created 05/30/2018

use serde_json::{Map, Value, from_str};
use util::merge_json_values;

pub struct StyleSheet {}

impl StyleSheet {
    pub fn default(styles: &str) -> (String, Value) {
        ("default".into(), from_str(styles).expect("Could not parse default JSON stylesheet"))
    }

    pub fn theme(name: &str, styles: &str) -> (String, Value) {
        (name.into(), from_str(styles).expect(&format!("Could not parse {} stylesheet", name)))
    }
}

pub fn load_styles(user_styles: Vec<(String, Value)>) -> Map<String, Value> {
    let mut styles = Map::new();
    for (name, value) in user_styles.into_iter() {
        if styles.contains_key("default") {
            let mut style = json!({});
            merge_json_values(&mut style, &styles["default"]);
            merge_json_values(&mut style, &value);
            styles.insert(name, style);
        } else {
            styles.insert(name, value);
        }
    }

    styles
}
