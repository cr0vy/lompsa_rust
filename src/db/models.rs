#[derive(Queryable)]
pub struct Events {
    pub event_id: i32,
    pub event_date: String,
    pub event_desc: String,
    pub event_from: String,
    pub event_to: String,
    pub event_acc: String,
    pub event_sum: i32
}
