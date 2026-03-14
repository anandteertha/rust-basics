use crate::{
    connection::Connection,
    connection_status::{ConnectionStatus, parse_connection_status},
};

#[derive(Debug)]
pub struct ApiConnection {
    name: String,
    status: ConnectionStatus,
    metadata: ApiMetaData,
}

#[derive(Debug)]
struct ApiMetaData {
    retry: i32,
}

impl ApiConnection {
    pub fn new(log: &str) -> Self {
        let words = log.split_whitespace();
        let mut index = 0;
        let mut api_connection = ApiConnection {
            name: String::new(),
            status: ConnectionStatus::Disconnected,
            metadata: ApiMetaData { retry: 3 },
        };
        for word in words {
            if index == 1 {
                api_connection.name = word.to_owned();
            } else if index == 2 {
                api_connection.status = parse_connection_status(word)
            }
            index += 1;
        }
        api_connection
    }
}

impl Connection for ApiConnection {
    fn get_status(&self) -> &ConnectionStatus {
        &self.status
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_summary(&self) -> String {
        format!(
            "Name: {}\nStatus: {:?}\n Retries: {}",
            &self.get_name(),
            &self.get_status(),
            &self.metadata.retry
        )
    }
}
