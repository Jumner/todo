#[derive(Debug, Copy, Clone)]
pub enum Status {
    INVALID,
    INCOMPLETE,
    COMPLETE,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Status::INVALID => write!(f, "Invalid"),
            Status::INCOMPLETE => write!(f, "Incomplete"),
            Status::COMPLETE => write!(f, "Complete"),
        };
    }
}
