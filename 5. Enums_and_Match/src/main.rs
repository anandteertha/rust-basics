pub mod connection;
pub mod connection_state;

fn main() {
    let address: String = "127.0.0.1:8080".to_owned();
    let mut conn = connection::Connection::new("#01".to_owned(), &address);
    conn.activate_connection();
    conn.start_connecting();
    conn.fail();
    conn.print_summary();

    let mut conn = connection::Connection::new("#02".to_owned(), &address);
    conn.activate_connection();
    conn.start_connecting();
    conn.mark_connected();
    conn.print_summary();

    let mut conn = connection::Connection::new("#03".to_owned(), &address);
    conn.start_connecting();
    conn.print_summary();

    let mut conn = connection::Connection::new("#04".to_owned(), &address);
    conn.activate_connection();
    conn.start_connecting();
    conn.mark_connected();
    conn.reset();
    conn.print_summary();

    let mut conn = connection::Connection::new("#05".to_owned(), &address);
    conn.activate_connection();
    conn.start_connecting();
    conn.mark_connected();
    conn.fail();
    conn.print_summary();
}
