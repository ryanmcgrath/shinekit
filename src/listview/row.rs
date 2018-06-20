//! row.rs
//!
//! A default implementation for a TableView row, which... well, depending
//! on the platform and/or environment, requires a few different things. This
//! ensures that we at least have the following:
//!
//!     - Properly flipped coordinates, because it's {CURRENT_YEAR} and who
//!         the hell judges from the bottom left. Confuses all newcomers.
//!
//! @author Ryan McGrath <ryan@rymc.io>
//! @created 05/30/2018

use cocoa::base::{id};

pub trait TableViewUI {
    fn layout(&self, view: &TableViewRow);
    fn update(&self, view: &TableViewRow);
}

pub struct TableViewRow {
    pub row: usize,
    pub view: id
}
