//! application.rs
//!
//! Wraps application lifetime pieces across platforms. A lot of
//! unsafe code in here since it has to interact with a lot of
//! outside actors. With that said, if this crashes... there's bigger
//! problems with the system, lol.
//!
//! @author Ryan McGrath <ryan@rymc.io>
//! @created 05/30/2018

use cocoa::base::{id, nil};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSAutoreleasePool};
use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivationPolicyRegular,
    NSRunningApplication, NSApplicationActivateIgnoringOtherApps
};

use serde_json::{Map, Value};

pub mod window;
use shinekit::color::Color;
use shinekit::application::window::Window;
use shinekit::view::View;

pub struct App {
    pub app: id,
    pub window: Window,
}
/*
pub trait Delegate {
    // AppDelegate lifecycle callbacks
    fn did_finish_launching(&self) {}
    fn will_become_active(&self) {}
    fn will_enter_foreground(&self) {}
    fn did_become_active(&self) {}
    fn will_resign_active(&self) {}
    fn did_enter_background(&self) {}
    fn will_terminate(&self) {}

    // Override-able settings
    fn initial_window_rect(&self) -> (f64, f64, f64, f64) { (0., 0., 800., 600.) }
    fn window_mask(&self) -> NSWindowStyleMask {
    }
}*/

impl App {
    pub fn new(title: &str, view: View) -> Self {
        App {
            app: unsafe {
                let _pool = NSAutoreleasePool::new(nil);
                let app = NSApp();
                app.setActivationPolicy_(NSApplicationActivationPolicyRegular);            
                app
            },

            window: Window::new(view, title, 0, 0, 0, 0)
        }
    }

    pub fn run(&self) {
        unsafe {
            let current_app = NSRunningApplication::currentApplication(nil);
            current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
            self.window.make_key();
            self.app.run();
        }
    }

    pub fn apply_styles(&mut self, styles: &mut Map<String, Value>) {
        let bg_color = Color::from_json(&styles["window"]["backgroundColor"]);
        self.window.set_background_color(bg_color);
        
        let width = &styles["window"]["defaultWidth"].as_f64().unwrap();
        let height = &styles["window"]["defaultHeight"].as_f64().unwrap();
        let rect = NSRect::new(NSPoint::new(0., 0.), NSSize::new(*width, *height));
        self.window.set_frame(rect);

        self.window.content_view.apply_styles(styles);
    }
}

//impl Delegate for App {}
