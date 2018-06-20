//! table.rs
//!
//! Wraps TableView class(es) on supported platforms. On macOS (OS X!) it wraps NSTableView,
//! and where possible on other platforms it wraps UITableView (Windows, for instance). In
//! either case this wraps the underlying APIs to mimic each other and remove nuisances that
//! exist.
//!
//! Many might ask "Why not \*CollectionView?", and to be fair, it's not a bad question. The
//! TableView APIs tend to be much better and smoother with regards to auto-calculating view
//! heights for dynamic entries, so... I just go with those when possible. Less headache.
//!
//! @author Ryan McGrath <ryan@rymc.io>
//! @created 05/23/2018

use objc_id::Id;
use cocoa::base::{class, id, nil, NO, YES};
use cocoa::foundation::NSString;

pub mod row;
pub mod datasource;
use shinekit::view::View;
use shinekit::scrollview::ScrollView;
use shinekit::util::empty_frame;
use shinekit::tableview::datasource::register_delegate_class;
pub use shinekit::tableview::row::{TableViewRow, TableViewUI};
pub use shinekit::tableview::datasource::TableViewData;

pub struct ListView {
    pub view: Id<Object>,
    pub scrollview: ScrollView,
    delegate: Id<Object>
}

impl ListView {
    pub fn new<T: TableViewData>(datasource: T) -> Self {
        let tableview: id;
        let scrollview = ScrollView::new();
        let delegate: id;

        unsafe {
            let cls: id = msg_send![register_list_class(), alloc];
            tableview = msg_send![cls, initWithFrame:empty_frame()];
            msg_send![tableview, setWantsLayer:YES];
            msg_send![tableview, setTranslatesAutoresizingMaskIntoConstraints:NO];
            msg_send![tableview, setUsesAutomaticRowHeights:YES];
            msg_send![tableview, setRowHeight:100.];
            msg_send![tableview, setFloatsGroupRows:YES];
            msg_send![tableview, setIntercellSpacing:NSSize::new(0., 0.)];
            msg_send![tableview, setColumnAutoresizingStyle:1];
            msg_send![tableview, setUsesAlternatingRowBackgroundColors:NO];
            //msg_send![tableview, setSelectionHighlightStyle:-1];
            msg_send![tableview, setAllowsEmptySelection:YES];
            msg_send![tableview, setAllowsMultipleSelection:NO];
            msg_send![tableview, setHeaderView:nil];

            // NSTableView requires at least one column to be manually added if doing so by code.
            // A relic of a bygone era, indeed.
            let default_column_alloc: id = msg_send![class("NSTableColumn"), new];
            let default_column: id = msg_send![default_column_alloc, initWithIdentifier:NSString::alloc(nil).init_str("Wut")];
            msg_send![default_column, setResizingMask:(1<<0)];
            msg_send![tableview, addTableColumn:default_column];
                       
            delegate = msg_send![register_delegate_class::<T>(), new];
    //msg_send![scrollview, setDocumentView:tableview];
        }

        let mut state = Box::new(datasource);
        let state_ptr: *mut T = &mut *state;
        unsafe {
            (&mut *delegate).set_ivar("shinekit_tableview_datasource", state_ptr as usize); 
            msg_send![tableview, setDelegate:delegate];
            msg_send![tableview, setDataSource:delegate];
        }
        
        TableView {
            view: IdRef::new(tableview),
            scrollview: IdRef::new(scrollview),
            delegate: IdRef::new(delegate)
        }
    }
}

fn register_list_class() -> *const Class {
    static mut list_class: *const Class = 0 as *const Class;
    static INIT: Once = ONCE_INIT;

    INIT.call_once(|| unsafe {
        let superclass = Class::get("NSTableView").unwrap();
        let mut decl = ClassDecl::new("shinekitListView", superclass).unwrap();
        add_autolayout_ivars(&mut decl);
        list_class = decl.register();
    });

    unsafe {
        list_class
    }
}
