mod connection;

fn main() {
    let mut conn = connection::Connection::new("#01".to_owned(), "127.0.0.1:8080".to_owned());
    conn.increment_retries();
    conn.activate();
    conn.increment_retries();
    conn.increment_retries();
    conn.deactivate();
    conn.print_summary();
}
