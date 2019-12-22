use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct EngineError {
    details: String,
}

impl EngineError {
    pub fn init(details: String) -> EngineError {
        EngineError { details }
    }
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for EngineError {
    fn description(&self) -> &str {
        &self.details
    }
}
