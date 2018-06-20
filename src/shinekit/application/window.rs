//! window.rs
//!
//! Handles creating/maintaining/applying styles for the root backing window of
//! an app. Very Desktop dependent, as mobile apps... well, they're just in a
//! window by default.
//!
//! @author Ryan McGrath <ryan@rymc.io>
//! @created 05/30/2018

use cocoa::base::{id, nil, YES, NO};
use cocoa::appkit::{NSWindow, NSWindowStyleMask, NSBackingStoreType};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSString, NSAutoreleasePool};

use shinekit::color::Color;
use shinekit::view::View;

pub struct Window {
    pub window: id,
    pub content_view: View
}

impl Window {
    pub fn new(view: View, title: &str, top: i32, left: i32, width: i32, height: i32) -> Self {
        Window {
            window: unsafe {
                let style = NSWindowStyleMask::NSResizableWindowMask |
                    NSWindowStyleMask::NSUnifiedTitleAndToolbarWindowMask | NSWindowStyleMask::NSMiniaturizableWindowMask |
                    NSWindowStyleMask::NSClosableWindowMask | NSWindowStyleMask::NSTitledWindowMask;
                    //NSWindowStyleMask::NSWindowStyleMaskResizable | NSWindowStyleMask::NSWindowStyleMaskMiniaturizable | NSWindowStyleMask::NSWindowStyleMaskClosable | NSWindowStyleMask::NSWindowStyleMaskTitled | NSWindowStyleMask::NSWindowStyleMaskUnifiedTitleAndToolbar

                let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
                    NSRect::new(NSPoint::new(top.into(), left.into()), NSSize::new(width.into(), height.into())),
                    style,
                    NSBackingStoreType::NSBackingStoreBuffered,
                    NO
	            ).autorelease();
            
                let title = NSString::alloc(nil).init_str(title);
                window.setTitle_(title);
                view.translates_resizing_mask_into_constraints(true);
                msg_send![window, setContentView:view.get_root_backing_node()];
                msg_send![window, setTitlebarAppearsTransparent:YES];
                msg_send![window, setTitleVisibility:1];
                window
            },
            content_view: view,
        }
    }
    
    pub fn make_key(&self) {
        unsafe {
            self.window.makeKeyAndOrderFront_(nil);
        }
    }

    /*pub fn set_content_view(&mut self, view: &T) {
        unsafe {
            view.translates_resizing_mask_into_constraints(true);
            msg_send![self.window, setContentView:&*view.get_root_backing_node()];
        }
    }*/

    pub fn set_frame(&self, rect: NSRect) {
        unsafe {
            msg_send![self.window, setFrame:rect display:YES];
        }
    }

    pub fn set_background_color(&self, color: Color) {
        unsafe {
            msg_send![self.window, setBackgroundColor:color.into_platform_specific_color()];
        }
    }
}
