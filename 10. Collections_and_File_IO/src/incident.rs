use crate::{severity::Severity, status::Status};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub struct Incident {
    severity: Severity,
    service: String,
    status: Status,
    message: String,
}

impl Incident {
    pub fn get_status(&self) -> Status {
        self.status
    }
    pub fn new(line: &str) -> Result<Self, String> {
        let items: Vec<&str> = line.split('|').collect();
        if items.len() != 4 {
            return Err(format!(
                "Expected 4 fields, but found {} in line : {}",
                items.len(),
                line
            ));
        }
        let severity = Severity::get_severity(items[0].trim());
        if matches!(severity, Severity::Unknown) {
            return Err(format!("Invalid Severity: '{}'", items[0].trim()));
        }
        let service = items[1].trim().to_owned();
        if service.is_empty() {
            return Err(String::from("Service name cannot be empty"));
        }
        let status = Status::get_status(items[2].trim());
        if matches!(status, Status::Unknown) {
            return Err(format!("Invalid Status: '{}'", items[2].trim()));
        }
        let message = items[3].trim().to_owned();
        if message.is_empty() {
            return Err(String::from("Message cannot be empty"));
        }
        Ok(Self {
            severity,
            service,
            status,
            message,
        })
    }

    pub fn count_by_severity(incidents: &[Self]) -> HashMap<Severity, usize> {
        Self::generic_count(incidents, |incident| incident.severity)
    }
    pub fn count_by_status(incidents: &[Self]) -> HashMap<Status, usize> {
        Self::generic_count(incidents, |incident| incident.status)
    }
    pub fn count_by_service(incidents: &[Self]) -> HashMap<String, usize> {
        Self::generic_count(incidents, |incident| incident.service.clone())
    }

    pub fn generic_count<T, F>(incidents: &[Self], mut key_fn: F) -> HashMap<T, usize>
    where
        T: Eq + Hash,
        F: FnMut(&Self) -> T,
    {
        let mut counts: HashMap<T, usize> = HashMap::new();
        for incident in incidents {
            let key = key_fn(incident);
            counts
                .entry(key)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        counts
    }

    pub fn print_summary<T>(counts: HashMap<T, usize>)
    where
        T: Eq + Hash + Debug,
    {
        for (key, value) in counts {
            println!("{:?}: {}", key, value);
        }
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }
}
