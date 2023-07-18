use thiserror::Error;

use crate::db::DbContextError;

#[derive(Debug, Error)]
pub enum RepositoryError {
    // crud errors
    #[error("REPOSITORY ERROR :: Entity Not Created :: {0}")]
    NotCreated(String),
    #[error("REPOSITORY ERROR :: Entity Already Exists :: {0}")]
    AlreadyExists(String),
    #[error("REPOSITORY ERROR :: Entity Not Found :: {0}")]
    NotFound(String),
    #[error("REPOSITORY ERROR :: Entity Not Updated :: {0}")]
    NotUpdated(String),
    #[error("REPOSITORY ERROR :: Entity Not Deleted :: {0}")]
    NotDeleted(String),

    // db specific errors
    #[error("REPOSITORY ERROR :: Failed to Establish Connection to Database :: {0}")]
    ConnectionFailed(String),
    #[error("REPOSITORY ERROR :: Database Operation Failed :: {0}")]
    Database(String),
}

impl RepositoryError {
    pub fn map_diesel_create<T: std::fmt::Debug>(create_model: &T, err: diesel::result::Error) -> Self {
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
