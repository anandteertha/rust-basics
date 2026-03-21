#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub enum Severity {
    Info,
    Warning,
    Critical,
    Unknown,
}

impl Severity {
    pub fn get_severity(text: &str) -> Self {
        match text {
            "critical" => Self::Critical,
            "warning" => Self::Warning,
            "info" => Self::Info,
            _ => Self::Unknown,
        }
    }
}
