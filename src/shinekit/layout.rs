//! layout.rs
//!
//! A trait that more or less allows any other types to compose
//! a "subclass" for a view. By using the actual backing [UI|NS]View
//! for storing certain properties, it becomes much easier to get around
//! Rust's (good-for-you) restrictions.
//!
//! @author Ryan McGrath <ryan@rymc.io>
//! @created 05/30/2018

use objc::declare::ClassDecl;
use objc::runtime::{Object, BOOL};
use cocoa::base::{class, id, nil, YES, NO};
use cocoa::foundation::NSArray;

use shinekit::view::View;

pub trait Layout {
    fn get_subviews(&self) -> &Vec<View>;
    fn get_root_backing_node(&self) -> &Object;
    fn set_constraint_ivar(&mut self, ivar: &str, constraint: id);

}

pub fn add_autolayout_ivars(decl: &mut ClassDecl) {
}
