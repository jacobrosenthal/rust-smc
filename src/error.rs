use std::error::Error;
use std::ffi::NulError;
use std::fmt;

pub type SmcResult<T> = Result<T, SmcError>;

#[derive(Debug)]
pub struct SmcError {
    details: String,
}

impl SmcError {
    pub fn new(msg: &str) -> SmcError {
        SmcError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for SmcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for SmcError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<NulError> for SmcError {
    fn from(err: NulError) -> Self {
        SmcError::new(err.description())
    }
}
