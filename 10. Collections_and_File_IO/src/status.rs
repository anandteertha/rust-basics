#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub enum Status {
    Resolved,
    Unresolved,
    Unknown,
}

impl Status {
    pub fn get_status(text: &str) -> Self {
        match text {
            "resolved" => Self::Resolved,
            "unresolved" => Self::Unresolved,
            _ => Self::Unknown,
        }
    }
}
