use std::str::FromStr;

mod error;

pub use error::{Error, ErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Config<'a> {
    pub title: Vec<XPath<'a>>,
    pub body: Vec<XPath<'a>>,
    pub date: Vec<XPath<'a>>,
    pub author: Vec<XPath<'a>>,
    pub strip: Vec<XPath<'a>>,
    pub strip_id_or_class: Vec<IdOrClass<'a>>,
    pub strip_image_src: Vec<ImageSrcFragment<'a>>,
    pub prune: YesNo,
    pub tidy: YesNo,
    pub autodetect_on_failure: YesNo,
    pub single_page_link: Option<XPath<'a>>,
    pub single_page_link_in_feed: Option<XPath<'a>>,
    pub next_page_link: Option<XPath<'a>>,
    pub replace_string: Vec<ReplaceString<'a>>,
    pub http_header: Vec<HttpHeader<'a>>,
    pub test_url: Vec<TestUrl<'a>>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct XPath<'a>(pub &'a str);

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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct IdOrClass<'a>(pub &'a str);

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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ImageSrcFragment<'a>(pub &'a str);

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
pub struct ReplaceString<'a> {
    pub find: &'a str,
    pub replace: &'a str,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct HttpHeader<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TestUrl<'a>(pub &'a str);

fn parse_line(line: &str) -> Result<(&str, Option<&str>, &str), error::ErrorKind> {
    let colon = line
        .find(':')
        .ok_or_else(|| error::ErrorKind::MalformedLine(line.to_string()))?;
    let key_part = line[..colon].trim();
    let value = line[colon + 1..].trim();

    if let Some(open) = key_part.find('(') {
        let close = key_part
            .find(')')
            .ok_or_else(|| error::ErrorKind::UnclosedParen(key_part.to_string()))?;
        let name = key_part[..open].trim();
        let param = key_part[open + 1..close].trim();
        Ok((name, Some(param), value))
    } else {
        Ok((key_part, None, value))
    }
}

/// Parse an ftr site-config file from its text content.
///
/// # Errors
///
/// Returns [`Error`] when a line value fails validation
/// The error includes the 1-based line number of the offending line.
pub fn parse_config(input: &str) -> Result<Config<'_>, Error> {
    let mut title = Vec::new();
    let mut body = Vec::new();
    let mut date = Vec::new();
    let mut author = Vec::new();
    let mut strip = Vec::new();
    let mut strip_id_or_class = Vec::new();
    let mut strip_image_src = Vec::new();
    let mut prune = YesNo::Yes;
    let mut tidy = YesNo::No;
    let mut autodetect_on_failure = YesNo::Yes;
    let mut single_page_link = None;
    let mut single_page_link_in_feed = None;
    let mut next_page_link = None;
    let mut replace_string = Vec::new();
    let mut http_header = Vec::new();
    let mut test_url = Vec::new();

    for (i, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let locate_err = |kind| error::Error { line: i + 1, kind };

        let (name, param, value) = parse_line(line).map_err(&locate_err)?;

        match name {
            "title" => title.push(XPath::try_from(value).map_err(&locate_err)?),
            "body" => body.push(XPath::try_from(value).map_err(&locate_err)?),
            "date" => date.push(XPath::try_from(value).map_err(&locate_err)?),
            "author" => author.push(XPath::try_from(value).map_err(&locate_err)?),
            "strip" => strip.push(XPath::try_from(value).map_err(&locate_err)?),
            "strip_id_or_class" => {
                for token in value.split_ascii_whitespace() {
                    strip_id_or_class.push(IdOrClass::try_from(token).map_err(&locate_err)?);
                }
            }
            "strip_image_src" => strip_image_src.push(ImageSrcFragment(value)),
            "prune" => prune = value.parse().map_err(&locate_err)?,
            "tidy" => tidy = value.parse().map_err(&locate_err)?,
            "autodetect_on_failure" => {
                autodetect_on_failure = value.parse().map_err(&locate_err)?;
            }
            "single_page_link" => {
                single_page_link = Some(XPath::try_from(value).map_err(&locate_err)?);
            }
            "single_page_link_in_feed" => {
                single_page_link_in_feed = Some(XPath::try_from(value).map_err(&locate_err)?);
            }
            "next_page_link" => next_page_link = Some(XPath::try_from(value).map_err(&locate_err)?),
            "replace_string" => replace_string.push(ReplaceString {
                find: param.unwrap_or(""),
                replace: value,
            }),
            "http_header" => http_header.push(HttpHeader {
                name: param.unwrap_or(""),
                value,
            }),
            "test_url" => test_url.push(TestUrl(value)),
            _ => {}
        }
    }

    Ok(Config {
        title,
        body,
        date,
        author,
        strip,
        strip_id_or_class,
        strip_image_src,
        prune,
        tidy,
        autodetect_on_failure,
        single_page_link,
        single_page_link_in_feed,
        next_page_link,
        replace_string,
        http_header,
        test_url,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_line {
        use super::*;

        #[test]
        fn simple_key() {
            assert_eq!(
                parse_line("body: //div[@id='content']").unwrap(),
                ("body", None, "//div[@id='content']")
            );
        }

        #[test]
        fn parametric_key() {
            assert_eq!(
                parse_line("http_header(Cookie): euConsent=true").unwrap(),
                ("http_header", Some("Cookie"), "euConsent=true")
            );
        }

        #[test]
        fn value_with_colon() {
            assert_eq!(
                parse_line("http_header(User-agent): Mozilla/5.0 (iPad; CPU OS 12_0_1)").unwrap(),
                (
                    "http_header",
                    Some("User-agent"),
                    "Mozilla/5.0 (iPad; CPU OS 12_0_1)"
                )
            );
        }

        #[test]
        fn no_colon_returns_error() {
            assert!(matches!(
                parse_line("badline"),
                Err(ErrorKind::MalformedLine(_))
            ));
        }

        #[test]
        fn unclosed_paren_returns_error() {
            assert!(matches!(
                parse_line("http_header(Cookie: value"),
                Err(ErrorKind::UnclosedParen(ref s)) if s == "http_header(Cookie"
            ));
        }
    }

    mod yes_no {
        use super::*;

        #[test]
        fn parses_yes() {
            assert!(matches!("yes".parse::<YesNo>().unwrap(), YesNo::Yes));
        }

        #[test]
        fn parses_no() {
            assert!(matches!("no".parse::<YesNo>().unwrap(), YesNo::No));
        }

        #[test]
        fn rejects_unknown() {
            assert!(matches!(
                "maybe".parse::<YesNo>(),
                Err(ErrorKind::InvalidBoolValue(_))
            ));
        }

        #[test]
        fn rejects_empty() {
            assert!(matches!(
                "".parse::<YesNo>(),
                Err(ErrorKind::InvalidBoolValue(_))
            ));
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
            assert!(matches!(
                IdOrClass::try_from(""),
                Err(ErrorKind::InvalidIdOrClass(_))
            ));
        }

        #[test]
        fn rejects_whitespace() {
            assert!(matches!(
                IdOrClass::try_from("foo bar"),
                Err(ErrorKind::InvalidIdOrClass(_))
            ));
            assert!(matches!(
                IdOrClass::try_from("foo\tbar"),
                Err(ErrorKind::InvalidIdOrClass(_))
            ));
            assert!(matches!(
                IdOrClass::try_from(" leading"),
                Err(ErrorKind::InvalidIdOrClass(_))
            ));
        }
    }

    mod xpath {
        use super::*;

        #[test]
        fn rejects_empty() {
            assert!(matches!(XPath::try_from(""), Err(ErrorKind::EmptyXPath)));
        }

        #[test]
        fn rejects_invalid_expression() {
            assert!(matches!(
                XPath::try_from("["),
                Err(ErrorKind::InvalidXPath { .. })
            ));
        }
    }

    mod general {
        use super::*;

        #[test]
        fn empty_input_returns_defaults() {
            assert_eq!(
                parse_config("").unwrap(),
                Config {
                    title: vec![],
                    body: vec![],
                    date: vec![],
                    author: vec![],
                    strip: vec![],
                    strip_id_or_class: vec![],
                    strip_image_src: vec![],
                    prune: YesNo::Yes,
                    tidy: YesNo::No,
                    autodetect_on_failure: YesNo::Yes,
                    single_page_link: None,
                    single_page_link_in_feed: None,
                    next_page_link: None,
                    replace_string: vec![],
                    http_header: vec![],
                    test_url: vec![],
                }
            );
        }

        #[test]
        fn comments_and_blank_lines_ignored() {
            let config = parse_config("#title: //div[@id='title']\n\n# another comment\n").unwrap();
            assert_eq!(config.title, vec![]);
        }

        #[test]
        fn unknown_keys_silently_skipped() {
            let config = parse_config("unknown_directive: some value\n").unwrap();
            assert_eq!(config.title, vec![]);
        }

        #[test]
        fn malformed_line_no_colon() {
            assert!(matches!(
                parse_config("no colon here").unwrap_err(),
                Error {
                    kind: ErrorKind::MalformedLine(_),
                    line: 1
                }
            ));
        }

        #[test]
        fn error_reports_correct_line_number() {
            let input = "body: //article\nprune: oops\n";
            let err = parse_config(input).unwrap_err();
            assert!(matches!(
                err,
                Error {
                    kind: ErrorKind::InvalidBoolValue(_),
                    line: 2
                }
            ));
        }

        #[test]
        fn error_line_number_skips_blanks_and_comments() {
            let input = "# comment\n\nbody: //article\nprune: oops\n";
            let err = parse_config(input).unwrap_err();
            // line 4 in the raw input (comment=1, blank=2, body=3, prune=4)
            assert!(matches!(
                err,
                Error {
                    kind: ErrorKind::InvalidBoolValue(_),
                    line: 4
                }
            ));
        }
    }

    mod title {
        use super::*;

        #[test]
        fn single_value() {
            let config = parse_config("title: //h1\n").unwrap();
            assert_eq!(config.title, vec![XPath("//h1")]);
        }

        #[test]
        fn multiple_values() {
            let config = parse_config("title: //h1\ntitle: //h2\n").unwrap();
            assert_eq!(config.title, vec![XPath("//h1"), XPath("//h2")]);
        }
    }

    mod body {
        use super::*;

        #[test]
        fn single_value() {
            let config = parse_config("body: //div[@id='content']\n").unwrap();
            assert_eq!(config.body, vec![XPath("//div[@id='content']")]);
        }

        #[test]
        fn multiple_values() {
            let config = parse_config("body: //article\nbody: //main\n").unwrap();
            assert_eq!(config.body, vec![XPath("//article"), XPath("//main")]);
        }
    }

    mod date {
        use super::*;

        #[test]
        fn single_value() {
            let config = parse_config("date: //time[@class='published']\n").unwrap();
            assert_eq!(config.date, vec![XPath("//time[@class='published']")]);
        }

        #[test]
        fn multiple_values() {
            let config = parse_config("date: //time\ndate: //span[@class='date']\n").unwrap();
            assert_eq!(
                config.date,
                vec![XPath("//time"), XPath("//span[@class='date']")]
            );
        }
    }

    mod author {
        use super::*;

        #[test]
        fn single_value() {
            let config = parse_config("author: //span[@class='author']\n").unwrap();
            assert_eq!(config.author, vec![XPath("//span[@class='author']")]);
        }

        #[test]
        fn multiple_values() {
            let config =
                parse_config("author: //span[@class='author']\nauthor: //a[@rel='author']\n")
                    .unwrap();
            assert_eq!(
                config.author,
                vec![
                    XPath("//span[@class='author']"),
                    XPath("//a[@rel='author']")
                ]
            );
        }
    }

    mod strip {
        use super::*;

        #[test]
        fn single_value() {
            let config = parse_config("strip: //div[@class='ad']\n").unwrap();
            assert_eq!(config.strip, vec![XPath("//div[@class='ad']")]);
        }

        #[test]
        fn multiple_values() {
            let config = parse_config("strip: //div[@class='ad']\nstrip: //aside\n").unwrap();
            assert_eq!(
                config.strip,
                vec![XPath("//div[@class='ad']"), XPath("//aside")]
            );
        }
    }

    mod strip_id_or_class {
        use super::*;

        #[test]
        fn empty_value() {
            // Empty value yields no tokens — field stays empty, no error
            let config = parse_config("strip_id_or_class: \n").unwrap();
            assert_eq!(config.strip_id_or_class, vec![]);
        }

        #[test]
        fn single_value() {
            let config = parse_config("strip_id_or_class: sidebar\n").unwrap();
            assert_eq!(config.strip_id_or_class, vec![IdOrClass("sidebar")]);
        }

        #[test]
        fn multiple_values() {
            let config =
                parse_config("strip_id_or_class: foo bar\nstrip_id_or_class: baz\n").unwrap();
            assert_eq!(
                config.strip_id_or_class,
                vec![IdOrClass("foo"), IdOrClass("bar"), IdOrClass("baz")]
            );
        }

        #[test]
        fn multiple_values_per_line() {
            let config = parse_config("strip_id_or_class: foo bar baz").unwrap();
            assert_eq!(
                config.strip_id_or_class,
                vec![IdOrClass("foo"), IdOrClass("bar"), IdOrClass("baz")]
            );
        }
    }

    mod strip_image_src {
        use super::*;

        #[test]
        fn empty_value() {
            let config = parse_config("strip_image_src: \n").unwrap();
            assert_eq!(config.strip_image_src, vec![ImageSrcFragment("")]);
        }

        #[test]
        fn single_value() {
            let config = parse_config("strip_image_src: /img/ad\n").unwrap();
            assert_eq!(config.strip_image_src, vec![ImageSrcFragment("/img/ad")]);
        }

        #[test]
        fn multiple_values() {
            let config =
                parse_config("strip_image_src: /img/ad\nstrip_image_src: /banner/\n").unwrap();
            assert_eq!(
                config.strip_image_src,
                vec![ImageSrcFragment("/img/ad"), ImageSrcFragment("/banner/")]
            );
        }
    }

    mod replace_string {
        use super::*;

        #[test]
        fn empty_value() {
            let config = parse_config("replace_string(foo): \n").unwrap();
            assert_eq!(
                config.replace_string,
                vec![ReplaceString {
                    find: "foo",
                    replace: ""
                }]
            );
        }

        #[test]
        fn single_value() {
            let config = parse_config("replace_string(foo): bar\n").unwrap();
            assert_eq!(
                config.replace_string,
                vec![ReplaceString {
                    find: "foo",
                    replace: "bar"
                }]
            );
        }

        #[test]
        fn multiple_values() {
            let config =
                parse_config("replace_string(foo): bar\nreplace_string(baz): qux\n").unwrap();
            assert_eq!(
                config.replace_string,
                vec![
                    ReplaceString {
                        find: "foo",
                        replace: "bar"
                    },
                    ReplaceString {
                        find: "baz",
                        replace: "qux"
                    },
                ]
            );
        }
    }

    mod http_header {
        use super::*;

        #[test]
        fn empty_value() {
            let config = parse_config("http_header(Cookie): \n").unwrap();
            assert_eq!(
                config.http_header,
                vec![HttpHeader {
                    name: "Cookie",
                    value: ""
                }]
            );
        }

        #[test]
        fn single_value() {
            let config = parse_config("http_header(Cookie): euConsent=true\n").unwrap();
            assert_eq!(
                config.http_header,
                vec![HttpHeader {
                    name: "Cookie",
                    value: "euConsent=true"
                }]
            );
        }

        #[test]
        fn multiple_values() {
            let config = parse_config(
                "http_header(Cookie): euConsent=true\nhttp_header(User-agent): Mozilla/5.0\n",
            )
            .unwrap();
            assert_eq!(
                config.http_header,
                vec![
                    HttpHeader {
                        name: "Cookie",
                        value: "euConsent=true"
                    },
                    HttpHeader {
                        name: "User-agent",
                        value: "Mozilla/5.0"
                    },
                ]
            );
        }
    }

    mod test_url {
        use super::*;

        #[test]
        fn empty_value() {
            let config = parse_config("test_url: \n").unwrap();
            assert_eq!(config.test_url, vec![TestUrl("")]);
        }

        #[test]
        fn single_value() {
            let config = parse_config("test_url: https://example.com/a\n").unwrap();
            assert_eq!(config.test_url, vec![TestUrl("https://example.com/a")]);
        }

        #[test]
        fn multiple_values() {
            let config =
                parse_config("test_url: https://example.com/a\ntest_url: https://example.com/b\n")
                    .unwrap();
            assert_eq!(
                config.test_url,
                vec![
                    TestUrl("https://example.com/a"),
                    TestUrl("https://example.com/b")
                ]
            );
        }
    }

    mod single_page_link {
        use super::*;

        #[test]
        fn none() {
            let config = parse_config("").unwrap();
            assert_eq!(config.single_page_link, None);
        }

        #[test]
        fn some() {
            // Last occurrence wins
            let config = parse_config(
                "single_page_link: //a[@class='first']\nsingle_page_link: //a[@class='second']\n",
            )
            .unwrap();
            assert_eq!(config.single_page_link, Some(XPath("//a[@class='second']")));
        }
    }

    mod single_page_link_in_feed {
        use super::*;

        #[test]
        fn none() {
            let config = parse_config("").unwrap();
            assert_eq!(config.single_page_link_in_feed, None);
        }

        #[test]
        fn some() {
            let config = parse_config("single_page_link_in_feed: //a[@class='page']\n").unwrap();
            assert_eq!(
                config.single_page_link_in_feed,
                Some(XPath("//a[@class='page']"))
            );
        }
    }

    mod next_page_link {
        use super::*;

        #[test]
        fn none() {
            let config = parse_config("").unwrap();
            assert_eq!(config.next_page_link, None);
        }

        #[test]
        fn some() {
            let config = parse_config("next_page_link: //a[@class='next']\n").unwrap();
            assert_eq!(config.next_page_link, Some(XPath("//a[@class='next']")));
        }
    }

    mod prune {
        use super::*;

        #[test]
        fn yes_value() {
            let config = parse_config("prune: yes\n").unwrap();
            assert_eq!(config.prune, YesNo::Yes);
        }

        #[test]
        fn no_value() {
            let config = parse_config("prune: no\n").unwrap();
            assert_eq!(config.prune, YesNo::No);
        }

        #[test]
        fn default_value() {
            let config = parse_config("").unwrap();
            assert_eq!(config.prune, YesNo::Yes);
        }
    }

    mod tidy {
        use super::*;

        #[test]
        fn yes_value() {
            let config = parse_config("tidy: yes\n").unwrap();
            assert_eq!(config.tidy, YesNo::Yes);
        }

        #[test]
        fn no_value() {
            let config = parse_config("tidy: no\n").unwrap();
            assert_eq!(config.tidy, YesNo::No);
        }

        #[test]
        fn default_value() {
            let config = parse_config("").unwrap();
            assert_eq!(config.tidy, YesNo::No);
        }
    }

    mod autodetect_on_failure {
        use super::*;

        #[test]
        fn yes_value() {
            let config = parse_config("autodetect_on_failure: yes\n").unwrap();
            assert_eq!(config.autodetect_on_failure, YesNo::Yes);
        }

        #[test]
        fn no_value() {
            let config = parse_config("autodetect_on_failure: no\n").unwrap();
            assert_eq!(config.autodetect_on_failure, YesNo::No);
        }

        #[test]
        fn default_value() {
            let config = parse_config("").unwrap();
            assert_eq!(config.autodetect_on_failure, YesNo::Yes);
        }
    }
}
