pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;

use models::Events;

use schema::events::dsl::*;

fn establish_connection() -> SqliteConnection {
    let database_url = format!("{}/.local/share/lompsa/lompsa.db", env::var("HOME").unwrap());
    
    return SqliteConnection::establish(&database_url)
        .expect(&format!("Error connection to {}", database_url));
}

pub fn load_events() -> Vec<Events> {
    let conn = establish_connection();

    let results = events
                    .load::<Events>(&conn)
                    .expect("Error loading events");

    return results;
}