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
    YES,
    NO,
}

impl From<bool> for YesNo {
    fn from(value: bool) -> Self {
        match value {
            true => Self::YES,
            false => Self::NO,
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

pub fn parse_config<'a>(input: &'a str) -> Result<Config<'a>, error::Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
