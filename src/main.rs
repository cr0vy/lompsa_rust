#[macro_use]
extern crate prettytable;

#[macro_use]
extern crate diesel;

mod db;
mod events;

fn main() {
    events::print_events();
}
