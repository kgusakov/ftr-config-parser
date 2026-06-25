use std::str::FromStr;

use crate::error::{self, ErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Config<'a> {
    pub(crate) title: Vec<XPath<'a>>,
    pub(crate) body: Vec<XPath<'a>>,
    pub(crate) date: Vec<XPath<'a>>,
    pub(crate) author: Vec<XPath<'a>>,
    pub(crate) strip: Vec<XPath<'a>>,
    pub(crate) strip_id_or_class: Vec<IdOrClass<'a>>,
    pub(crate) strip_image_src: Vec<ImageSrcFragment<'a>>,
    pub(crate) prune: YesNo,
    pub(crate) tidy: YesNo,
    pub(crate) autodetect_on_failure: YesNo,
    pub(crate) single_page_link: Option<XPath<'a>>,
    pub(crate) single_page_link_in_feed: Option<XPath<'a>>,
    pub(crate) next_page_link: Option<XPath<'a>>,
    pub(crate) replace_string: Vec<FindReplaceString<'a>>,
    pub(crate) http_header: Vec<HttpHeader<'a>>,
    pub(crate) test_url: Vec<TestUrl<'a>>,
}

impl<'a> Config<'a> {
    #[must_use]
    pub fn title(&self) -> &[XPath<'a>] {
        &self.title
    }
    #[must_use]
    pub fn body(&self) -> &[XPath<'a>] {
        &self.body
    }
    #[must_use]
    pub fn date(&self) -> &[XPath<'a>] {
        &self.date
    }
    #[must_use]
    pub fn author(&self) -> &[XPath<'a>] {
        &self.author
    }
    #[must_use]
    pub fn strip(&self) -> &[XPath<'a>] {
        &self.strip
    }
    #[must_use]
    pub fn strip_id_or_class(&self) -> &[IdOrClass<'a>] {
        &self.strip_id_or_class
    }
    #[must_use]
    pub fn strip_image_src(&self) -> &[ImageSrcFragment<'a>] {
        &self.strip_image_src
    }
    #[must_use]
    pub fn prune(&self) -> YesNo {
        self.prune
    }
    #[must_use]
    pub fn tidy(&self) -> YesNo {
        self.tidy
    }
    #[must_use]
    pub fn autodetect_on_failure(&self) -> YesNo {
        self.autodetect_on_failure
    }
    #[must_use]
    pub fn single_page_link(&self) -> Option<XPath<'a>> {
        self.single_page_link
    }
    #[must_use]
    pub fn single_page_link_in_feed(&self) -> Option<XPath<'a>> {
        self.single_page_link_in_feed
    }
    #[must_use]
    pub fn next_page_link(&self) -> Option<XPath<'a>> {
        self.next_page_link
    }
    #[must_use]
    pub fn replace_string(&self) -> &[FindReplaceString<'a>] {
        &self.replace_string
    }
    #[must_use]
    pub fn http_header(&self) -> &[HttpHeader<'a>] {
        &self.http_header
    }
    #[must_use]
    pub fn test_url(&self) -> &[TestUrl<'a>] {
        &self.test_url
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct XPath<'a>(pub(crate) &'a str);

impl<'a> TryFrom<&'a str> for XPath<'a> {
    type Error = error::ErrorKind;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        if s.is_empty() {
            return Err(error::ErrorKind::EmptyXPath);
        }
        sxd_xpath::Factory::new()
            .build(s)
            .map_err(|e| error::ErrorKind::InvalidXPath {
                expr: s.to_string(),
                source: e,
            })?;
        Ok(XPath(s))
    }
}

impl<'a> XPath<'a> {
    /// Returns the underlying `XPath` expression.
    #[must_use]
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}

impl AsRef<str> for XPath<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl std::fmt::Display for XPath<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct IdOrClass<'a>(pub(crate) &'a str);

impl<'a> TryFrom<&'a str> for IdOrClass<'a> {
    type Error = error::ErrorKind;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        if s.is_empty() {
            return Err(error::ErrorKind::InvalidIdOrClass(s.to_string()));
        }
        if s.chars().any(|c| c.is_ascii_whitespace()) {
            return Err(error::ErrorKind::InvalidIdOrClass(s.to_string()));
        }
        Ok(IdOrClass(s))
    }
}

impl<'a> IdOrClass<'a> {
    /// Returns the underlying id or class value.
    #[must_use]
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}

impl AsRef<str> for IdOrClass<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl std::fmt::Display for IdOrClass<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ImageSrcFragment<'a>(pub(crate) &'a str);

impl<'a> TryFrom<&'a str> for ImageSrcFragment<'a> {
    type Error = error::ErrorKind;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        if s.is_empty() {
            return Err(ErrorKind::EmptyStripImageSrc);
        }

        Ok(ImageSrcFragment(s))
    }
}

impl<'a> ImageSrcFragment<'a> {
    /// Returns the underlying image-src fragment.
    #[must_use]
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}

impl AsRef<str> for ImageSrcFragment<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl std::fmt::Display for ImageSrcFragment<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum YesNo {
    Yes,
    No,
}

impl From<bool> for YesNo {
    fn from(value: bool) -> Self {
        if value { Self::Yes } else { Self::No }
    }
}

impl From<YesNo> for bool {
    fn from(value: YesNo) -> Self {
        match value {
            YesNo::Yes => true,
            YesNo::No => false,
        }
    }
}

impl FromStr for YesNo {
    type Err = error::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "yes" => Ok(YesNo::Yes),
            "no" => Ok(YesNo::No),
            other => Err(error::ErrorKind::InvalidBoolValue(other.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FindString<'a>(pub(crate) &'a str);

impl<'a> TryFrom<&'a str> for FindString<'a> {
    type Error = error::ErrorKind;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        if s.is_empty() {
            return Err(ErrorKind::EmptyFindString);
        }

        Ok(FindString(s))
    }
}

impl<'a> FindString<'a> {
    /// Returns the underlying find string.
    #[must_use]
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}

impl AsRef<str> for FindString<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl std::fmt::Display for FindString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FindReplaceString<'a> {
    pub(crate) find: FindString<'a>,
    pub(crate) replace: &'a str,
}

impl<'a> FindReplaceString<'a> {
    /// Returns the find part of the replacement pair.
    #[must_use]
    pub fn find(&self) -> FindString<'a> {
        self.find
    }

    /// Returns the replacement string.
    #[must_use]
    pub fn replace(&self) -> &'a str {
        self.replace
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct HttpHeader<'a> {
    pub(crate) name: &'a str,
    pub(crate) value: &'a str,
}

impl<'a> HttpHeader<'a> {
    /// Returns the header name.
    #[must_use]
    pub fn name(&self) -> &'a str {
        self.name
    }

    /// Returns the header value.
    #[must_use]
    pub fn value(&self) -> &'a str {
        self.value
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TestUrl<'a>(pub(crate) &'a str);

impl<'a> TestUrl<'a> {
    /// Returns the underlying test URL.
    #[must_use]
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}

impl AsRef<str> for TestUrl<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl std::fmt::Display for TestUrl<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

#[cfg(test)]
mod tests {
    use core::assert_matches;

    use super::*;

    mod yes_no {
        use super::*;

        #[test]
        fn parses_yes() {
            assert_matches!("yes".parse::<YesNo>().unwrap(), YesNo::Yes);
        }

        #[test]
        fn parses_no() {
            assert_matches!("no".parse::<YesNo>().unwrap(), YesNo::No);
        }

        #[test]
        fn rejects_unknown() {
            assert_matches!(
                "maybe".parse::<YesNo>(),
                Err(ErrorKind::InvalidBoolValue(_))
            );
        }

        #[test]
        fn rejects_empty() {
            assert_matches!("".parse::<YesNo>(), Err(ErrorKind::InvalidBoolValue(_)));
        }
    }

    mod id_or_class {
        use super::*;

        #[test]
        fn valid() {
            assert!(IdOrClass::try_from("sidebar").is_ok());
            assert!(IdOrClass::try_from("main-content").is_ok());
            assert!(IdOrClass::try_from("_private").is_ok());
            assert!(IdOrClass::try_from("item123").is_ok());
        }

        #[test]
        fn rejects_empty() {
            assert_matches!(IdOrClass::try_from(""), Err(ErrorKind::InvalidIdOrClass(_)));
        }

        #[test]
        fn rejects_whitespace() {
            assert_matches!(
                IdOrClass::try_from("foo bar"),
                Err(ErrorKind::InvalidIdOrClass(_))
            );
            assert_matches!(
                IdOrClass::try_from("foo\tbar"),
                Err(ErrorKind::InvalidIdOrClass(_))
            );
            assert_matches!(
                IdOrClass::try_from(" leading"),
                Err(ErrorKind::InvalidIdOrClass(_))
            );
        }
    }

    mod xpath {
        use super::*;

        #[test]
        fn rejects_empty() {
            assert_matches!(XPath::try_from(""), Err(ErrorKind::EmptyXPath));
        }

        #[test]
        fn rejects_invalid_expression() {
            assert_matches!(XPath::try_from("["), Err(ErrorKind::InvalidXPath { .. }));
        }
    }
}
