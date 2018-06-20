//! scrollview.rs
//!
//! Handles making NSScrollView behave more like a UIKit counterpart. Wraps
//! up various methods to make it more... enjoyable to use.
//!
//! @author Ryan McGrath <ryan@rymc.io>
//! @created 05/30/2018

use std::sync::{Once, ONCE_INIT};

use objc_id::Id;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object};

use cocoa::base::{id, YES, NO};

use shinekit::util::empty_frame;
use shinekit::layout::{Layout, add_autolayout_ivars};

pub struct ScrollView {
    backing_node: Id<Object>
}

impl ScrollView {
    pub fn new() -> Self {
        ScrollView {
            backing_node: unsafe {
                let ins: id = msg_send![register_scrollview_class(), alloc];
                let scrollview: id = msg_send![ins, initWithFrame:empty_frame()];
                msg_send![scrollview, setTranslatesAutoresizingMaskIntoConstraints:NO];
                msg_send![scrollview, setDrawsBackground:NO];
                msg_send![scrollview, setWantsLayer:YES];
                msg_send![scrollview, setBorderType:0];
                msg_send![scrollview, setHorizontalScrollElasticity:1];
                msg_send![scrollview, setHasVerticalScroller:YES];
                Id::from_ptr(scrollview)
            }
        }
    }
}

impl Layout for ScrollView {
    fn get_root_backing_node(&self) -> &Object { &*self.backing_node }
    fn set_constraint_ivar(&mut self, ivar: &str, constraint: id) { unsafe { self.backing_node.set_ivar(ivar, constraint); } }
}

fn register_scrollview_class() -> *const Class {
    static mut scrollview_class: *const Class = 0 as *const Class;
    static INIT: Once = ONCE_INIT;

    INIT.call_once(|| unsafe {
        let superclass = Class::get("NSScrollView").unwrap();
        let mut decl = ClassDecl::new("shinekitScrollView", superclass).unwrap();
        add_autolayout_ivars(&mut decl);
        scrollview_class = decl.register();
    });

    unsafe {
        scrollview_class
    }
}
