#[derive(Debug)]
pub enum State {
    Disconnected,
    Connecting,
    Connected,
    Failed,
}
