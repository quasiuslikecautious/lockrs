use diesel::result as DieselResult;
use diesel::result::Error as DieselError;
use redis::{ErrorKind as RedisErrorKind, RedisError};
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
    #[error("REPOSITORY ERROR :: Database Query Failed")]
    QueryFailed(QueryFailure),

    #[error("REPOSITORY ERROR :: An error occured while attempting to execute a query")]
    InternalError,
}

impl RepositoryError {
    pub fn map_diesel_create(err: DieselError) -> Self {
        let _err_msg = format!("{}", &err);
        tracing::error!(error = %err);

        match err {
            DieselError::DatabaseError(db_err, _) => match db_err {
                DieselResult::DatabaseErrorKind::UniqueViolation => {
                    Self::QueryFailed(QueryFailure::AlreadyExists)
                }
                DieselResult::DatabaseErrorKind::ForeignKeyViolation
                | DieselResult::DatabaseErrorKind::NotNullViolation
                | DieselResult::DatabaseErrorKind::CheckViolation => {
                    Self::QueryFailed(QueryFailure::NotCreated)
                }
                _ => Self::InternalError,
            },

            _ => Self::InternalError,
        }
    }

    pub fn map_diesel_found(err: DieselError) -> Self {
        let _err_msg = format!("{}", &err);
        tracing::error!(error = %err);
        match err {
            DieselError::NotFound => Self::QueryFailed(QueryFailure::NotFound),
            _ => Self::InternalError,
        }
    }

    pub fn map_diesel_update(err: DieselError) -> Self {
        let _err_msg = format!("{}", &err);
        tracing::error!(error = %err);
        match err {
            DieselError::NotFound => Self::QueryFailed(QueryFailure::NotUpdated),

            DieselError::DatabaseError(db_err, _) => match db_err {
                DieselResult::DatabaseErrorKind::UniqueViolation => {
                    Self::QueryFailed(QueryFailure::AlreadyExists)
                }
                DieselResult::DatabaseErrorKind::ForeignKeyViolation
                | DieselResult::DatabaseErrorKind::NotNullViolation
                | DieselResult::DatabaseErrorKind::CheckViolation => {
                    Self::QueryFailed(QueryFailure::NotUpdated)
                }
                _ => Self::InternalError,
            },

            _ => Self::InternalError,
        }
    }

    pub fn map_diesel_delete(err: DieselError) -> Self {
        let _err_msg = format!("{}", &err);
        tracing::error!(error = %err);
        match err {
            DieselError::NotFound => Self::QueryFailed(QueryFailure::NotDeleted),
            _ => Self::InternalError,
        }
    }

    pub fn map_redis_create(err: RedisError) -> Self {
        let msg = err
            .detail()
            .unwrap_or("Failed to create entity redis")
            .to_string();

        tracing::error!(error = msg);

        Self::InternalError
    }

    pub fn map_redis(err: RedisError) -> Self {
        let kind = err.kind();
        let msg = err
            .detail()
            .unwrap_or("Failed to create entity redis")
            .to_string();

        tracing::error!(error = msg);

        // assume type error is converting nil to struct. annoying error to debug
        // if it is not but ¯\_(ツ)_/¯, TODO later
        match kind {
            RedisErrorKind::TypeError => Self::QueryFailed(QueryFailure::NotFound),
            _ => Self::InternalError,
        }
    }
}

impl From<DbContextError> for RepositoryError {
    fn from(err: DbContextError) -> Self {
        tracing::error!(eror = %err);

        let _err_msg = format!("{}", &err);
        Self::InternalError
    }
}
