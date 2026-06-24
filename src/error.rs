use thiserror::Error;

#[derive(Error, Debug)]
#[error("line {line}: {kind}")]
pub struct Error {
    pub line: usize,
    #[source]
    pub kind: ErrorKind,
}

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("malformed line: {0}")]
    MalformedLine(String),

    #[error("key {0} has no params")]
    MalformedSimpleKey(String),

    #[error("replace_string with no param must be prepended by find_string field")]
    MalformedReplaceString(),

    #[error("key {0} can be defined only together with param")]
    MalformedKeyWithParam(String),

    #[error("unclosed parenthesis in key: {0}")]
    UnclosedParen(String),

    #[error("invalid yes/no value: {0}")]
    InvalidBoolValue(String),

    #[error("invalid HTML id/class value: {0}")]
    InvalidIdOrClass(String),

    #[error("strip_id_or_class must not be empty")]
    EmptyStripImageSrc,

    #[error("XPath expression must not be empty")]
    EmptyXPath,

    #[error("invalid XPath expression {expr:?}: {source}")]
    InvalidXPath {
        expr: String,
        #[source]
        source: sxd_xpath::ParserError,
    },
}
