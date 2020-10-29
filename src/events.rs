use crate::db;

use prettytable::Table;

pub fn print_events() {
    let events = db::load_events();
    let mut ptable = Table::new();
    ptable.set_titles(row!["Id", "Date", "Description", "From", "To", "Account", "Sum"]);
    ptable.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    for event in events.iter() {
        let eid = &event.event_id;
        let date = &event.event_date;
        let desc = &event.event_desc;
        let from = &event.event_from;
        let to = &event.event_to;
        let acc = &event.event_acc;
        let sum_str = ((event.event_sum as f32) / 100.0).to_string() + " €";

        ptable.add_row(row![eid, date, desc, from, to, acc, sum_str]);
    }

    ptable.printstd();
}
