use crate::db;

pub fn print_events() {
    let events = db::load_events();

    for event in events.iter() {
        println!("{0} | {1}", event.event_id, event.event_date);
    }
}
