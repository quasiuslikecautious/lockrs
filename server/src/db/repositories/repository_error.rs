use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RepositoryError {
    // crud errors
    NotCreated(String),
    NotFound(String),
    NotUpdated(String),
    NotDeleted(String),

    // db specific errors
    ConnectionFailed(String),
    BadData(String),
}

impl Error for RepositoryError {}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotCreated(msg) => write!(f, "REPOSITORY ERROR :: Entity not created :: {}", msg),
            Self::NotFound(msg) => write!(f, "REPOSITORY ERROR :: Entity not found :: {}", msg),
            Self::NotUpdated(msg) => write!(f, "REPOSITORY ERROR :: Entity not updated :: {}", msg),
            Self::NotDeleted(msg) => write!(f, "REPOSITORY ERROR :: Entity not deleted :: {}", msg),

            Self::ConnectionFailed(msg) => write!(
                f,
                "REPOSITORY ERROR :: Failed to get db connection :: {}",
                msg
            ),
            Self::BadData(msg) => write!(
                f,
                "REPOSITORY ERROR :: Entity is in an invalid format :: {}",
                msg
            ),
        }
    }
}
