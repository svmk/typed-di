use tokio::task::JoinError as TokioJoinError;
use std::fmt;

#[derive(Debug)]
pub struct JoinError {
    error: TokioJoinError,
}

impl JoinError {
    pub fn new(error: TokioJoinError) -> JoinError {
        return JoinError {
            error,
        }
    }
}

impl fmt::Display for JoinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl std::error::Error for JoinError {}