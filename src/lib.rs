use std::str::FromStr;

mod error;

pub use error::{Error, ErrorKind};

#[derive(Debug, Eq, PartialEq)]
pub struct Config<'a> {
    pub title: Vec<XPath<'a>>,
    pub body: Vec<XPath<'a>>,
    pub date: Vec<XPath<'a>>,
    pub author: Vec<XPath<'a>>,
    pub strip: Vec<XPath<'a>>,
    pub strip_id_or_class: Vec<IdOrClass<'a>>,
    pub strip_image_src: Vec<ImageSrcFragment<'a>>,
    pub prune: bool,
    pub tidy: bool,
    pub autodetect_on_failure: YesNo,
    pub single_page_link: Option<XPath<'a>>,
    pub single_page_link_in_feed: Option<XPath<'a>>,
    pub next_page_link: Option<XPath<'a>>,
    pub replace_string: Vec<ReplaceString<'a>>,
    pub http_header: Vec<HttpHeader<'a>>,
    pub test_url: Vec<TestUrl<'a>>,
}

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
pub struct ImageSrcFragment<'a>(pub &'a str);

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
pub struct ReplaceString<'a> {
    pub find: &'a str,
    pub replace: &'a str,
}

#[derive(Debug, Eq, PartialEq)]
pub struct HttpHeader<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TestUrl<'a>(pub &'a str);

fn parse_line(line: &str) -> Result<(&str, Option<&str>, &str), error::ErrorKind> {
    let colon = line
        .find(':')
        .ok_or_else(|| error::ErrorKind::MalformedLine(line.to_string()))?;
    let key_part = line[..colon].trim();
    let value = line[colon + 1..].trim();

    if let Some(open) = key_part.find('(') {
        let close = key_part.find(')').unwrap_or(key_part.len());
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
    let mut prune = true;
    let mut tidy = false;
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
                strip_id_or_class.push(IdOrClass::try_from(value).map_err(&locate_err)?);
            }
            "strip_image_src" => strip_image_src.push(ImageSrcFragment(value)),
            "prune" => prune = value.parse::<YesNo>().map_err(&locate_err)?.into(),
            "tidy" => tidy = value.parse::<YesNo>().map_err(&locate_err)?.into(),
            "autodetect_on_failure" => {
                autodetect_on_failure = value.parse().map_err(&locate_err)?
            }
            "single_page_link" => {
                single_page_link = Some(XPath::try_from(value).map_err(&locate_err)?)
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

    #[test]
    fn yes_no_parses_yes() {
        assert!(matches!("yes".parse::<YesNo>().unwrap(), YesNo::Yes));
    }

    #[test]
    fn yes_no_parses_no() {
        assert!(matches!("no".parse::<YesNo>().unwrap(), YesNo::No));
    }

    #[test]
    fn yes_no_rejects_unknown() {
        assert!(matches!(
            "maybe".parse::<YesNo>(),
            Err(ErrorKind::InvalidBoolValue(_))
        ));
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
    fn parse_line_simple_key() {
        assert_eq!(
            parse_line("body: //div[@id='content']").unwrap(),
            ("body", None, "//div[@id='content']")
        );
    }

    #[test]
    fn parse_line_parametric_key() {
        assert_eq!(
            parse_line("http_header(Cookie): euConsent=true").unwrap(),
            ("http_header", Some("Cookie"), "euConsent=true")
        );
    }

    #[test]
    fn parse_line_value_with_colon() {
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
    fn parse_line_no_colon_returns_error() {
        assert!(matches!(
            parse_line("badline"),
            Err(ErrorKind::MalformedLine(_))
        ));
    }

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
                prune: true,
                tidy: false,
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
        let config =
            parse_config("#title: div[@id='title'] but this is a comment\n\n# another comment\n")
                .unwrap();
        assert!(config.title.is_empty());
    }

    #[test]
    fn wikipedia_example() {
        let input = "body: //div[@id='content']\nstrip_id_or_class: editsection\nstrip_id_or_class: toc\nprune: no\n";
        let config = parse_config(input).unwrap();
        assert_eq!(config.body, vec![XPath("//div[@id='content']")]);
        assert_eq!(
            config.strip_id_or_class,
            vec![IdOrClass("editsection"), IdOrClass("toc")]
        );
        assert!(!config.prune);
    }

    #[test]
    fn multiple_vec_fields_accumulated() {
        let input = "title: //h1\ntitle: //h2\nbody: //article\nbody: //main\n";
        let config = parse_config(input).unwrap();
        assert_eq!(config.title, vec![XPath("//h1"), XPath("//h2")]);
        assert_eq!(config.body, vec![XPath("//article"), XPath("//main")]);
    }

    #[test]
    fn http_header_parsed() {
        let input = "http_header(Cookie): euConsent=true\nhttp_header(User-agent): Mozilla/5.0\n";
        let config = parse_config(input).unwrap();
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

    #[test]
    fn replace_string_parsed() {
        let input = "replace_string(foo): bar\n";
        let config = parse_config(input).unwrap();
        assert_eq!(
            config.replace_string,
            vec![ReplaceString {
                find: "foo",
                replace: "bar"
            }]
        );
    }

    #[test]
    fn single_page_link_last_wins() {
        let input =
            "single_page_link: //a[@class='first']\nsingle_page_link: //a[@class='second']\n";
        let config = parse_config(input).unwrap();
        assert_eq!(config.single_page_link, Some(XPath("//a[@class='second']")));
    }

    #[test]
    fn test_url_accumulated() {
        let input = "test_url: https://example.com/a\ntest_url: https://example.com/b\n";
        let config = parse_config(input).unwrap();
        assert_eq!(
            config.test_url,
            vec![
                TestUrl("https://example.com/a"),
                TestUrl("https://example.com/b")
            ]
        );
    }

    #[test]
    fn unknown_keys_silently_skipped() {
        let config = parse_config("unknown_directive: some value\n").unwrap();
        assert!(config.title.is_empty());
    }

    #[test]
    fn invalid_yes_no_returns_error() {
        let err = parse_config("prune: maybe\n").unwrap_err();
        assert!(matches!(
            err,
            Error {
                kind: ErrorKind::InvalidBoolValue(_),
                line: 1
            }
        ));
    }

    #[test]
    fn id_or_class_valid() {
        assert!(IdOrClass::try_from("sidebar").is_ok());
        assert!(IdOrClass::try_from("main-content").is_ok());
        assert!(IdOrClass::try_from("_private").is_ok());
        assert!(IdOrClass::try_from("item123").is_ok());
    }

    #[test]
    fn id_or_class_rejects_empty() {
        assert!(matches!(
            IdOrClass::try_from(""),
            Err(ErrorKind::InvalidIdOrClass(_))
        ));
    }

    #[test]
    fn id_or_class_rejects_whitespace() {
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

    #[test]
    fn parse_config_rejects_invalid_id_or_class() {
        let err = parse_config("strip_id_or_class: foo bar\n").unwrap_err();
        assert!(matches!(err.kind, ErrorKind::InvalidIdOrClass(_)));
        assert_eq!(err.line, 1);
    }

    #[test]
    fn error_reports_correct_line_number() {
        let input = "body: //article\nprune: oops\n";
        let err = parse_config(input).unwrap_err();
        assert!(matches!(err.kind, ErrorKind::InvalidBoolValue(_)));
        assert_eq!(err.line, 2);
    }

    #[test]
    fn error_line_number_skips_blanks_and_comments() {
        let input = "# comment\n\nbody: //article\nprune: oops\n";
        let err = parse_config(input).unwrap_err();
        assert!(matches!(err.kind, ErrorKind::InvalidBoolValue(_)));
        // line 4 in the raw input (comment=1, blank=2, body=3, prune=4)
        assert_eq!(err.line, 4);
    }
}
