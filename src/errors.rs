use std::{error::Error, fmt};

#[derive(Debug)]
pub enum RusterError{
    Of(ErrorType)
}

impl fmt::Display for RusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A network error has occurred")
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorType {
    Network,
    MethodNotSupported,
    ReadingDirectory,
    IO
}

// Implement std::fmt::Display for AppError
impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
            ErrorType::Network => write!(f, "A network error has occurred"),
            ErrorType::MethodNotSupported => write!(f, "HTTP Method not supported"),
            ErrorType::ReadingDirectory => write!(f, "Failed to read spec files from directory"),
            ErrorType::IO => write!(f, "An error which needs to be better described"),
        }
    }
}

impl From<reqwest::Error> for RusterError {
    fn from(_: reqwest::Error) -> RusterError {
       return RusterError::Of(ErrorType::Network);
    }
}

impl From<std::io::Error> for RusterError {
    fn from(_: std::io::Error) -> RusterError {
       return RusterError::Of(ErrorType::IO);
    }
}

impl Error for RusterError{}
