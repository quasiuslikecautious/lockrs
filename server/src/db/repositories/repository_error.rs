use std::error::Error;
use std::fmt;

use crate::db::DbContextError;

#[derive(Debug)]
pub enum RepositoryError {
    // crud errors
    NotCreated(String),
    AlreadyExists(String),

    NotFound(String),

    NotUpdated(String),

    NotDeleted(String),

    // db specific errors
    ConnectionFailed(String),
    Database(String),
}

impl RepositoryError {
    pub fn map_diesel_create<T: fmt::Debug>(create_model: &T, err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::DatabaseError(db_err, _) => match db_err {
                diesel::result::DatabaseErrorKind::UniqueViolation => {
                    Self::AlreadyExists(format!("{:?}", create_model))
                }
                diesel::result::DatabaseErrorKind::ForeignKeyViolation
                | diesel::result::DatabaseErrorKind::NotNullViolation
                | diesel::result::DatabaseErrorKind::CheckViolation => {
                    Self::NotFound(format!("{:?}", create_model))
                }
                _ => Self::Database(format!("{}", err)),
            },
            _ => Self::Database(format!("{}", err)),
        }
    }

    pub fn map_diesel_found(id: &str, err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => Self::NotFound(format!("{}", id)),
            _ => Self::Database(format!("{}", err)),
        }
    }

    pub fn map_diesel_update(id: &str, err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => Self::NotUpdated(format!("{}", id)),
            diesel::result::Error::DatabaseError(db_err, _) => match db_err {
                diesel::result::DatabaseErrorKind::UniqueViolation => {
                    Self::AlreadyExists(format!("{}", id))
                }
                diesel::result::DatabaseErrorKind::ForeignKeyViolation
                | diesel::result::DatabaseErrorKind::NotNullViolation
                | diesel::result::DatabaseErrorKind::CheckViolation => {
                    Self::NotUpdated(format!("{}", err))
                }
                _ => Self::Database(format!("{}", err)),
            },
            _ => Self::Database(format!("{}", err)),
        }
    }

    pub fn map_diesel_delete(id: &str, err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => Self::NotDeleted(format!("{}", id)),
            _ => Self::Database(format!("{}", err)),
        }
    }
}

impl Error for RepositoryError {}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotCreated(msg) => write!(f, "REPOSITORY ERROR :: Entity not created :: {}", msg),
            Self::AlreadyExists(msg) => {
                write!(f, "REPOSITORY ERROR :: Entity already exists :: {}", msg)
            }
            Self::NotFound(msg) => write!(f, "REPOSITORY ERROR :: Entity not found :: {}", msg),
            Self::NotUpdated(msg) => write!(f, "REPOSITORY ERROR :: Entity not updated :: {}", msg),
            Self::NotDeleted(msg) => write!(f, "REPOSITORY ERROR :: Entity not deleted :: {}", msg),

            Self::ConnectionFailed(msg) => write!(
                f,
                "REPOSITORY ERROR :: Failed to get db connection :: {}",
                msg
            ),
            Self::Database(msg) => write!(
                f,
                "REPOSITORY ERROR :: An error has occured from the database :: {}",
                msg
            ),
        }
    }
}
