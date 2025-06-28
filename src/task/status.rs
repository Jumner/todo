#[derive(Debug)]
pub enum Status {
    INCOMPLETE,
    COMPLETE,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Status::INCOMPLETE => write!(f, "Incomplete"),
            Status::COMPLETE => write!(f, "Complete"),
        };
    }
}
