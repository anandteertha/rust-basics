use crate::alert_level::AlertLevel;

#[derive(Debug)]
pub struct Alert {
    service_name: String,
    alert_level: AlertLevel,
    message: String,
    is_resolved: bool,
    severity_score: u8,
}

impl Alert {
    pub fn get_alert_level(&self) -> &AlertLevel {
        &self.alert_level
    }

    pub fn get_is_resolved(&self) -> bool {
        self.is_resolved
    }

    fn get_resolve_string(&self) -> String {
        if self.is_resolved {
            String::from("resolved")
        } else {
            String::from("unresolved")
        }
    }

    pub fn print_alert(&self) {
        let resolve_string = self.get_resolve_string();
        println!(
            "{:?}: {}({}) with severity {} is {}",
            self.alert_level, self.service_name, self.message, self.severity_score, resolve_string
        );
    }

    pub fn get_summary(&self) -> String {
        format!(
            "name: {}\nAlert Level: {:?}\nMessage: {}\nResolved: {}\nSeverity Score: {}\n",
            self.service_name,
            self.alert_level,
            self.message,
            self.get_resolve_string(),
            self.severity_score
        )
    }
}

pub fn get_mock_alerts() -> Vec<Alert> {
    vec![
        Alert {
            service_name: String::from("Database"),
            alert_level: AlertLevel::Critical,
            message: String::from("Database is on fire"),
            is_resolved: false,
            severity_score: 8,
        },
        Alert {
            service_name: String::from("Auth"),
            alert_level: AlertLevel::Warning,
            message: String::from("Someone un-authenticated tried to access the service"),
            is_resolved: true,
            severity_score: 5,
        },
        Alert {
            service_name: String::from("Payments"),
            alert_level: AlertLevel::Critical,
            message: String::from("Payment failed due to some issue"),
            is_resolved: true,
            severity_score: 8,
        },
        Alert {
            service_name: String::from("Cache"),
            alert_level: AlertLevel::Info,
            message: String::from("Cache miss due to new key fetch!"),
            is_resolved: false,
            severity_score: 3,
        },
        Alert {
            service_name: String::from("API"),
            alert_level: AlertLevel::Warning,
            message: String::from("Slow requests are creating higher latency in the system"),
            is_resolved: false,
            severity_score: 6,
        },
    ]
}
