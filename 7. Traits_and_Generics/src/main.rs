mod api_connection;
mod cache_connection;
mod connection;
mod connection_status;
mod database_connection;
mod sample_input;
use sample_input::sample_input;

use crate::{
    api_connection::ApiConnection, cache_connection::CacheConnection, connection::Connection,
    database_connection::DatabaseConnection,
};

fn main() {
    for log in sample_input().lines() {
        if log.starts_with("database") {
            print_summary(DatabaseConnection::new(log));
        } else if log.starts_with("api") {
            print_summary(ApiConnection::new(log));
        } else {
            print_summary(CacheConnection::new(log));
        }
    }
}

fn print_summary<T: Connection>(conn: T) {
    let summary = conn.get_summary();
    println!("{}", summary);
}
