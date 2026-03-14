use crate::connection_status::ConnectionStatus;

pub trait Connection {
    fn get_status(&self) -> &ConnectionStatus;
    fn get_name(&self) -> &str;
    fn get_summary(&self) -> String;
}
