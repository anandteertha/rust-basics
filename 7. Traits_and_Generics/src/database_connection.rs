use crate::{
    connection::Connection,
    connection_status::{ConnectionStatus, parse_connection_status},
};

#[derive(Debug)]
pub struct DatabaseConnection {
    name: String,
    status: ConnectionStatus,
    metadata: DatabaseMetaData,
}

#[derive(Debug)]
struct DatabaseMetaData {
    users_count: i32,
}

impl DatabaseConnection {
    pub fn new(log: &str) -> Self {
        let words = log.split_whitespace();
        let mut index = 0;
        let mut db_connection = DatabaseConnection {
            name: String::new(),
            status: ConnectionStatus::Disconnected,
            metadata: DatabaseMetaData { users_count: 0 },
        };
        for word in words {
            if index == 1 {
                db_connection.name = word.to_owned();
            } else if index == 2 {
                db_connection.status = parse_connection_status(word)
            }
            index += 1;
        }
        db_connection
    }
}

impl Connection for DatabaseConnection {
    fn get_status(&self) -> &ConnectionStatus {
        &self.status
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_summary(&self) -> String {
        format!(
            "Name: {}\nStatus: {:?}\nUsers Connected: {}",
            self.get_name(),
            self.get_status(),
            self.metadata.users_count
        )
    }
}
