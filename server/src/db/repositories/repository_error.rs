pub enum RepositoryError {
    // crud errors
    NotCreated,
    NotFound,
    NotUpdated,
    NotDeleted,

    // db specific errors
    ConnectionFailed,
    BadData,
}
