#[derive(Debug)]
pub struct Connection {
    id: String,
    address: String,
    active_status: bool,
    retry_count: u8,
}

impl Connection {
    pub fn new(id: String, address: String) -> Self {
        println!("Created connection with id: {} to address: {}", id, address);
        println!("Initial: inactive, retries: 0");
        Self {
            id,
            address,
            active_status: false,
            retry_count: 0,
        }
    }

    pub fn activate(&mut self) {
        self.active_status = true;
        println!("Connection {} activated at {}", self.id, self.address);
    }

    pub fn deactivate(&mut self) {
        self.active_status = false;
        println!("Connection {} deactivated at {}", self.id, self.address);
    }

    pub fn increment_retries(&mut self) {
        if !self.active_status {
            println!("Cannot increment retry count as connection is inactive!");
            return;
        }
        self.retry_count += 1;
        println!("Retry count increased to {}", self.retry_count);
    }

    pub fn print_summary(&self) {
        println!("FINAL: {:?}", self);
    }
}
