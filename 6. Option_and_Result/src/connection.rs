use crate::connection_state::ConnectionState;

#[derive(Debug)]
pub struct Connection {
    id: String,
    address: String,
    active: bool,
    status: ConnectionState,
}

impl Connection {
    pub fn new(id: String, address: &str) -> Self {
        let address = address.to_owned();
        let conn = Self {
            id,
            address,
            active: false,
            status: ConnectionState::Disconnected,
        };
        println!(
            "Initial: connection id: {} at address: {}, active: {}, status: {:?}",
            conn.id, conn.address, conn.active, conn.status
        );
        conn
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn activate_connection(&mut self) -> Result<String, String> {
        self.active = true;
        Ok(String::from(
            "Connection is active. You can start to connect now",
        ))
    }

    pub fn start_connecting(&mut self) -> Result<String, String> {
        if !self.active {
            return Err(String::from(
                "Connection is inactive. Failed to start connecting",
            ));
        }
        match self.status {
            ConnectionState::Disconnected | ConnectionState::Failed => {
                self.status = ConnectionState::Connecting;
                let message = format!(
                    "Connection with id: {} has status {:?}",
                    self.id, self.status
                );
                Ok(message)
            }
            ConnectionState::Connecting => {
                Err(String::from("Connection is already trying to connect!"))
            }
            ConnectionState::Connected => Err(String::from("Connection is already connected!")),
        }
    }

    pub fn mark_connected(&mut self) -> Result<String, String> {
        if !self.active {
            return Err(String::from("Connection is inactive. Failed to connect"));
        }
        match self.status {
            ConnectionState::Disconnected | ConnectionState::Failed => {
                Err(String::from("Please start connection to get connected"))
            }
            ConnectionState::Connecting => {
                self.status = ConnectionState::Connected;
                let message = format!(
                    "Connection with id: {} has status {:?}",
                    self.id, self.status
                );
                Ok(message)
            }
            ConnectionState::Connected => Err(String::from("Connection is already connected!")),
        }
    }

    pub fn fail(&mut self) -> Result<String, String> {
        self.status = ConnectionState::Failed;
        let message = format!(
            "Connection with id: {} has status {:?}",
            self.id, self.status
        );
        Ok(message)
    }

    pub fn disconnect(&mut self) -> Result<String, String> {
        self.status = ConnectionState::Disconnected;
        self.active = false;
        let message = format!(
            "Connection with id: {} has status {:?}",
            self.id, self.status
        );
        Ok(message)
    }

    pub fn print_summary(&self) -> Result<String, String> {
        let summary = format!("Connection Summary: {:?}\n\n", self);
        Ok(summary)
    }
}
