use ftr_config_parser::{TestUrl, XPath, parse_config};

fn main() {
    let input_config = "
body: //article
author: //aside/a[contains(@href, '/authors/')]/span
strip: //article/div[1]
strip: //article/ul[1]
strip: //aside
test_url: https://blog.sentry.io/gdpr-sentry-and-you/
";

    let config = parse_config(input_config).unwrap();

    let body: Vec<&str> = config.body().iter().map(XPath::as_str).collect();
    assert_eq!(body, ["//article"]);

    let author: Vec<&str> = config.author().iter().map(XPath::as_str).collect();
    assert_eq!(author, ["//aside/a[contains(@href, '/authors/')]/span"]);

    let strip: Vec<&str> = config.strip().iter().map(XPath::as_str).collect();
    assert_eq!(strip, ["//article/div[1]", "//article/ul[1]", "//aside"]);

    let urls: Vec<&str> = config.test_url().iter().map(TestUrl::as_str).collect();
    assert_eq!(urls, ["https://blog.sentry.io/gdpr-sentry-and-you/"]);
}
