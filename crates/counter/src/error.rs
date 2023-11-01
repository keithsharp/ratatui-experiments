use std::fmt::Display;

#[derive(Debug)]
pub(crate) enum CounterError {
    IoError(std::io::Error),
}

impl Display for CounterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (module, e) = match self {
            CounterError::IoError(e) => ("std::io", e.to_string()),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl std::error::Error for CounterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CounterError::IoError(e) => Some(e),
        }
    }
}

impl From<std::io::Error> for CounterError {
    fn from(e: std::io::Error) -> Self {
        CounterError::IoError(e)
    }
}
