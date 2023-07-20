use diesel::result as DieselResult;
use diesel::result::Error as DieselError;
use redis::{RedisError, ErrorKind as RedisErrorKind};
use thiserror::Error;

use crate::db::DbContextError;

#[derive(Debug, Error)]
pub enum QueryFailure {
    #[error("Entity Not Created")]
    NotCreated,
    #[error("Entity Already Exists")]
    AlreadyExists,
    #[error("Entity Not Found")]
    NotFound,
    #[error("Entity Not Updated")]
    NotUpdated,
    #[error("Entity Not Deleted")]
    NotDeleted,
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("REPOSITORY ERROR :: Database Query Failed :: {0} :: {1}")]
    QueryFailed(String, QueryFailure),

    #[error("REPOSITORY ERROR :: An error occured while attempting to execute a query :: {0}")]
    InternalError(String),
}

impl RepositoryError {
    pub fn map_diesel_create(err: DieselError) -> Self {
        let err_msg = format!("{}", &err);
        match err {
            DieselError::DatabaseError(db_err, _) => match db_err {
                DieselResult::DatabaseErrorKind::UniqueViolation => {
                    Self::QueryFailed(err_msg, QueryFailure::AlreadyExists)
                }
                DieselResult::DatabaseErrorKind::ForeignKeyViolation
                | DieselResult::DatabaseErrorKind::NotNullViolation
                | DieselResult::DatabaseErrorKind::CheckViolation => {
                    Self::QueryFailed(err_msg, QueryFailure::NotCreated)
                }
                _ => Self::InternalError(format!("{}", err)),
            },

            _ => Self::InternalError(format!("{}", err)),
        }
    }

    pub fn map_diesel_found(err: DieselError) -> Self {
        let err_msg = format!("{}", &err);
        match err {
            DieselError::NotFound => Self::QueryFailed(err_msg, QueryFailure::NotFound),
            _ => Self::InternalError(format!("{}", err)),
        }
    }

    pub fn map_diesel_update(err: DieselError) -> Self {
        let err_msg = format!("{}", &err);
        match err {
            DieselError::NotFound => Self::QueryFailed(err_msg, QueryFailure::NotUpdated),

            DieselError::DatabaseError(db_err, _) => match db_err {
                DieselResult::DatabaseErrorKind::UniqueViolation => {
                    Self::QueryFailed(err_msg, QueryFailure::AlreadyExists)
                }
                DieselResult::DatabaseErrorKind::ForeignKeyViolation
                | DieselResult::DatabaseErrorKind::NotNullViolation
                | DieselResult::DatabaseErrorKind::CheckViolation => {
                    Self::QueryFailed(err_msg, QueryFailure::NotUpdated)
                }
                _ => Self::InternalError(err_msg),
            },

            _ => Self::InternalError(err_msg),
        }
    }

    pub fn map_diesel_delete(err: DieselError) -> Self {
        let err_msg = format!("{}", &err);
        match err {
            DieselError::NotFound => Self::QueryFailed(err_msg, QueryFailure::NotDeleted),
            _ => Self::InternalError(err_msg),
        }
    }

    pub fn map_redis_create(err: RedisError) -> Self {
        let msg = err.detail().unwrap_or("Failed to create entity redis").to_string();
        
        Self::InternalError(msg)
    }

    pub fn map_redis(err: RedisError) -> Self {
        let kind = err.kind();
        let msg = err.detail().unwrap_or("Failed to create entity redis").to_string();

        // assume type error is converting nil to struct. annoying error to debug
        // if it is not but ¯\_(ツ)_/¯, TODO later
        match kind {
            RedisErrorKind::TypeError => Self::QueryFailed(msg, QueryFailure::NotFound),
            _ => Self::InternalError(msg),
        }
    }
}

impl From<DbContextError> for RepositoryError {
    fn from(err: DbContextError) -> Self {
        let err_msg = format!("{}", &err);
        Self::InternalError(err_msg)
    }
}
