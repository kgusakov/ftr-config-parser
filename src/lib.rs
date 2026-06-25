mod error;
mod parser;
mod types;

pub use error::{Error, ErrorKind};
pub use parser::parse_config;
pub use types::{
    Config, FindReplaceString, FindString, HttpHeader, IdOrClass, ImageSrcFragment, TestUrl, XPath,
    YesNo,
};
