use crate::{
    connection::Connection,
    connection_status::{ConnectionStatus, parse_connection_status},
};
#[derive(Debug)]
pub struct CacheConnection {
    name: String,
    status: ConnectionStatus,
    metadata: CacheMetaData,
}

#[derive(Debug)]
struct CacheMetaData {
    ttl: i32,
}

impl CacheConnection {
    pub fn new(log: &str) -> Self {
        let words = log.split_whitespace();
        let mut index = 0;
        let mut cache_connection = CacheConnection {
            name: String::new(),
            status: ConnectionStatus::Disconnected,
            metadata: CacheMetaData { ttl: 100 },
        };
        for word in words {
            if index == 1 {
                cache_connection.name = word.to_owned();
            } else if index == 2 {
                cache_connection.status = parse_connection_status(word)
            }
            index += 1;
        }
        cache_connection
    }
}

impl Connection for CacheConnection {
    fn get_status(&self) -> &ConnectionStatus {
        &self.status
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_summary(&self) -> String {
        format!(
            "Name: {}\nStatus: {:?}\nTTL: {}",
            self.get_name(),
            self.get_status(),
            self.metadata.ttl
        )
    }
}
