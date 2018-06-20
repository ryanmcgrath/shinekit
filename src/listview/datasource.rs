//! tableview_delegate_internal.rs
//!
//! This is a struct that handles the nitty gritty implementation of [NS/UI]TableViews.
//! These are classes that have a lot of delegate-based implementation, so this is
//! essentially bridging that gap for Rust.
//!
//! @author Ryan McGrath <ryan@rymc.io>
//! @created 05/30/2018

use std::sync::{Once, ONCE_INIT};

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use cocoa::base::{id, nil, NO, YES};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSString, NSInteger};

use shinekit::tableview::row::TableViewRow;
use shinekit::view::register_view_class;

pub trait TableViewData {
    fn number_of_items(&self) -> usize { 0 }
    fn configure_item(&mut self, view: &TableViewRow, row: usize);
}

extern fn number_of_items<T: TableViewData>(this: &Object, _: Sel, _: id) -> NSInteger {
    println!("number_of_items");
    let count: usize;

    unsafe {
        let state: usize = *this.get_ivar("shinekitDataSourceAndDelegate");
        let state = state as *mut T;
        count = (*state).number_of_items();
    }
    
    return count as NSInteger;
} 

extern fn make_view<T: TableViewData>(this: &Object, _: Sel, table_view: id, _: id, row: NSInteger) -> id {
    println!("make_view");
    let mut cell: id;
    let view: TableViewRow;
    let mut requires_layout = false;

    unsafe {       
        let state: usize = *this.get_ivar("shinekitgSourceAndDelegate");
        let state = state as *mut T;

        let title = NSString::alloc(nil).init_str("WHAT");
        cell = msg_send![table_view, makeViewWithIdentifier:title owner:nil];
        if cell == nil {
            println!("    Creating new cell...");
            requires_layout = true;
            let default_frame = NSRect::new(NSPoint::new(0., 0.), NSSize::new(0., 0.));
            let cls: id = msg_send![register_view_class(), alloc];
            cell = msg_send![cls, initWithFrame:default_frame];
            msg_send![cell, setIdentifier:title];
            msg_send![cell, setWantsLayer:YES];
            msg_send![cell, setTranslatesAutoresizingMaskIntoConstraints:NO];
        }
        
        view = TableViewRow {
            row: row as usize,
            view: cell
        };
        println!("Configuring cell...");
        (*state).configure_item(&view, row as usize);
        println!("Configured Cell");
    }

    return cell;
}

pub fn register_listview_delegate_class<T: TableViewData>() -> *const Class {
    static mut delegate_class: *const Class = 0 as *const Class;
    static INIT: Once = ONCE_INIT;

    INIT.call_once(|| unsafe {
        let superclass = Class::get("NSObject").unwrap();
        let mut decl = ClassDecl::new("shinekitDataSourceAndDelegate", superclass).unwrap();

        // Add callback methods
        decl.add_method(sel!(numberOfRowsInTableView:), number_of_items::<T> as extern fn(&Object, _, id) -> NSInteger);
        decl.add_method(sel!(tableView:viewForTableColumn:row:), make_view::<T> as extern fn(&Object, _, id, id, NSInteger) -> id);

        // Store internal state as user data
        decl.add_ivar::<usize>("shinekitDataSourceAndDelegate");
        delegate_class = decl.register();
    });

    unsafe {
        delegate_class
    }
}
