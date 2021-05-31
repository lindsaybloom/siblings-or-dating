use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("MongoDB error: {0}")]
    MongoDB(#[from] mongodb::error::Error),
    #[error("No element found in database.")]
    NotFound,
}

impl warp::reject::Reject for Error {}

