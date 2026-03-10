use crate::connection_state::State;

#[derive(Debug)]
pub struct Connection {
    id: String,
    address: String,
    active: bool,
    status: State,
}

impl Connection {
    pub fn new(id: String, address: &str) -> Self {
        println!(
            "Created a new Connection with id: {} and address: {}",
            id, address
        );
        let address: String = address.to_owned();
        let conn = Self {
            id,
            address,
            active: false,
            status: State::Disconnected,
        };
        println!(
            "Initial: connection id: {} at address: {}, active: {}, status: {:?}",
            conn.id, conn.address, conn.active, conn.status
        );
        conn
    }

    pub fn activate_connection(&mut self) {
        self.active = true;
        println!("Connection is active. You can start to connect now");
    }

    pub fn start_connecting(&mut self) {
        if !self.active {
            println!("Connection is inactive. Failed to start connecting");
            return;
        }
        match self.status {
            State::Disconnected | State::Failed => {
                self.status = State::Connecting;
                println!(
                    "Connection with id: {} has status {:?}",
                    self.id, self.status
                );
            }
            State::Connecting => {
                println!("Connection is already trying to connect!");
            }
            State::Connected => {
                println!("Connection is already connected!");
            }
        }
    }

    pub fn mark_connected(&mut self) {
        if !self.active {
            println!("Connection is inactive. Failed to connect");
            return;
        }
        match self.status {
            State::Disconnected | State::Failed => {
                println!("Please start connection to get connected");
            }
            State::Connecting => {
                self.status = State::Connected;
                println!(
                    "Connection with id: {} has status {:?}",
                    self.id, self.status
                );
            }
            State::Connected => {
                println!("Connection is already connected!");
            }
        }
    }

    pub fn fail(&mut self) {
        self.status = State::Failed;
        println!(
            "Connection with id: {} has status {:?}",
            self.id, self.status
        );
    }

    pub fn reset(&mut self) {
        self.status = State::Disconnected;
        self.active = false;
        println!(
            "Connection with id: {} has status {:?}",
            self.id, self.status
        );
    }

    pub fn print_summary(&self) {
        println!("Connection Summary: {:?}\n\n", self);
    }
}
