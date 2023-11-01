use std::fmt::Display;

#[derive(Debug)]
pub enum CounterError {
    IoError(std::io::Error),
    EventError(std::sync::mpsc::RecvError),
}

impl Display for CounterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (module, e) = match self {
            CounterError::IoError(e) => ("std::io", e.to_string()),
            CounterError::EventError(e) => ("std::sync::mpsc", e.to_string()),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl std::error::Error for CounterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CounterError::IoError(e) => Some(e),
            CounterError::EventError(e) => Some(e),
        }
    }
}

impl From<std::io::Error> for CounterError {
    fn from(e: std::io::Error) -> Self {
        CounterError::IoError(e)
    }
}

impl From<std::sync::mpsc::RecvError> for CounterError {
    fn from(e: std::sync::mpsc::RecvError) -> Self {
        CounterError::EventError(e)
    }
}
