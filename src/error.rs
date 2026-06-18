use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("malformed line (missing `:`): {0}")]
    MalformedLine(String),

    #[error("invalid yes/no value: {0}")]
    InvalidBoolValue(String),

    #[error("invalid HTML id/class value: {0}")]
    InvalidIdOrClass(String),

    #[error("XPath expression must not be empty")]
    EmptyXPath,

    #[error("invalid XPath expression {expr:?}: {source}")]
    InvalidXPath {
        expr: String,
        #[source]
        source: sxd_xpath::ParserError,
    },
}
