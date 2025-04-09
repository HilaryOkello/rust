use std::{error::Error, fmt};

// Represents parsing errors for the TODO list JSON.
#[derive(Debug)]
pub enum ParseErr {
    Empty,
    /// Error variant for malformed JSON or incorrect structure.
    /// Contains the underlying error dynamically boxed.
    Malformed(Box<dyn Error>),
}

// Implement the Display trait for ParseErr.
impl fmt::Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse todo file")
    }
}

impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            ParseErr::Empty => None,
            _ => Some(self),
        }
    }
}

// Represents reading errors encountered while accessing the TODO list file.
#[derive(Debug)]
pub struct ReadErr {
    /// The underlying I/O or related error that occurred during reading.
    pub child_err: Box<dyn Error>,
}

// Implement the Display trait for ReadErr.
impl fmt::Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to read todo file")
    }
}

impl Error for ReadErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.child_err.as_ref())
    }
}
