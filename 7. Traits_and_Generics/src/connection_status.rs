#[derive(Debug)]
pub enum ConnectionStatus {
    Connected,
    Degraded,
    Disconnected,
}

pub fn parse_connection_status(log_status: &str) -> ConnectionStatus {
    match log_status {
        "connected" => ConnectionStatus::Connected,
        "degraded" => ConnectionStatus::Degraded,
        "disconnected" | &_ => ConnectionStatus::Disconnected,
    }
}
