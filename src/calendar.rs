//! calendar.rs
//!
//! The actual implementation of the calendar view/data/etc. Fetches
//! upcoming tournaments, ensures everything's good, and passes it on
//! demand to the rendering view(s).
//!
//! @author Ryan McGrath <ryan@rymc.io>
//! @created 05/30/2018

use ruikit::view::View;
use ruikit::color::Color;
use ruikit::tableview::{TableViewData, TableViewRow};

#[derive(Debug)]
pub struct Data {
    pub color: Color,
    pub label: String
}

impl Data {
    pub fn new(msg: &str, r: i32, g: i32, b: i32) -> Self {
        Data {
            label: msg.to_string(),
            color: Color::rgb(r,g,b)
        }
    }
}

pub struct Calendar {
    pub tournaments: Vec<Data>,
    pub r: Color
}

impl TableViewData for Calendar {
    fn number_of_items(&self) -> usize { 
        self.tournaments.len()
    }
    
    fn configure_item(&mut self, view: &TableViewRow, row: usize) {
        println!("Hmmm... {}", row);
        if row == 3 {
            //view.set_background_color(&Color::system_red());
            view.set_background_color(&self.r);
        }/* else if row == 2 {
            view.set_background_color(&Color::rgb(5,35,229));
        } else {
            view.set_background_color(&Color::system_blue());
        }*/
    }
}
