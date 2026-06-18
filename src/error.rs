use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("malformed line (missing `:`): {0}")]
    MalformedLine(String),

    #[error("invalid yes/no value: {0}")]
    InvalidBoolValue(String),
}
