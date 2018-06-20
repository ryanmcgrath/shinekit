//! text.rs
//!
//! A class that wraps NSTextField and/or UILabel to make them act pretty
//! much the same across platforms. Believe it or not... this is a thing.
//!
//! @author Ryan McGrath <ryan@rymc.io>
//! @created 05/30/2018

use std::sync::{Once, ONCE_INIT};

use objc_id::Id;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object};

use cocoa::base::{id, nil, YES, NO};
use cocoa::foundation::NSString;

use shinekit::color::Color;
use shinekit::util::empty_frame;

pub struct Label;

impl Label {
    pub fn named(name: &str) -> Self {
        View::named_of_kind_with_backing_node(name, ViewKind::Label, unsafe {
            let alloc: id = msg_send![register_text_class(), alloc];
            let view: id = msg_send![alloc, initWithFrame:empty_frame()];
            msg_send![view, setTranslatesAutoresizingMaskIntoConstraints:NO];
            msg_send![view, setEditable:NO];
            msg_send![view, setBezeled:NO];
            msg_send![view, setBordered:NO];
            msg_send![view, setDrawsBackground:YES];
            msg_send![view, setAllowsEditingTextAttributes:NO];
            msg_send![view, setContentCompressionResistancePriority:250 forOrientation:0];
            
            let cell: id = msg_send![view, cell];
            msg_send![cell, setUsesSingleLineMode:NO];
            msg_send![cell, setWraps:YES];
            msg_send![cell, setLineBreakMode:0];
            Id::from_ptr(view)
        })
    }
}

#[allow(dead_code)]
impl Text {
    pub fn new() -> Self {
        Text {
            backing_node: unsafe {
            }
        }
    }
    
    pub fn set_background_color(&mut self, color: &Color) {
        unsafe {
            self.backing_node.set_ivar("shinekitBackgroundColor", color.into_platform_specific_color());
            msg_send![&*self.backing_node, setNeedsDisplay:YES];
        }
    } 

    pub fn set_text(&self, text: &str) {
        unsafe {
            let value = NSString::alloc(nil).init_str(text);
            msg_send![&*self.backing_node, setStringValue:value];
        }
    }

    pub fn set_text_color(&self, color: &Color) {
        unsafe {
            msg_send![&*self.backing_node, setTextColor:color.into_platform_specific_color()];
        }
    }
}

fn register_text_class() -> *const Class {
    static mut text_class: *const Class = 0 as *const Class;
    static INIT: Once = ONCE_INIT;

    INIT.call_once(|| unsafe {
        let superclass = Class::get("NSTextField").unwrap();
        let mut decl = ClassDecl::new("ShineKitLabel", superclass).unwrap();
        //decl.add_method(sel!(wantsUpdateLayer), enforce_normalcy as extern fn(&Object, _) -> BOOL);
        //decl.add_method(sel!(updateLayer), update_layer as extern fn(&Object, _));
        add_autolayout_ivars(&mut decl);
        text_class = decl.register();
    });

    unsafe {
        text_class
    }
}
