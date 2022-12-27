use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReservationError {
    #[error("database error")]
    DbError(#[from] sqlx::Error),
    #[error("invalid time")]
    InvalidTime,
    #[error("unknown error")]
    Unknown,
}
