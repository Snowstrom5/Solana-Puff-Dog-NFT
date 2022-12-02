use async_graphql::{ErrorExtensions, FieldError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GqlError {
    #[error("Could not find resource")]
    NotFound,

    #[error("ServerError")]
    ServerError(String),

    #[error("No Extensions")]
    ErrorWithoutExtensions,
}

impl ErrorExtensions for GqlError {
    // base extensions
    fn extend(&self) -> FieldError {
        self.extend_with(|err, e| match err {
            GqlError::NotFound => e.set("code", "NOT_FOUND"),
            GqlError::ServerError(reason) => e.set("reason", reason.to_string()),
            GqlError::ErrorWithoutExtensions => {}
        })
    }
}
