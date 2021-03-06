//! view.rs
//!
//! "Fixes" NSView to be a bit more... how does one say, modern. Flips drawing
//! and layout coordinates to be fitting for {{CURRENT YEAR}}, layer-backs it all
//! by default, and does some ivar trickery to make NSColor less of a headache.
//!
//! @author Ryan McGrath <ryan@rymc.io>
//! @created 05/30/2018

use std::sync::{Once, ONCE_INIT};
use serde_json::{Map, Value};

use objc_id::Id;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel, BOOL};
use cocoa::foundation::NSArray;
use cocoa::base::{class, id, nil, YES, NO};

use color::Color;
use util::empty_frame;

#[derive(Debug)]
pub enum ViewKind {
    View,
    Label
}

#[derive(Debug)]
pub struct View {
    pub kind: ViewKind,
    pub name: String,
    pub backing_node: Id<Object>,
    pub subviews: Vec<View>
}

impl View {
    pub fn named_of_kind_with_backing_node(name: &str, kind: ViewKind, backing_node: Id<Object>) -> Self {
        View { 
            name: name.into(),
            kind: kind,
            backing_node: backing_node,
            subviews: vec![]
        }
    }

    pub fn named(name: &str) -> Self {
        View::named_of_kind_with_backing_node(name, ViewKind::View, unsafe {
            let alloc: id = msg_send![register_view_class(), alloc];
            let view: id = msg_send![alloc, initWithFrame:empty_frame()];
            msg_send![view, setWantsLayer:YES];
            msg_send![view, setLayerContentsRedrawPolicy:1];
            msg_send![view, setTranslatesAutoresizingMaskIntoConstraints:NO];
            Id::from_ptr(view)
        })
    }

    pub fn subviews(self, views: Vec<View>) -> Self {
        let mut subviews = vec![];
        for view in views.into_iter() {
            unsafe { msg_send![&*self.backing_node, addSubview:&*view.backing_node]; }
            subviews.push(view);
        }
        
        View { subviews: subviews, ..self }
    }

    pub fn set_background_color(&mut self, color: &Color) {
        unsafe {
            self.backing_node.set_ivar("shinekitBackgroundColor", color.into_platform_specific_color());
            msg_send![&*self.backing_node, setNeedsDisplay:YES];
        }
    }
    
    pub fn get_root_backing_node(&self) -> &Object { &*self.backing_node }
    pub fn get_subviews(&self) -> &Vec<View> { &self.subviews }
    pub fn set_ivar(&mut self, ivar: &str, constraint: id) {
        unsafe {
            self.backing_node.set_ivar(ivar, constraint);
        }
    }
    
    pub fn add_subview(&self, view: &View) {
        unsafe {
            msg_send![self.get_root_backing_node(), addSubview:view.get_root_backing_node()];
        }
    }
    
    pub fn translates_resizing_mask_into_constraints(&self, translates: bool) {
        let t: BOOL = if translates { YES } else { NO };
        unsafe { 
            msg_send![self.get_root_backing_node(), setTranslatesAutoresizingMaskIntoConstraints:t];
        }
    }
  
    pub fn set_width(&mut self, width: f64) {
        unsafe {
            let anchor: id = msg_send![self.get_root_backing_node(), widthAnchor];
            let constraint: id = msg_send![anchor, constraintEqualToConstant:width];
            self.set_ivar("shinekitConstraintWidth", constraint);
        }
    }

    pub fn set_height(&mut self, height: f64) {
        unsafe {
            let anchor: id = msg_send![self.get_root_backing_node(), heightAnchor];
            let constraint: id = msg_send![anchor, constraintEqualToConstant:height];
            self.set_ivar("shinekitConstraintHeight", constraint);
        }
    }
    
    pub fn top_relative_to(&mut self, view: &View, margin: i32) {
        unsafe {
            let top_anchor: id = msg_send![self.get_root_backing_node(), topAnchor];
            let view_top_anchor: id = msg_send![view.get_root_backing_node(), topAnchor];
            let constraint: id = msg_send![top_anchor, constraintEqualToAnchor:view_top_anchor constant:margin as f64];
            self.set_ivar("shinekitConstraintTop", constraint);
        }
    }
   
    pub fn leading_relative_to(&mut self, view: &View, margin: i32) {
        unsafe {
            let leading_anchor: id = msg_send![self.get_root_backing_node(), leadingAnchor];
            let view_leading_anchor: id = msg_send![view.get_root_backing_node(), leadingAnchor];
            let constraint: id = msg_send![leading_anchor, constraintEqualToAnchor:view_leading_anchor constant:margin as f64];
            self.set_ivar("shinekitConstraintLeading", constraint);
        }
    }

    pub fn trailing_relative_to(&mut self, view: &View, margin: i32) {
        let m = margin as f64 * -1.;
        unsafe {
            let trailing_anchor: id = msg_send![self.get_root_backing_node(), trailingAnchor];
            let view_trailing_anchor: id = msg_send![view.get_root_backing_node(), trailingAnchor];
            let constraint: id = msg_send![trailing_anchor, constraintEqualToAnchor:view_trailing_anchor constant:m];
            self.set_ivar("shinekitConstraintTrailing", constraint);
        }
    }

    pub fn bottom_relative_to(&mut self, view: &View, margin: i32) {
        let m = margin as f64 * -1.;
        unsafe {
            let bottom_anchor: id = msg_send![self.get_root_backing_node(), bottomAnchor];
            let view_bottom_anchor: id = msg_send![view.get_root_backing_node(), bottomAnchor];
            let constraint: id = msg_send![bottom_anchor, constraintEqualToAnchor:view_bottom_anchor constant:m];
            self.set_ivar("shinekitConstraintBottom", constraint);
        }
    }
    
    pub fn activate_constraints(&self) {
        let mut constraints: Vec<id> = vec![];
        
        let ivars = [
            "shinekitConstraintWidth", "shinekitConstraintHeight",
            "shinekitConstraintTop", "shinekitConstraintLeading",
            "shinekitConstraintTrailing", "shinekitConstraintBottom"
        ];
 
        for ivar in &ivars {
            let constraint: id;
            unsafe { constraint = *self.backing_node.get_ivar(ivar); }

            if constraint != nil {
                constraints.push(constraint);
            }
        }
        
        if constraints.len() > 0 {
            unsafe {
                let bundle = NSArray::arrayWithObjects(nil, &constraints);
                msg_send![class("NSLayoutConstraint"), activateConstraints:bundle];
            }
        }
    }

    pub fn apply_styles(&mut self, styles: &mut Map<String, Value>) {
        let bg_color = Color::from_json(&styles[&self.name]["backgroundColor"]);
        self.set_background_color(&bg_color);

        match styles[&self.name]["width"].as_f64() {
            Some(width) => self.set_width(width),
            None => ()
        }
        
        match styles[&self.name]["height"].as_f64() {
            Some(width) => self.set_width(width),
            None => ()
        }
        
        for view in &mut self.subviews {
            view.apply_styles(styles);
        }
    }
}

extern fn enforce_normalcy(_: &Object, _: Sel) -> BOOL {
    return YES;
}

extern fn update_layer(this: &Object, _: Sel) {
    unsafe {
        let background_color: id = *this.get_ivar("shinekitBackgroundColor");
        if background_color != nil {
            let layer: id = msg_send![this, layer];
            let cg: id = msg_send![background_color, CGColor];
            msg_send![layer, setBackgroundColor:cg];
        }
    }
}

fn register_view_class() -> *const Class {
    static mut view_class: *const Class = 0 as *const Class;
    static INIT: Once = ONCE_INIT;

    INIT.call_once(|| unsafe {
        let superclass = Class::get("NSView").unwrap();
        let mut decl = ClassDecl::new("shinekitView", superclass).unwrap();
        decl.add_method(sel!(isFlipped), enforce_normalcy as extern fn(&Object, _) -> BOOL);
        decl.add_method(sel!(requiresConstraintBasedLayout), enforce_normalcy as extern fn(&Object, _) -> BOOL);
        decl.add_method(sel!(wantsUpdateLayer), enforce_normalcy as extern fn(&Object, _) -> BOOL);
        decl.add_method(sel!(updateLayer), update_layer as extern fn(&Object, _));
        decl.add_ivar::<id>("shinekitBackgroundColor");
        decl.add_ivar::<id>("shinekitConstraintWidth");
        decl.add_ivar::<id>("shinekitConstraintHeight");
        decl.add_ivar::<id>("shinekitConstraintTop");
        decl.add_ivar::<id>("shinekitConstraintLeading");
        decl.add_ivar::<id>("shinekitConstraintTrailing");
        decl.add_ivar::<id>("shinekitConstraintBottom");
        view_class = decl.register();
    });

    unsafe {
        view_class
    }
}
