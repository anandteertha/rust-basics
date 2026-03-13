#[derive(Debug)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Failed,
}
