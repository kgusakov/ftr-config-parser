mod error;
mod types;

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

pub struct XPath<'a>(pub &'a str);
pub struct IdOrClass<'a>(pub &'a str);
pub struct ImageSrcFragment<'a>(pub &'a str);

pub enum YesNo {
    Yes,
    No,
}

impl From<bool> for YesNo {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Yes,
            false => Self::No,
        }
    }
}

pub struct ReplaceString<'a> {
    pub find: &'a str,
    pub replace: &'a str,
}

pub struct HttpHeader<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

pub struct TestUrl<'a>(pub &'a str);

fn parse_yes_no(value: &str) -> Result<bool, error::Error> {
    match value {
        "yes" => Ok(true),
        "no" => Ok(false),
        other => Err(error::Error::InvalidBoolValue(other.to_string())),
    }
}

fn parse_line(line: &str) -> Result<(&str, Option<&str>, &str), error::Error> {
    let colon = line
        .find(':')
        .ok_or_else(|| error::Error::MalformedLine(line.to_string()))?;
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

pub fn parse_config<'a>(input: &'a str) -> Result<Config<'a>, error::Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn malformed_line_no_colon() {
        let result = parse_config("no colon here");
        assert!(matches!(result, Err(error::Error::MalformedLine(_))));
    }

    #[test]
    fn yes_no_parses_yes() {
        assert_eq!(parse_yes_no("yes").unwrap(), true);
    }

    #[test]
    fn yes_no_parses_no() {
        assert_eq!(parse_yes_no("no").unwrap(), false);
    }

    #[test]
    fn yes_no_rejects_unknown() {
        assert!(matches!(
            parse_yes_no("maybe"),
            Err(error::Error::InvalidBoolValue(_))
        ));
    }

    #[test]
    fn parse_line_simple_key() {
        let (name, param, value) = parse_line("body: //div[@id='content']").unwrap();
        assert_eq!(name, "body");
        assert_eq!(param, None);
        assert_eq!(value, "//div[@id='content']");
    }

    #[test]
    fn parse_line_parametric_key() {
        let (name, param, value) = parse_line("http_header(Cookie): euConsent=true").unwrap();
        assert_eq!(name, "http_header");
        assert_eq!(param, Some("Cookie"));
        assert_eq!(value, "euConsent=true");
    }

    #[test]
    fn parse_line_value_with_colon() {
        let (name, param, value) = parse_line("http_header(User-agent): Mozilla/5.0 (iPad; CPU OS 12_0_1)").unwrap();
        assert_eq!(name, "http_header");
        assert_eq!(param, Some("User-agent"));
        assert_eq!(value, "Mozilla/5.0 (iPad; CPU OS 12_0_1)");
    }

    #[test]
    fn parse_line_no_colon_returns_error() {
        assert!(matches!(
            parse_line("badline"),
            Err(error::Error::MalformedLine(_))
        ));
    }
}
